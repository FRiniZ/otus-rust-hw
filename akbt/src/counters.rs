use std::{io::Read, io::Write, sync::atomic::AtomicUsize};
pub struct Counter {
    pub messages: u64,
}

pub struct ByteCounter<'a, W> {
    inner: W,
    count: &'a AtomicUsize,
}

impl<'a, W> ByteCounter<'a, W>
where
    W: Write,
{
    pub fn new(inner: W, count: &'a AtomicUsize) -> Self {
        ByteCounter { inner, count }
    }
}

impl<'a, W> Write for ByteCounter<'a, W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let res = self.inner.write(buf);
        if let Ok(size) = res {
            self.count
                .fetch_add(size, std::sync::atomic::Ordering::SeqCst);
        }
        res
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl<'a, R> Read for ByteCounter<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = self.inner.read(buf);
        if let Ok(size) = res {
            self.count
                .fetch_add(size, std::sync::atomic::Ordering::SeqCst);
        }
        res
    }
}
