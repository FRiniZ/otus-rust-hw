use flate2::write::GzEncoder;
use flate2::Compression;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use std::io::BufWriter;
use std::path::Path;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::{fs::File, io::Write};

use crate::errors::AppError;
use crate::protos::kafka_messages::{kafka_message_len, kafka_message_pack, KafkaMessage};

struct ByteCounter<W> {
    inner: W,
    count: Arc<Mutex<u64>>,
}

impl<W> ByteCounter<W>
where
    W: Write,
{
    fn new(inner: W, count: Arc<Mutex<u64>>) -> Self {
        ByteCounter { inner, count }
    }
}

impl<W> Write for ByteCounter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let res = self.inner.write(buf);
        if let Ok(size) = res {
            let mut count = self.count.lock().unwrap();
            *count += size as u64;
        }
        res
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[derive(Debug)]
pub struct GzWriter {}

impl GzWriter {
    pub fn run(
        pathfile: String,
        pb: ProgressBar,
    ) -> Result<(SyncSender<Vec<KafkaMessage>>, JoinHandle<usize>), AppError> {
        if Path::new(pathfile.as_str()).exists() {
            return Err(AppError::FileExists(pathfile));
        }

        let bytes_written = Arc::new(Mutex::new(0));
        let writer = BufWriter::new(ByteCounter::new(
            File::create(&pathfile)?,
            bytes_written.clone(),
        ));

        let mut encoder = GzEncoder::new(writer, Compression::best());

        pb.set_style(ProgressStyle::with_template("{spinner:.green} archive size: {msg}").unwrap());
        let (sender, receiver) = sync_channel(10);

        let handle: JoinHandle<usize> = thread::spawn(move || {
            let mut msg_count = 0;
            for kbatch in receiver {
                for kmsg in kbatch {
                    msg_count += 1;
                    let packed_size = kafka_message_len(&kmsg);
                    let packed_data = kafka_message_pack(&kmsg);
                    encoder.write_all(&packed_size.to_be_bytes()).unwrap();
                    encoder.write_all(&packed_data).unwrap();
                    let bytes = bytes_written.lock().unwrap();
                    pb.set_message(format!(
                        "{} messages processed: {}",
                        HumanBytes(*bytes),
                        msg_count
                    ));
                    pb.tick();
                }
            }
            encoder.finish().unwrap();
            pb.finish();
            msg_count
        });

        Ok((sender, handle))
    }
}
