use flate2::write::GzEncoder;
use flate2::Compression;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use std::io::BufWriter;
use std::path::Path;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::{fs::File, io::Write};

use crate::errors::AppError;

pub struct ByteCounter<W> {
    inner: W,
    count: Arc<Mutex<u64>>,
}

impl<W> ByteCounter<W>
where
    W: Write,
{
    pub fn new(inner: W, count: Arc<Mutex<u64>>) -> Self {
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
pub struct GzMsg {
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct GzWriter {}

impl GzWriter {
    pub fn run(
        pathfile: String,
        pb: ProgressBar,
        level: u32,
        messages: Arc<Mutex<u64>>,
    ) -> Result<(SyncSender<GzMsg>, JoinHandle<()>), AppError> {
        if Path::new(pathfile.as_str()).exists() {
            return Err(AppError::FileExists(pathfile));
        }

        let bytes_written = Arc::new(Mutex::new(0));
        let writer = BufWriter::new(ByteCounter::new(
            File::create(&pathfile)?,
            bytes_written.clone(),
        ));

        let mut encoder = GzEncoder::new(writer, Compression::new(level));

        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {elapsed_precise} archive size: {msg}")
                .unwrap(),
        );
        let (sender, receiver): (SyncSender<GzMsg>, Receiver<GzMsg>) = sync_channel(0);

        let handle: JoinHandle<()> = thread::spawn(move || {
            let mut bytes: u64;
            let mut msg_count: u64;
            for gzmsg in receiver {
                //encoder.write_all(&gzmsg.data).unwrap();
                encoder.write_all(&gzmsg.data).unwrap();
                bytes = *bytes_written.lock().unwrap();
                msg_count = *messages.lock().unwrap();
                pb.set_message(format!(
                    "{} messages processed: {}",
                    HumanBytes(bytes),
                    msg_count
                ));
                pb.tick();
            }
            encoder.finish().unwrap();
            pb.finish();
        });

        Ok((sender, handle))
    }
}
