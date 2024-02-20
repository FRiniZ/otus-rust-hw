use rdkafka::{
    consumer::{BaseConsumer, Consumer, ConsumerContext},
    util::Timeout,
    ClientConfig, ClientContext, Message, TopicPartitionList,
};

use std::{
    sync::{mpsc::SyncSender, Arc, Mutex},
    thread,
    time::Duration,
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::{
    errors::AppError,
    gzwriter::{GzMsg, GzWriter},
    protos::kafka_messages::{kafka_message_len, kafka_message_new, kafka_message_pack},
};

const PB_PROCESS: &str = "{spinner:.green} {msg:>4} {bar:.green/red} ETA:{eta}";
const PB_FINISH: &str = "{spinner:.green} {msg:>4} {bar:.green/red} done";

struct Backup {
    _id: i32,
    brokers: String,
    topic_name: String,
    _offset_begin: i64,
    offset_end: i64,
    progress_bar: ProgressBar,
    sender: SyncSender<GzMsg>,
    messages_counter: Arc<Mutex<u64>>,
}

impl Backup {
    pub fn new(
        id: i32,
        brokers: String,
        topic_name: String,
        offset_begin: i64,
        offset_end: i64,
        progress_bar: ProgressBar,
        sender: SyncSender<GzMsg>,
        messages_counter: Arc<Mutex<u64>>,
    ) -> Result<Self, AppError> {
        Ok(Self {
            _id: id,
            brokers,
            topic_name,
            _offset_begin: offset_begin,
            offset_end,
            progress_bar,
            sender,
            messages_counter,
        })
    }

    pub fn start(self) {
        let context = BackupContext;
        let consumer: BaseConsumer<BackupContext> = ClientConfig::new()
            .set("bootstrap.servers", self.brokers)
            .set("group.id", "akbt")
            .create_with_context(context)
            .expect("Consumer creation failed");

        let mut partitions = TopicPartitionList::with_capacity(1);
        partitions.add_partition(&self.topic_name, self._id);
        partitions
            .set_all_offsets(rdkafka::Offset::Beginning)
            .unwrap();

        consumer
            .assign(&partitions)
            .expect("Couldnt assign consumer to topic");

        let mut last_offset = 0;
        let mut last_offset_set = 0;
        let pb = self.progress_bar;

        let sender = self.sender.clone();
        let mut done = false;
        loop {
            let mut gzmsg = GzMsg {
                data: Vec::with_capacity(1024),
            };
            let mut messages = 0;
            for _ in 0..1000 {
                let rd_msg = consumer.poll(None);

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

                last_offset = msg.offset();

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
                messages += 1;
            }

            pb.inc((last_offset - last_offset_set) as u64);
            last_offset_set = last_offset;

            {
                let mut lock = self.messages_counter.lock().unwrap();
                *lock += messages;
            }
            sender.send(gzmsg).unwrap();
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
    mut n_wrk: usize,
    _brokers: String,
    topic_name: String,
    _file: String,
    level: u32,
) -> Result<(), AppError> {
    let mb = MultiProgress::new();
    let header_pb =
        mb.add(ProgressBar::new(0).with_style(ProgressStyle::with_template("{msg}").unwrap()));
    header_pb.set_message(format!("Archive: {}", _file));
    let pb = mb.add(ProgressBar::new(0));

    let messages_counter = Arc::new(Mutex::new(0));
    let (sender, gzhandler) = GzWriter::run(_file, pb, level, messages_counter.clone())?;

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

    if n_wrk == 0 || n_wrk > topic.partitions().len() {
        n_wrk = topic.partitions().len();
    }

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
            messages_counter.clone(),
        )?;
        vbackup.push(b);
    }
    drop(consumer);

    let header_pb2 = mb.insert(
        0,
        ProgressBar::new(0).with_style(ProgressStyle::with_template("{msg}").unwrap()),
    );
    header_pb2.set_message("Processing partitions");
    header_pb.finish();
    header_pb2.finish();

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

    Ok(())
}

struct BackupContext;

impl ClientContext for BackupContext {}

impl ConsumerContext for BackupContext {}
