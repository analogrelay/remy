use mem;

use std::{io,convert};

/// Cursor which implements the ability to read and seek over memory
pub struct ReadCursor<'a, M> where M: mem::Memory + 'a {
    inner: &'a M,
    pos: u64
}

impl<'a, M> ReadCursor<'a, M> where M: mem::Memory + 'a {
    /// Gets the current position of the cursor
    pub fn position(&self) -> u64 {
        self.pos
    }

    /// Sets the position of the cursor
    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<'a, M> io::Read for ReadCursor<'a, M> where M: mem::Memory + 'a {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        try!(self.inner.get(self.pos, buf));
        self.pos += buf.len() as u64;
        Ok(buf.len())
    }
}

impl<'a, M> io::Seek for ReadCursor<'a, M> where M: mem::Memory + 'a {
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

pub fn read_cursor<'a, M>(memory: &'a M, start: u64) -> ReadCursor<'a, M> where M: mem::Memory {
    ReadCursor {
        inner: memory,
        pos: start
    }
}
