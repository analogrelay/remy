pub use self::cart::Cartridge;
pub use self::rom::{Rom,load_rom};

/*
use hw::mos6502;
use mem;
*/

/// Contains code to load and manipulate ROMs in the iNES and NES 2.0 formats
pub mod rom;

/// Contains code to emulate cartridge hardware (Mappers, etc.)
pub mod cart;

/*
pub struct Nes<'a> {
    cpu: mos6502::Mos6502,
    mem: MemoryMap,
    rom: Option<Rom>
}

impl<'a> Nes<'a> {
    pub fn new() -> Nes<'a> {
        // 2KB internal ram mirrored through 0x1FFF
        let ram = Box::new(mem::Mirrored::new(mem::Fixed::new(0x0800), 0x2000));

        // Create a black hole for APU/IO registers
        let apu_io = Box::new(mem::Fixed::from_contents(vec![0xFF; 0x20]));

        // Set up the virtual memory
        let mut mem = mem::Virtual::new();
        mem.attach(0x0000, ram).unwrap();
        mem.attach(0x4000, apu_io).unwrap();

        // Set up the CPU
        let mut cpu = mos6502::Mos6502::without_bcd();
        cpu.flags.replace(mos6502::Flags::new(0x24));
        cpu.pc.set(0xC000);

        Nes {
            cpu: cpu,
            mem: mem
        }
    }

    pub fn load(&mut self, rom: Rom) {
        self.rom = Some(rom);

        // Set up the necessary additional memory
    }
}
*/
