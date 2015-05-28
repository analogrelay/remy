use mem;
use systems::nes;

pub struct MemoryMap {
    ram: mem::Fixed,
    cart: nes::Cartridge
}

impl MemoryMap {
    pub fn new(cart: nes::Cartridge) -> MemoryMap {
        MemoryMap {
            ram: mem::Fixed::new(0x0800),
            cart: cart
        }
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
            info!("read from ${:4X} going to cartridge mapper", addr);
            self.cart.mapper.prg().get_u8(addr)
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
            info!("write to ${:4X} going to cartridge mapper", addr);
            self.cart.mapper.prg_mut().set_u8(addr, val)
        }
    }
}
