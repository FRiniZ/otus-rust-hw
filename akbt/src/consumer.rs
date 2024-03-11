use crate::errors::AppError;
use rdkafka::{
    self,
    consumer::{BaseConsumer, Consumer},
    message::OwnedMessage,
    util::Timeout,
    ClientConfig, Message, TopicPartitionList,
};
use std::{ops::Index, time::Duration};

struct BackupContext;
impl rdkafka::client::ClientContext for BackupContext {}
impl rdkafka::consumer::ConsumerContext for BackupContext {}

pub struct MyConsumer {
    inner: rdkafka::consumer::BaseConsumer<BackupContext>,
    topic_name: String,
    partitions: i32,
    partitions_paused: i32,
    offsets_end: Vec<i64>,
}

impl MyConsumer {
    pub fn new(brokers: String, topic_name: &str) -> Result<Self, AppError> {
        let context = BackupContext;
        let consumer: BaseConsumer<BackupContext> = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
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
            return Err(AppError::TopicNotFound(topic_name.to_string()));
        }

        Ok(MyConsumer {
            inner: consumer,
            topic_name: topic_name.to_string(),
            partitions: topic.partitions().len() as i32,
            partitions_paused: 0,
            offsets_end: vec![],
        })
    }

    pub fn assign(
        &mut self,
        _offset_begin: Option<i64>,
        _offset_end: Option<i64>,
    ) -> Result<(), AppError> {
        let mut tppa = rdkafka::TopicPartitionList::new();
        for part_idx in 0..self.partitions() {
            let (_offset_begin, offset_end) = self.offsets(part_idx)?;
            tppa.add_partition(self.topic_name(), part_idx as i32);
            tppa.set_all_offsets(rdkafka::Offset::Beginning).unwrap();
            self.offsets_end.push(offset_end);
        }
        self.inner.assign(&tppa)?;
        Ok(())
    }

    // Returns (OffsetBegin, OffsetEnd)
    pub fn offsets(&self, part_id: i32) -> Result<(i64, i64), AppError> {
        let (offset_begin, offset_end) = self.inner.fetch_watermarks(
            self.topic_name(),
            part_id,
            Timeout::After(Duration::from_secs(1)),
        )?;

        Ok((offset_begin, offset_end))
    }

    pub fn get_offset_end(&self, part_id: i32) -> i64 {
        *self.offsets_end.index(part_id as usize)
    }

    pub fn poll(&mut self, timeout: Option<Duration>) -> Option<Result<OwnedMessage, AppError>> {
        if self.all_partitions_paused() {
            return Some(Err(AppError::EOF));
        }

        let rd_msg = self.inner.poll(timeout);
        let msg = match rd_msg {
            Some(msg) => msg.unwrap().detach(),
            None => return None,
        };

        let end_offset = self.get_offset_end(msg.partition());
        let offset = msg.offset();
        let part = msg.partition();

        if offset + 1 == end_offset {
            let mut tppa = TopicPartitionList::with_capacity(1);
            tppa.add_partition(self.topic_name(), part);
            self.inner.pause(&tppa).unwrap();
            self.partitions_paused += 1;
        }

        Some(Ok(msg))
    }

    pub fn all_partitions_paused(&self) -> bool {
        self.partitions_paused == self.partitions
    }

    pub fn partitions(&self) -> i32 {
        self.partitions
    }

    pub fn topic_name(&self) -> &str {
        self.topic_name.as_str()
    }
}
