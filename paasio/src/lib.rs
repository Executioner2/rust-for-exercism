use std::io::{Read, Result, Write};
use std::marker::PhantomData;
// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    buf: R,
    reads: usize,
    bytes_through: usize,
    _phantom: PhantomData<R>,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(buf: R) -> ReadStats<R> {
        Self {
            buf,
            reads: 0,
            bytes_through: 0,
            _phantom: PhantomData,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.buf
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    /// Collect statistics about this call reading {buf:?}
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.buf.read(buf) {
            Ok(n) => {
                self.reads += 1;
                self.bytes_through += n;
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct WriteStats<W> {
    buf: W,
    writes: usize,
    bytes_through: usize,
    _phantom: PhantomData<W>,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(buf: W) -> WriteStats<W> {
        Self {
            buf,
            writes: 0,
            bytes_through: 0,
            _phantom: PhantomData,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.buf
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.buf.write(buf) {
            Ok(n) => {
                self.writes += 1;
                self.bytes_through += n;
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.buf.flush()
    }
}
