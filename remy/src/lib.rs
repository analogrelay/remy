#![allow(unstable)]

mod cpu;
mod util;
mod mem;
mod pc;

pub enum Endianness {
    BigEndian,
    LittleEndian
}