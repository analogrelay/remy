use mem;
use std::convert;

pub use self::ppu::Rp2C02;

// Contains code to emulate the PPU
pub mod ppu;

// Contains code to manage the registers on the PPU
pub mod registers;

pub struct ScreenBuffer<'a> {
    data: &'a mut [u8],
    pitch: usize,
}

impl<'a> ScreenBuffer<'a> {
    pub fn new(buffer: &'a mut [u8], pitch: usize) -> ScreenBuffer {
        ScreenBuffer {
            data: buffer,
            pitch: pitch
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        let base = (y * self.pitch) + (x * 3);

        // Put it to the screen
        self.data[base] = pixel.blue;
        self.data[base + 1] = pixel.green;
        self.data[base + 2] = pixel.red;
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
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

