use mem;

use std::{io,convert};

/// Cursor which implements the ability to read, write and seek over memory
pub struct Cursor<M> where M: mem::Memory {
    inner: M,
    pos: u64
}

impl<M> Cursor<M> where M: mem::Memory {
    /// Gets the current position of the cursor
    pub fn position(&self) -> u64 {
        self.pos
    }

    /// Sets the position of the cursor
    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<M> io::Read for Cursor<M> where M: mem::Memory {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        try!(self.inner.get(self.pos, buf));
        self.pos += buf.len() as u64;
        Ok(buf.len())
    }
}

impl<M> io::Write for Cursor<M> where M: mem::Memory {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        try!(self.inner.set(self.pos, buf));
        self.pos += buf.len() as u64;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<M> io::Seek for Cursor<M> where M: mem::Memory {
    fn seek(&mut self, style: io::SeekFrom) -> io::Result<u64> {
        let pos = match style {
            io::SeekFrom::Start(n) => { self.pos = n; return Ok(n) }
            io::SeekFrom::End(n) => self.inner.len() as i64 + n,
            io::SeekFrom::Current(n) => self.pos as i64 + n,
        };

        if pos < 0 {
            Err(io::Error::new(io::ErrorKind::InvalidInput,
                           "invalid seek to a negative position"))
        } else {
            self.pos = pos as u64;
            Ok(self.pos)
        }
    }
}

impl convert::From<mem::Error> for io::Error {
    fn from(e: mem::Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, e)
    }
}

pub fn cursor<M>(memory: M, start: u64) -> Cursor<M> where M: mem::Memory {
    Cursor {
        inner: memory,
        pos: start
    }
}
