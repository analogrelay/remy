pub use self::rom::{Rom,RomHeader,load_rom};

use std::convert;

use mem;
use hw::mos6502::{self,exec};
use hw::mos6502::instr::decoder;
use hw::rp2C02;

pub use hw::rp2C02::ppu::PIXELS_PER_SCANLINE;
pub use hw::rp2C02::ppu::PIXELS_PER_SCREEN;
pub use hw::rp2C02::ppu::SCANLINES_PER_FRAME;

/// Contains code to load and manipulate ROMs in the iNES and NES 2.0 formats
pub mod rom;

/// Contains code to emulate cartridge hardware (Mappers, etc.)
pub mod cart;

mod memmap;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InstructionDecodeError(decoder::Error),
    ExecutionError(exec::Error),
    PpuError(rp2C02::Error),
    NoCartridgeInserted
}

impl convert::From<rp2C02::Error> for Error {
    fn from(err: rp2C02::Error) -> Error {
        Error::PpuError(err)
    }
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
    cpu: mos6502::Mos6502,
    mem: memmap::Mem,
    vmem: Option<Box<mem::Memory>>,
    rom_header: Option<rom::RomHeader>
}

impl Nes {
    /// Construct a new NES
    pub fn new() -> Nes {
        // Set up the CPU
        let mut cpu = mos6502::Mos6502::without_bcd();
        cpu.flags.replace(mos6502::Flags::new(0x24));

        let ppu = rp2C02::Rp2C02::new();

        Nes {
            cpu: cpu,
            mem: memmap::Mem::new(ppu),
            vmem: None,
            rom_header: None
        }
    }

    /// Loads a cartridge into the NES
    pub fn load(&mut self, rom: rom::Rom) -> cart::Result<()> {
        let cart::Cartridge { header, prg, chr } = try!(cart::load(rom));

        self.rom_header = Some(header);
        self.mem.load(prg);
        self.vmem = Some(chr);
        Ok(())
    }

    /// Ejects the cartridge from the NES
    pub fn eject(&mut self) {
        self.rom_header = None;
        self.mem.eject();
        self.vmem = None;
    }

    /// Performs a single CPU cycle, and the matching PPU cycles.
    pub fn step(&mut self, screen: &mut rp2C02::ScreenBuffer) -> Result<()> {
        // Fetch next instruction
        let instr: mos6502::Instruction = try!(self.cpu.pc.decode(&self.mem));

        // Dispatch the instruction
        debug!("dispatching {:?}", instr);
        try!(mos6502::dispatch(instr, &mut self.cpu, &mut self.mem));

        // Run the PPU as necessary
        let cycles = self.cpu.clock.get();
        debug!("running PPU to cycle: {}", cycles);
        if let Some(ref mut vmem) = self.vmem {
            try!(self.mem.ppu.step(cycles, &mut **vmem, screen));
            Ok(())
        } else {
            Err(Error::NoCartridgeInserted)
        }
    }
}
