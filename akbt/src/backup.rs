use log::{error, info};
use rdkafka::Message;

use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    thread,
};

use crate::{
    consumer::MyConsumer,
    errors::AppError,
    gzip::{GzMsg, GzWriter},
    mbprocess::MProgressBars,
    protos::kafka_messages::{kafka_message_len, kafka_message_new, kafka_message_pack},
};

fn pack_process(
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
                match encoder.send(gzmsg) {
                    Ok(_) => (),
                    Err(e) => return Err(AppError::Send2Encoder(e.to_string())),
                };
            }
            Err(_) => break,
        }
    }
    Ok(())
}

fn consumer_process(
    mut consumer: MyConsumer,
    sender: SyncSender<Vec<rdkafka::message::OwnedMessage>>,
) -> Result<(), AppError> {
    let mut messages = 0;
    let mut last_batch = false;

    loop {
        let mut batch = Vec::with_capacity(1000);

        for _ in 0..1000 {
            let rd_msg = consumer.poll(None);
            match rd_msg {
                Some(msg) => match msg {
                    Ok(msg) => {
                        batch.push(msg);
                        messages += 1;
                    }
                    Err(e) if e == AppError::EOF => {
                        last_batch = true;
                        break;
                    }
                    Err(e) => panic!("Error:{}", e.to_string()),
                },
                None => {
                    continue;
                }
            };
        }

        sender.send(batch).unwrap();

        if messages % 1000000 == 0 {
            info!("Recevied: 1M Total:{}", messages);
        }

        if last_batch {
            info!("Recevied Total:{}", messages);
            break;
        }
    }

    Ok(())
}

pub fn backup(
    brokers: String,
    topic_name: String,
    file: String,
    level: u32,
    log_enabled: bool,
) -> Result<(), AppError> {
    let mut consumer = MyConsumer::new(brokers, &topic_name)?;
    let mb = MProgressBars::backup(&consumer, log_enabled)?;
    MProgressBars::ticker(mb.clone());

    consumer.assign(None, None)?;

    let (sender2encoder, encoder_handler) = GzWriter::run(file, level)?;
    let (sender2worker, receiver) = sync_channel(2);

    let mb_clone = mb.clone();
    let pack_handler = thread::spawn(move || pack_process(receiver, sender2encoder, mb_clone));

    let consumer_handler = thread::spawn(move || consumer_process(consumer, sender2worker));

    match consumer_handler.join() {
        Ok(_) => info!("Consumer closed"),
        Err(e) => error!("Consumer closed with error:{:?}", e),
    }

    match pack_handler.join() {
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
