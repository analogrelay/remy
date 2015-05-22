pub use self::cart::Cartridge;
pub use self::rom::{Rom,load_rom};

use hw::mos6502;

/// Contains code to load and manipulate ROMs in the iNES and NES 2.0 formats
pub mod rom;

/// Contains code to emulate cartridge hardware (Mappers, etc.)
pub mod cart;

mod memmap;

pub struct Nes<'a> {
    cpu: mos6502::Mos6502,
    mem: memmap::MemoryMap<'a>,
    rom: Option<Rom>
}

impl<'a> Nes<'a> {
    pub fn new() -> Nes<'a> {
        // Set up the memory map
        let mem = memmap::MemoryMap::new();

        // Set up the CPU
        let mut cpu = mos6502::Mos6502::without_bcd();
        cpu.flags.replace(mos6502::Flags::new(0x24));
        cpu.pc.set(0xC000);

        Nes {
            cpu: cpu,
            mem: mem,
            rom: None
        }
    }

    pub fn load(&mut self, rom: Rom) {
        self.rom = Some(rom);

        // Set up the necessary additional memory
    }
}
