#![allow(unstable)]

pub mod cpu;
pub mod util;
pub mod mem;
pub mod pc;

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