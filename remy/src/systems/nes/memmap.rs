use mem;
use systems::nes;

/// Represents the memory map for a Nintendo Entertainment System
pub struct MemoryMap {
    mem: Mem,
    vmem: Vmem
}

pub struct Mem {
    ram: mem::Fixed,
    cart: Box<mem::Memory>
}

pub struct Vmem {
    ram: mem::Fixed,
    cart: Box<mem::Memory>
}

impl MemoryMap {
    /// Constructs a new `MemoryMap` with no Cartridge present
    pub fn new() -> MemoryMap {
        MemoryMap {
            ram: mem::Fixed::new(0x0800),
            cart: None
        }
    }


    /// Loads the provided cartridge into the `MemoryMap`, releasing the cartridge previously
    /// loaded, if any
    pub fn load(&mut self, cart: nes::Cartridge) {
        self.cart = Some(cart);
    }

    /// Releases the cartridge currently loaded, if any
    pub fn eject(&mut self) {
        self.cart = None;
    }
}

impl mem::Memory for MemoryMap {
    fn len(&self) -> u64 { 0xFFFF }

    fn get_u8(&self, addr: u64) -> mem::Result<u8> {
        if addr < 0x2000 {
            let eaddr = addr % 0x0800;
            info!("read from ${:4X} going to RAM at ${:4X}", addr, eaddr);
            self.ram.get_u8(eaddr)
        }
        else if addr < 0x4000 {
            let eaddr = (addr - 0x2000) % 0x0008;
            info!("read from ${:4X} going to PPU register at ${:4X}", addr, eaddr);
            // Todo: Do something!
            Ok(0)
        }
        else if addr < 0x4020 {
            let eaddr = addr - 0x4000;
            info!("read from ${:4X} going to APU or I/O register at ${:4X}", addr, eaddr);
            // Todo: Do something!
            Ok(0)
        } else {
            match self.cart {
                None => {
                    error!("read from ${:4X} failed due to missing cartridge", addr);
                    Err(mem::Error::new(
                            mem::ErrorKind::MemoryNotPresent,
                            "attempted to read from cartridge memory, but there is no cartridge present"))
                },
                Some(ref cart) => {
                    info!("read from ${:4X} going to cartridge mapper", addr);
                    cart.mapper.prg().get_u8(addr)
                }
            }
        }
    }

    fn set_u8(&mut self, addr: u64, val: u8) -> mem::Result<()> {
        if addr < 0x2000 {
            let eaddr = addr % 0x0800;
            info!("write to ${:4X} going to RAM at ${:4X}", addr, eaddr);
            self.ram.set_u8(eaddr, val)
        }
        else if addr < 0x4000 {
            let eaddr = (addr - 0x2000) % 0x0008;
            info!("write to ${:4X} going to PPU register at ${:4X}", addr, eaddr);
            // Todo: Do something!
            Ok(())
        }
        else if addr < 0x4020 {
            let eaddr = addr - 0x4000;
            info!("write to ${:4X} going to APU or I/O register at ${:4X}", addr, eaddr);
            // Todo: Do something!
            Ok(())
        } else {
            match self.cart {
                None => {
                    error!("write to ${:4X} failed due to missing cartridge", addr);
                    Err(mem::Error::new(
                            mem::ErrorKind::MemoryNotPresent,
                            "attempted to write to cartridge memory, but there is no cartridge present"))
                },
                Some(ref mut cart) => {
                    info!("write to ${:4X} going to cartridge mapper", addr);
                    cart.mapper.prg_mut().set_u8(addr, val)
                }
            }
        }
    }
}
