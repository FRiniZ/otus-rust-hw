use core::{panic, time};
use std::{
    sync::{mpsc::Receiver, Arc},
    thread,
    time::Duration,
};

use rdkafka::{
    consumer::ConsumerContext,
    error::KafkaError,
    producer::{BaseProducer, BaseRecord, Producer},
    types::RDKafkaErrorCode,
    ClientConfig, ClientContext,
};

use crate::{
    errors::AppError, gzip::GzReader, mbprocess::MProgressBars,
    protos::kafka_messages::KafkaMessage,
};

fn produce_worker(brokers: String, topic_name: String, receiver: Receiver<Vec<KafkaMessage>>) {
    let prod: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .create()
        .expect("Producer creation failed");
    for batch in receiver {
        for kmsg in batch {
            loop {
                let record = BaseRecord::to(&topic_name)
                    .key(kmsg.key())
                    .payload(kmsg.value())
                    .partition(kmsg.partition() as i32);
                match prod.send(record) {
                    Ok(_) => break,
                    Err((KafkaError::MessageProduction(RDKafkaErrorCode::QueueFull), _)) => {
                        prod.poll(Duration::from_millis(100));
                        continue;
                    }
                    Err(e) => panic!("Can't sending message:{:?}", e),
                }
            }
        }
    }
    prod.flush(None).unwrap();
}

pub fn restore(
    _brokers: String,
    _topic_name: String,
    _file: String,
    log_enabled: bool,
) -> Result<(), AppError> {
    let mb = MProgressBars::restore(_topic_name.clone(), _file.clone(), log_enabled);
    let (receiver, max, decoder_handler) = GzReader::run(_file, mb.clone())?;

    mb.lock().unwrap().add_pb(0, 0, max as i64);

    if !log_enabled {
        let mb_clone = Arc::clone(&mb);
        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(100));
            mb_clone.lock().unwrap().tick();
        });
    }

    let prod_handler = thread::spawn(move || {
        produce_worker(_brokers, _topic_name, receiver);
    });

    decoder_handler.join().unwrap();
    prod_handler.join().unwrap();
    mb.lock().unwrap().finish();
    Ok(())
}

struct RestoreContext;

impl ClientContext for RestoreContext {}

impl ConsumerContext for RestoreContext {}
