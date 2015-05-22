use mem;
use systems::nes;

pub struct MemoryMap<'a> {
    ram: mem::Mirrored<mem::Fixed>,
    apu_io: mem::Fixed,
    cartridge: Option<&'a nes::Cartridge>
}

impl<'a> MemoryMap<'a> {
    pub fn new() -> MemoryMap<'a> {
        // 2KB internal ram mirrored through 0x1FFF
        let ram = mem::Mirrored::new(mem::Fixed::new(0x0800), 0x2000);

        // Create a black hole for APU/IO registers
        let apu_io = mem::Fixed::from_contents(vec![0xFF; 0x20]);

        MemoryMap {
            ram: ram,
            apu_io: apu_io,
            cartridge: None
        }
    }
}
