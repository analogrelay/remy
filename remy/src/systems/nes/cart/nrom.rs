use mem;

pub struct Prg {
    ram: mem::Fixed,
    rom: mem::Fixed
}

#[allow(dead_code)]
pub struct Chr {
    rom: mem::Fixed
}

pub fn prg(ram_size: usize, rom: Vec<u8>) -> Prg {
    Prg {
        ram: mem::Fixed::new(ram_size),
        rom: mem::Fixed::from_contents(rom)
    }
}

pub fn chr(rom: Vec<u8>) -> Chr {
    Chr {
        rom: mem::Fixed::from_contents(rom)
    }
}

impl mem::Memory for Prg {
    fn len(&self) -> u64 { 0xA000 }

    fn get_u8(&self, addr: u64) -> mem::Result<u8> {
        if addr < 0x6000 {
            // Out of range!
            Err(mem::Error::with_detail(
                    mem::ErrorKind::OutOfBounds,
                    "memory access out of range addressable on NROM cartridge",
                    format!("${:4X} is below the addressable range of 0x6000-0xFFFF", addr)))
        } else if addr < 0x8000 {
            // RAM! Mirrored as needed
            let eaddr = (addr - 0x6000) % self.ram.len();
            info!("read from ${:4X} going to cartridge PRG RAM", addr);
            self.ram.get_u8(eaddr)
        } else {
            // ROM! Mirrored again as needed
            let eaddr = (addr - 0x8000) % self.rom.len();
            info!("read from ${:4X} going to cartridge PRG ROM", addr);
            self.rom.get_u8(eaddr)
        }
    }

    fn set_u8(&mut self, addr: u64, val: u8) -> mem::Result<()> {
        if addr < 0x6000 {
            // Out of range!
            Err(mem::Error::with_detail(
                    mem::ErrorKind::OutOfBounds,
                    "memory access out of range addressable on NROM cartridge",
                    format!("${:4X} is below the addressable range of 0x6000-0xFFFF", addr)))
        } else if addr < 0x8000 {
            // RAM! Mirrored as needed
            let eaddr = (addr - 0x6000) % self.ram.len();
            info!("write to ${:4X} going to cartridge PRG RAM", addr);
            self.ram.set_u8(eaddr, val)
        } else {
            // ROM! Can't write to that!
            Err(mem::Error::new(
                    mem::ErrorKind::MemoryNotWritable,
                    "cannot write to cartridge PRG ROM"))
        }
    }
}

impl mem::Memory for Chr {
    fn len(&self) -> u64 { unimplemented!() }

    fn get_u8(&self, _addr: u64) -> mem::Result<u8> {
        unimplemented!()
    }

    fn set_u8(&mut self, _addr: u64, _val: u8) -> mem::Result<()> {
        unimplemented!()
    }
}
