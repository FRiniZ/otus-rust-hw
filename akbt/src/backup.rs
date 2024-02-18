use flate2::write::GzEncoder;
use flate2::Compression;
use rdkafka::{
    consumer::{BaseConsumer, Consumer, ConsumerContext},
    util::Timeout,
    ClientConfig, ClientContext, Message, TopicPartitionList,
};

use std::io::BufWriter;
use std::thread;
use std::{fs::File, io::Write};

use serde_json;
use std::{path::Path, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::info;

use crate::{
    errors::AppError,
    protos::kafka_messages::{kafka_message_len, kafka_message_new, kafka_message_pack},
};

fn all_partitions_finished(parts: &Vec<bool>) -> bool {
    let result = parts.iter().try_for_each(|b| -> Result<(), ()> {
        if *b == false {
            return Err(());
        }
        Ok(())
    });

    if result.is_err() {
        return false;
    }
    true
}

pub fn backup(_brokers: String, topic_name: String, file: String) -> Result<(), AppError> {
    // Check if file exists then exit with error.

    // if Path::new(file.as_str()).exists() {
    //     panic!("File:{} is exists", file);
    // }

    let writer = BufWriter::new(File::create(&file).expect("Can't create file"));
    let mut encoder = GzEncoder::new(writer, Compression::default());
    let mb = MultiProgress::new();
    mb.println("Partiton, ETA");
    let sty =
        //ProgressStyle::with_template("[{msg:<12} {eta}] {wide_bar:.cyan/blue} {pos:>7}/{len:7}")
        ProgressStyle::with_template("{spinner:.green} {msg:<12} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({eta})")
            .unwrap()
            .progress_chars("█▛▌▖  ");

    // Connect to topic. Read medatada
    let context = BackupContext;
    let consumer: BaseConsumer<BackupContext> = ClientConfig::new()
        .set("bootstrap.servers", &_brokers)
        .set("group.id", "group2")
        .create_with_context(context)
        .expect("Consumer creation failed");

    let metadata =
        consumer.fetch_metadata(Some(&topic_name), Timeout::After(Duration::from_secs(60)))?;
    let topics = metadata.topics();
    let topic = &topics[0];

    if topic.partitions().len() == 0 {
        return Err(AppError::TopicNotFound(topic_name));
    }

    let mut vec_pb = Vec::with_capacity(topic.partitions().len());
    let mut vec_last_pb = Vec::with_capacity(topic.partitions().len());
    let mut vec_part_offset_stop = Vec::with_capacity(topic.partitions().len());
    let mut vec_part_finished = Vec::with_capacity(topic.partitions().len());

    for part in topic.partitions() {
        let (offset_begin, offset_end) = consumer.fetch_watermarks(
            topic.name(),
            part.id(),
            Timeout::After(Duration::from_secs(1)),
        )?;

        let pb = mb.add(ProgressBar::new((offset_end - offset_begin - 1) as u64));

        pb.set_style(sty.clone());
        pb.set_message(format!("Partition:{}", part.id()));

        vec_pb.push(pb);
        vec_last_pb.push(offset_begin);
        vec_part_finished.push(false);
        vec_part_offset_stop.push(offset_end - 1);
    }

    let partitions = metadata.topics()[0].partitions();
    let mut topic_partition_list = TopicPartitionList::with_capacity(partitions.len());
    for partition in partitions {
        topic_partition_list.add_partition(&topic_name, partition.id());
    }
    topic_partition_list.set_all_offsets(rdkafka::Offset::Beginning)?;

    consumer
        .assign(&topic_partition_list)
        .expect("Couldnt assign consumer to topic");

    loop {
        let rd_msg = consumer.poll(None);

        thread::sleep(Duration::from_millis(10));
        let rd_msg = match rd_msg {
            Some(msg) => msg,
            None => {
                continue;
            }
        };

        let msg = rd_msg?;
        // let msg = match rd_msg {
        //     Ok(msg) => msg,
        //     Err(e) => match e {
        //         rdkafka::error::KafkaError::PartitionEOF(_p) => {
        //             // info!("PartitionEOF({})", p);
        //             continue;
        //         }
        //         _ => panic!("Unknown error"),
        //     },
        // };

        let part: usize = msg.partition() as usize;
        let pb = &mut vec_pb[msg.partition() as usize];
        let last_offset = &vec_last_pb[msg.partition() as usize];

        pb.inc((msg.offset() - last_offset) as u64);
        pb.tick();
        vec_last_pb[msg.partition() as usize] = msg.offset();

        let _kmsg = kafka_message_new(
            msg.key().map(|slice| Vec::from(slice)),
            msg.payload().map(|slice| Vec::from(slice)),
            Some(msg.partition() as u32),
            vec![],
        );

        // let packed_size = kafka_message_len(&kmsg);
        // let packed_data = kafka_message_pack(&kmsg);
        // encoder.write_all(&packed_size.to_be_bytes())?;
        // encoder.write_all(&packed_data)?;
        if msg.offset() == vec_part_offset_stop[part] {
            let mut tp_list = TopicPartitionList::new();
            tp_list.add_partition(msg.topic(), msg.partition());
            consumer.pause(&tp_list)?;
            pb.finish();
            vec_part_finished[part] = true;
            // info!("Partiotion:{} has paused", part);
            if all_partitions_finished(&vec_part_finished) {
                // info!("All partitions finished");
                break;
            }
        }
    }

    // encoder.finish()?;

    Ok(())
}

struct BackupContext;

impl ClientContext for BackupContext {}

impl ConsumerContext for BackupContext {}
