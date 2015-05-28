pub use self::cart::{Mapper,Cartridge};
pub use self::rom::{Rom,RomHeader,load_rom};

use std::convert;
use hw::mos6502::{self,exec};
use hw::mos6502::instr::decoder;

/// Contains code to load and manipulate ROMs in the iNES and NES 2.0 formats
pub mod rom;

/// Contains code to emulate cartridge hardware (Mappers, etc.)
pub mod cart;

mod memmap;

pub type Result<T> = ::std::result::Result<T, Error>;

pub enum Error {
    InstructionDecodeError(decoder::Error),
    ExecutionError(exec::Error)
}

impl convert::From<decoder::Error> for Error {
    fn from(err: decoder::Error) -> Error {
        Error::InstructionDecodeError(err)
    }
}

impl convert::From<exec::Error> for Error {
    fn from(err: exec::Error) -> Error {
        Error::ExecutionError(err)
    }
}

pub struct Nes {
    cpu: mos6502::Mos6502,
    mem: memmap::MemoryMap
}

impl Nes {
    pub fn new(cart: Cartridge) -> Nes {
        // Set up the CPU
        let mut cpu = mos6502::Mos6502::without_bcd();
        cpu.flags.replace(mos6502::Flags::new(0x24));

        Nes {
            cpu: cpu,
            mem: memmap::MemoryMap::new(cart)
        }
    }

    pub fn step(&mut self) -> Result<()> {
        // Fetch next instruction
        let instr: mos6502::Instruction = try!(self.cpu.pc.decode(&self.mem));

        // Dispatch the instruction
        try!(mos6502::dispatch(instr, &mut self.cpu, &mut self.mem));

        Ok(())
    }
}
