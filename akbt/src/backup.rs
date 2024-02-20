use rdkafka::{
    consumer::{BaseConsumer, Consumer, ConsumerContext},
    util::Timeout,
    ClientConfig, ClientContext, Message, TopicPartitionList,
};

use std::{sync::mpsc::SyncSender, thread, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::{
    errors::AppError,
    gzwriter::GzWriter,
    protos::kafka_messages::{kafka_message_new, KafkaMessage},
};

const PB_PROCESS: &str = "{spinner:.green} {msg:>4} {elapsed_precise} {bar:.green/red} ETA:{eta}";
const PB_FINISH: &str = "{spinner:.green} {msg:>4} {elapsed_precise} {bar:.green/red} done";

struct Backup {
    _id: i32,
    consumer: BaseConsumer<BackupContext>,
    _offset_begin: i64,
    offset_end: i64,
    progress_bar: ProgressBar,
    sender: SyncSender<Vec<KafkaMessage>>,
}

impl Backup {
    pub fn new(
        id: i32,
        brokers: String,
        topic_name: String,
        offset_begin: i64,
        offset_end: i64,
        progress_bar: ProgressBar,
        sender: SyncSender<Vec<KafkaMessage>>,
    ) -> Result<Self, AppError> {
        let context = BackupContext;
        let consumer: BaseConsumer<BackupContext> = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("group.id", "akbt")
            .create_with_context(context)
            .expect("Consumer creation failed");

        let mut partitions = TopicPartitionList::with_capacity(1);
        partitions.add_partition(&topic_name, id);
        partitions
            .set_all_offsets(rdkafka::Offset::Beginning)
            .unwrap();

        consumer
            .assign(&partitions)
            .expect("Couldnt assign consumer to topic");

        Ok(Self {
            _id: id,
            consumer,
            _offset_begin: offset_begin,
            offset_end,
            progress_bar,
            sender,
        })
    }

    pub fn start(self) {
        let mut last_offset = 0;
        let pb = self.progress_bar;

        let sender = self.sender.clone();
        let mut done = false;
        loop {
            let mut kbatch = Vec::with_capacity(100);
            for _ in 0..100 {
                let rd_msg = self.consumer.poll(None);

                let rd_msg = match rd_msg {
                    Some(msg) => msg,
                    None => {
                        continue;
                    }
                };

                let msg = rd_msg.unwrap();

                if msg.offset() + 1 == self.offset_end {
                    done = true;
                    break;
                }

                pb.inc((msg.offset() - last_offset) as u64);
                pb.tick();
                last_offset = msg.offset();

                let _kmsg = kafka_message_new(
                    msg.key().map(|slice| Vec::from(slice)),
                    msg.payload().map(|slice| Vec::from(slice)),
                    Some(msg.partition() as u32),
                    vec![],
                );
                kbatch.push(_kmsg);
            }
            sender.send(kbatch).unwrap();
            if done {
                break;
            }
        }
        pb.set_style(
            ProgressStyle::with_template(PB_FINISH)
                .unwrap()
                .progress_chars("=>-"),
        );
        pb.finish();
    }
}

pub fn backup(
    n_wrk: usize,
    _brokers: String,
    topic_name: String,
    _file: String,
) -> Result<(), AppError> {
    let mb = MultiProgress::new();
    let header_pb =
        mb.add(ProgressBar::new(0).with_style(ProgressStyle::with_template("{msg}").unwrap()));
    header_pb.set_message(format!("Archive: {}", _file));
    let pb = mb.add(ProgressBar::new(0));

    let (sender, gzhandler) = GzWriter::run(_file, pb)?;

    // Connect to topic. Read medatada
    let context = BackupContext;
    let consumer: BaseConsumer<BackupContext> = ClientConfig::new()
        .set("bootstrap.servers", &_brokers)
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

    //mb.println("Processing partitions").unwrap();

    let mut vbackup = Vec::new();
    for part in topic.partitions() {
        let (offset_begin, offset_end) = consumer
            .fetch_watermarks(
                &topic_name,
                part.id(),
                Timeout::After(Duration::from_secs(1)),
            )
            .unwrap();

        let pb = mb.insert_before(
            &header_pb,
            ProgressBar::new((offset_end - offset_begin - 1) as u64),
        );

        pb.set_style(
            ProgressStyle::with_template(PB_PROCESS)
                .unwrap()
                .progress_chars("=>-"),
        );
        pb.set_message(format!("{}|", part.id()));

        let b = Backup::new(
            part.id(),
            _brokers.clone(),
            topic_name.clone(),
            offset_begin,
            offset_end,
            pb,
            sender.clone(),
        )?;
        vbackup.push(b);
    }
    drop(consumer);

    let header_pb2 = mb.insert(
        0,
        ProgressBar::new(0).with_style(ProgressStyle::with_template("{msg}").unwrap()),
    );
    header_pb2.set_message("Processing partitions");

    while vbackup.len() > 0 {
        let mut vhandle = Vec::new();
        for _ in 0..n_wrk {
            let b = vbackup.pop();
            if b.is_some() {
                let b = b.unwrap();
                let h = thread::spawn(move || b.start());
                vhandle.push(h);
            } else {
                break;
            }
        }
        for h in vhandle {
            h.join().unwrap();
        }
    }

    drop(sender);
    gzhandler.join().unwrap();
    header_pb.finish();
    header_pb2.finish();
    drop(mb);
    Ok(())
}

struct BackupContext;

impl ClientContext for BackupContext {}

impl ConsumerContext for BackupContext {}
