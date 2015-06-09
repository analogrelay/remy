use mem;
use hw::rp2C02;

pub struct Mem {
    ram: mem::Fixed,
    prg: Option<Box<mem::Memory>>,
    pub ppu: rp2C02::Rp2C02
}

impl Mem {
    pub fn new(ppu: rp2C02::Rp2C02) -> Mem {
        Mem {
            ram: mem::Fixed::new(0x0800),
            prg: None,
            ppu: ppu
        }
    }

    pub fn load(&mut self, prg: Box<mem::Memory>) {
        self.prg = Some(prg);
    }

    pub fn eject(&mut self) {
        self.prg = None
    }
}

impl mem::Memory for Mem {
    fn len(&self) -> u64 { 0xFFFF }

    fn get_u8(&self, addr: u64) -> mem::Result<u8> {
        if addr < 0x2000 {
            let eaddr = addr % 0x0800;
            trace!("read from ${:4X} going to RAM at ${:4X}", addr, eaddr);
            self.ram.get_u8(eaddr)
        }
        else if addr < 0x4000 {
            let eaddr = (addr - 0x2000) % 0x0008;
            trace!("read from ${:4X} going to PPU register at ${:4X}", addr, eaddr);
            // Todo: Do something!
            Ok(0)
        }
        else if addr < 0x4020 {
            let eaddr = addr - 0x4000;
            trace!("read from ${:4X} going to APU or I/O register at ${:4X}", addr, eaddr);
            // Todo: Do something!
            Ok(0)
        } else {
            match self.prg {
                None => {
                    error!("read from ${:4X} failed due to missing cartridge", addr);
                    Err(mem::Error::new(
                            mem::ErrorKind::MemoryNotPresent,
                            "attempted to read from cartridge memory, but there is no cartridge present"))
                },
                Some(ref prg) => {
                    trace!("read from ${:4X} going to cartridge mapper", addr);
                    prg.get_u8(addr)
                }
            }
        }
    }

    fn set_u8(&mut self, addr: u64, val: u8) -> mem::Result<()> {
        if addr < 0x2000 {
            let eaddr = addr % 0x0800;
            trace!("write to ${:4X} going to RAM at ${:4X}", addr, eaddr);
            self.ram.set_u8(eaddr, val)
        }
        else if addr < 0x4000 {
            let eaddr = (addr - 0x2000) % 0x0008;
            trace!("write to ${:4X} going to PPU register at ${:4X}", addr, eaddr);
            // Todo: Do something!
            Ok(())
        }
        else if addr < 0x4020 {
            let eaddr = addr - 0x4000;
            trace!("write to ${:4X} going to APU or I/O register at ${:4X}", addr, eaddr);
            // Todo: Do something!
            Ok(())
        } else {
            match self.prg {
                None => {
                    error!("write to ${:4X} failed due to missing cartridge", addr);
                    Err(mem::Error::new(
                            mem::ErrorKind::MemoryNotPresent,
                            "attempted to write to cartridge memory, but there is no cartridge present"))
                },
                Some(ref mut prg) => {
                    trace!("write to ${:4X} going to cartridge mapper", addr);
                    prg.set_u8(addr, val)
                }
            }
        }
    }
}
