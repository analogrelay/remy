use mem;

use std::{io,convert};

/// Cursor which implements the ability to read and seek over memory
pub struct ReadCursor<'a, M> where M: mem::Memory + 'a {
    inner: &'a M,
    pos: u64
}

/// Cursor which implements the ability to read, write and seek over memory
pub struct Cursor<'a, M> where M: mem::Memory + 'a {
    inner: &'a mut M,
    pos: u64
}

macro_rules! cursor_impl {
    () => {
        /// Gets the current position of the cursor
        pub fn position(&self) -> u64 {
            self.pos
        }

        /// Sets the position of the cursor
        pub fn set_position(&mut self, pos: u64) {
            self.pos = pos;
        }
    }
}

impl<'a, M> ReadCursor<'a, M> where M: mem::Memory + 'a { cursor_impl!{} }
impl<'a, M> Cursor<'a, M> where M: mem::Memory + 'a { cursor_impl!{} }

macro_rules! read_impl {
    () => {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            try!(self.inner.get(self.pos, buf));
            self.pos += buf.len() as u64;
            Ok(buf.len())
        }
    }
}

impl<'a, M> io::Read for ReadCursor<'a, M> where M: mem::Memory + 'a { read_impl!{} }
impl<'a, M> io::Read for Cursor<'a, M> where M: mem::Memory + 'a { read_impl!{} }

macro_rules! seek_impl {
    () => {
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
}

impl<'a, M> io::Seek for ReadCursor<'a, M> where M: mem::Memory + 'a { seek_impl!{} }
impl<'a, M> io::Seek for Cursor<'a, M> where M: mem::Memory + 'a { seek_impl!{} }

impl<'a, M> io::Write for Cursor<'a, M> where M: mem::Memory + 'a {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        try!(self.inner.set(self.pos, buf));
        self.pos += buf.len() as u64;
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

impl convert::From<mem::Error> for io::Error {
    fn from(e: mem::Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, e)
    }
}

/// Creates a read-only cursor pointing in to memory which allows the consumer to view the memory as
/// an I/O stream.
pub fn read_cursor<'a, M>(memory: &'a M, start: u64) -> ReadCursor<'a, M> where M: mem::Memory {
    ReadCursor {
        inner: memory,
        pos: start
    }
}

/// Creates a read/write cursor pointing in to memory which allows the consumer to view the memory as
/// an I/O stream.
pub fn cursor<'a, M>(memory: &'a mut M, start: u64) -> Cursor<'a, M> where M: mem::Memory {
    Cursor {
        inner: memory,
        pos: start
    }
}
