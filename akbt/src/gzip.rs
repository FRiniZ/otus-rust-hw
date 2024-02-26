use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use log::{error, info};
use std::io::{self, BufReader, BufWriter, Read};
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::{fs::File, io::Write};

use crate::counters::ByteCounter;
use crate::errors::AppError;
use crate::mbprocess::MProgressBars;
use crate::protos::kafka_messages::{kafka_message_unpack, KafkaMessage};

#[derive(Debug)]
pub struct GzMsg {
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct GzWriter {}

impl GzWriter {
    pub fn run(file: String, level: u32) -> Result<(SyncSender<GzMsg>, JoinHandle<()>), AppError> {
        let pathfile = match Path::new(&file).extension() {
            Some(_) => file,
            None => file + ".gz",
        };

        if Path::new(pathfile.as_str()).exists() {
            return Err(AppError::FileExists(pathfile));
        }

        let (sender, receiver): (SyncSender<GzMsg>, Receiver<GzMsg>) = sync_channel(1);

        let handle: JoinHandle<()> = thread::spawn(move || {
            let bytes_written = AtomicUsize::new(0);
            let writer = BufWriter::new(ByteCounter::new(
                File::create(&pathfile).expect("Can't create file"),
                &bytes_written,
            ));

            let mut encoder = GzEncoder::new(writer, Compression::new(level));
            for gzmsg in receiver {
                encoder.write_all(&gzmsg.data).unwrap();
            }
            encoder.finish().unwrap();
        });

        Ok((sender, handle))
    }
}

#[derive(Debug)]
pub struct GzReader {}

impl GzReader {
    pub fn run(
        pathfile: String,
        mb: Arc<Mutex<MProgressBars>>,
    ) -> Result<(Receiver<Vec<KafkaMessage>>, u64, JoinHandle<()>), AppError> {
        if !Path::new(&pathfile).exists() {
            return Err(AppError::FileNotExists(pathfile));
        }

        let metadata = std::fs::metadata(&pathfile)?;

        let file_size = metadata.len();
        info!("File size: {}", file_size);

        let (sender, receiver): (SyncSender<Vec<KafkaMessage>>, Receiver<Vec<KafkaMessage>>) =
            sync_channel(1);

        let handle: JoinHandle<()> = thread::spawn(move || {
            let bytes_read = AtomicUsize::new(0);
            let reader = BufReader::new(ByteCounter::new(
                File::open(&pathfile).expect("Can't open file"),
                &bytes_read,
            ));

            let mut last_batch = false;
            let mut decoder = GzDecoder::new(reader);
            loop {
                let mut batch = Vec::with_capacity(1000);
                for _ in 0..1000 {
                    let mut buf_size: [u8; 8] = [0; 8];
                    let msg_size: usize;
                    let res = decoder.read_exact(&mut buf_size);

                    match res {
                        Ok(_) => (),
                        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                            info!("EOF");
                            last_batch = true;
                            break;
                        }
                        Err(e) => {
                            panic!("Closed by {:?}", e);
                        }
                    }

                    msg_size = usize::from_be_bytes(buf_size);
                    let mut msg_body = vec![0; msg_size];
                    let res = decoder.read_exact(&mut msg_body);
                    if let Err(e) = res {
                        error!("Closed by {:?}", e);
                        break;
                    }
                    let kmsg = kafka_message_unpack(&msg_body).unwrap();
                    batch.push(kmsg);

                    mb.lock().unwrap().update(
                        0,
                        bytes_read.load(std::sync::atomic::Ordering::Relaxed) as i64,
                    );
                }
                sender.send(batch).unwrap();
                if last_batch {
                    break;
                }
            }
        });

        Ok((receiver, file_size, handle))
    }
}
