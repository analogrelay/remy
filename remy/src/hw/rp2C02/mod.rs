use mem;
use std::convert;

pub use self::ppu::{Rp2C02,ScreenBuffer};

// Contains code to emulate the PPU
pub mod ppu;

// Contains code to manage the registers on the PPU
pub mod registers;

pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Error {
    MemoryAccessError(mem::Error)
}

impl convert::From<mem::Error> for Error {
    fn from(err: mem::Error) -> Error {
        Error::MemoryAccessError(err)
    }
}

