pub use self::cart::{Mapper,Cartridge};
pub use self::rom::{Rom,RomHeader,load_rom};

use slog;
use std::convert;

use mem;
use hw::mos6502::{self,exec};
use hw::mos6502::instr::decoder;

//use hw::rp2C02;

/// Contains code to load and manipulate ROMs in the iNES and NES 2.0 formats
pub mod rom;

/// Contains code to emulate cartridge hardware (Mappers, etc.)
pub mod cart;

mod memmap;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
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

/// Represents a complete NES system, including all necessary hardware and memory
pub struct Nes {
    pub cpu: mos6502::Mos6502,
    pub mem: memmap::MemoryMap,
    
    log: slog::Logger
}

impl Nes {
    /// Construct a new NES
    pub fn new(logger: Option<slog::Logger>) -> Nes {
        let log = unwrap_logger!(logger);

        // Set up the CPU
        let mut cpu = mos6502::Mos6502::without_bcd();
        cpu.flags.replace(mos6502::Flags::new(0x24));

        Nes {
            cpu: cpu,
            mem: memmap::MemoryMap::new(Some(log.clone())),
            log: log
        }
    }

    /// Gets a mutable reference to the current memory
    pub fn mem_mut(&mut self) -> &mut mem::Memory {
        &mut self.mem
    }

    /// Gets an immutable reference to the current memory
    pub fn mem(&self) -> &mem::Memory {
        &self.mem
    }

    /// Loads a cartridge into the NES
    pub fn load(&mut self, cart: Cartridge) {
        self.mem.load(cart);
    }

    /// Ejects the cartridge from the NES
    pub fn eject(&mut self) {
        self.mem.eject();
    }

    /// Runs a single frame of the system
    pub fn step(&mut self) -> Result<()> {
        // Fetch next instruction
        let instr: mos6502::Instruction = try!(self.cpu.pc.decode(&self.mem));

        // Dispatch the instruction
        trace!(self.log,
            "instr" => instr,
            "cycle" => self.cpu.clock.get();
            "dispatching");
        try!(mos6502::dispatch(instr, &mut self.cpu, &mut self.mem, Some(self.log.clone())));
        trace!(self.log,
            "instr" => instr,
            "cycle" => self.cpu.clock.get();
            "dispatched");

        // Run the PPU as necessary
        //let cycles = self.cpu.clock.get();
        //self.ppu.step(cycles);

        Ok(())
    }
}
