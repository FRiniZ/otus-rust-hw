use log::{error, info};
use rdkafka::{
    consumer::{BaseConsumer, Consumer, ConsumerContext},
    util::Timeout,
    ClientConfig, ClientContext, Message, TopicPartitionList,
};

use std::{
    ops::Index,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    thread, time,
    time::Duration,
};

use crate::{
    counters::Counter,
    errors::AppError,
    gzip::{GzMsg, GzWriter},
    mbprocess::MProgressBars,
    protos::kafka_messages::{kafka_message_len, kafka_message_new, kafka_message_pack},
};

fn backup_worker(
    receiver: Receiver<Vec<rdkafka::message::OwnedMessage>>,
    encoder: SyncSender<GzMsg>,
    mb: Arc<Mutex<MProgressBars>>,
) -> Result<(), AppError> {
    let mut max_capacity = 1024;
    loop {
        match receiver.recv() {
            Ok(batch) => {
                let mut gzmsg = GzMsg {
                    data: Vec::with_capacity(max_capacity),
                };
                for msg in batch {
                    mb.lock().unwrap().update(msg.partition(), msg.offset());
                    let kmsg = kafka_message_new(
                        msg.key().map(|slice| Vec::from(slice)),
                        msg.payload().map(|slice| Vec::from(slice)),
                        Some(msg.partition() as u32),
                        vec![],
                    );
                    gzmsg
                        .data
                        .append(&mut kafka_message_len(&kmsg).to_be_bytes().to_vec());
                    gzmsg.data.append(&mut kafka_message_pack(&kmsg));
                }
                if max_capacity < gzmsg.data.len() {
                    max_capacity = gzmsg.data.len();
                }
                encoder.send(gzmsg)?;
            }
            Err(_) => break,
        }
    }
    Ok(())
}

fn consumer_task(
    topic_name: String,
    consumer: BaseConsumer<BackupContext>,
    messages: Arc<std::sync::Mutex<Counter>>,
    sender: SyncSender<Vec<rdkafka::message::OwnedMessage>>,
    mb: Arc<Mutex<MProgressBars>>,
    offsets_end: Vec<i64>,
    partitions_count: Arc<Mutex<usize>>,
) -> Result<(), AppError> {
    let mut last_batch = false;

    loop {
        let mut batch = Vec::with_capacity(1000);

        for _ in 0..1000 {
            let rd_msg = consumer.poll(None);
            let msg = match rd_msg {
                Some(msg) => msg.unwrap().detach(),
                None => {
                    continue;
                }
            };

            let end_offset = *offsets_end.index(msg.partition() as usize);
            let offset = msg.offset();
            let part = msg.partition();
            batch.push(msg);

            if offset + 1 == end_offset {
                let mut tppa = TopicPartitionList::with_capacity(1);
                tppa.add_partition(&topic_name, part);
                consumer.pause(&tppa).unwrap();
                mb.lock().unwrap().finish_partition(part);
                info!("Partition:{} has paused", part,);
                *partitions_count.lock().unwrap() -= 1;
                if *partitions_count.lock().unwrap() == 0 {
                    last_batch = true;
                    break;
                }
            }
        }

        let batch_size = batch.len();
        sender.send(batch).unwrap();

        let mut count = messages.lock().unwrap();
        count.messages += batch_size as u64;
        if count.messages % 1000000 == 0 {
            info!("Recevied: 1M Total:{}", count.messages);
        }

        if last_batch {
            info!("Recevied Total:{}", count.messages);
            break;
        }
    }

    Ok(())
}

pub fn backup(
    _brokers: String,
    topic_name: String,
    _file: String,
    level: u32,
    log_enabled: bool,
) -> Result<(), AppError> {
    let messages_counter = Arc::new(std::sync::Mutex::new(Counter { messages: 0 }));
    let (sender2encoder, encoder_handler) = GzWriter::run(_file + ".gz", level)?;

    // Connect to topic. Read medatada
    let context = BackupContext;
    let consumer: BaseConsumer<BackupContext> = ClientConfig::new()
        .set("bootstrap.servers", &_brokers)
        .set("enable.auto.offset.store", "false")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "beginning")
        .set("group.id", "akbt")
        .create_with_context(context)
        .expect("Consumer creation failed");

    let metadata =
        consumer.fetch_metadata(Some(&topic_name), Timeout::After(Duration::from_secs(60)))?;
    let topics = metadata.topics();
    let topic = &topics[0];

    if topic.partitions().len() == 0 {
        return Err(AppError::TopicNotFound(topic_name));
    }

    let parts_count = Arc::new(Mutex::new(topic.partitions().len()));

    let mb = MProgressBars::backup(
        topic_name.clone(),
        *parts_count.lock().unwrap(),
        log_enabled,
    );

    let mut tppa = TopicPartitionList::with_capacity(*parts_count.lock().unwrap());

    let mut vec_offset_end = Vec::new();

    for part_idx in 0..*parts_count.lock().unwrap() {
        let (offset_begin, offset_end) = consumer
            .fetch_watermarks(
                &topic_name,
                part_idx as i32,
                Timeout::After(Duration::from_secs(1)),
            )
            .unwrap();

        tppa.add_partition(&topic_name, part_idx as i32);
        tppa.set_all_offsets(rdkafka::Offset::Beginning).unwrap();

        mb.lock()
            .unwrap()
            .add_pb(part_idx as i32, offset_begin, offset_end);
        vec_offset_end.push(offset_end);
    }

    consumer
        .assign(&tppa)
        .expect("Couldnt assign consumer to topic");

    let cpu_count = num_cpus::get();
    if !log_enabled {
        let mb_clone = Arc::clone(&mb);
        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(100));
            mb_clone.lock().unwrap().tick();
        });
    }

    let mb_clone = mb.clone();
    let (sender2worker, receiver) = sync_channel(cpu_count * 4);
    let worker_handler = thread::spawn(move || backup_worker(receiver, sender2encoder, mb_clone));

    let mb_clone = Arc::clone(&mb);
    let consumer_handler = thread::spawn(move || {
        consumer_task(
            topic_name.clone(),
            consumer,
            messages_counter.clone(),
            sender2worker,
            mb_clone.clone(),
            vec_offset_end.clone(),
            parts_count.clone(),
        )
    });

    match consumer_handler.join() {
        Ok(_) => info!("Consumer closed"),
        Err(e) => error!("Consumer closed with error:{:?}", e),
    }

    match worker_handler.join() {
        Ok(_) => info!("Worker closed"),
        Err(e) => error!("Worker closed with error:{:?}", e),
    }

    match encoder_handler.join() {
        Ok(_) => info!("Encoder closed"),
        Err(e) => error!("Encoder closed with error:{:?}", e),
    }

    mb.lock().unwrap().finish();

    Ok(())
}

struct BackupContext;

impl ClientContext for BackupContext {}

impl ConsumerContext for BackupContext {}
