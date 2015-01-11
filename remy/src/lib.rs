#![allow(unstable)]

mod cpu;
mod util;
mod mem;
mod pc;
mod stack;

pub enum Endianness {
    BigEndian,
    LittleEndian
}

impl Endianness {
	fn native() -> Endianness {
		if cfg!(target_endian = "big") {
			Endianness::BigEndian
		} else {
			Endianness::LittleEndian
		}
	}
}