use std::borrow::Borrow;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    pos: usize,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key: AsRef<[u8]> + ?Sized + 'a>(key: &'a Key) -> Xorcism<'a> {
        Xorcism {
            key: key.as_ref(),
            pos: 0,
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut().for_each(|byte| *byte ^= self.next_byte());
    }

    pub fn munge<Data: IntoIterator<Item = T>, T: Borrow<u8>>(
        &mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> {
        data.into_iter().map(|x| x.borrow() ^ self.next_byte())
    }

    fn next_byte(&mut self) -> u8 {
        let pos = self.pos;
        self.pos = (self.pos + 1) % self.key.len();
        self.key[pos]
    }

    pub fn reader(self, reader: impl Read) -> impl Read {
        XorcismReader::new(reader, self)
    }

    pub fn writer(self, writer: impl Write) -> impl Write {
        XorcismWriter::new(writer, self)
    }
}

struct XorcismWriter<'a, W: Write> {
    writer: W,
    xorcism: Xorcism<'a>,
}

impl<'a, W: Write> XorcismWriter<'a, W> {
    fn new(writer: W, xorcism: Xorcism<'a>) -> XorcismWriter<'a, W> {
        XorcismWriter { writer, xorcism }
    }
}

impl<W: Write> Write for XorcismWriter<'_, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buf = buf.to_vec();
        self.xorcism.munge_in_place(&mut buf);
        self.writer.write(&buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

struct XorcismReader<'a, R: Read> {
    reader: R,
    xorcism: Xorcism<'a>,
}

impl<'a, R: Read> XorcismReader<'a, R> {
    fn new(reader: R, xorcism: Xorcism<'a>) -> XorcismReader<'a, R> {
        XorcismReader { reader, xorcism }
    }
}

impl<R: Read> Read for XorcismReader<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.reader.read(buf)?;
        self.xorcism.munge_in_place(&mut buf[..n]);
        Ok(n)
    }
}
