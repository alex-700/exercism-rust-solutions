use std::io::{Read, Result, Write};

#[derive(Default)]
struct Stats {
    bytes_through: usize,
    calls: usize,
}

impl Stats {
    fn handle(&mut self, res: Result<usize>) -> Result<usize> {
        self.calls += 1;
        self.bytes_through += *res.as_ref().unwrap_or(&0);
        res
    }
}

pub struct ReadStats<R: Read> {
    wrapped: R,
    stats: Stats,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            stats: Default::default(),
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.stats.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.stats.calls
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.stats.handle(self.wrapped.read(buf))
    }
}

pub struct WriteStats<W: Write> {
    wrapped: W,
    stats: Stats,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            stats: Default::default(),
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.stats.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.stats.calls
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stats.handle(self.wrapped.write(buf))
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
