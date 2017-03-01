use slog;

use mem;
use systems::nes;

/// Represents the memory map for a Nintendo Entertainment System
pub struct MemoryMap {
    ram: mem::Fixed,
    cart: Option<nes::Cartridge>,
    log: slog::Logger,
    memlog: slog::Logger
}

impl MemoryMap {
    /// Constructs a new `MemoryMap` with no Cartridge present
    pub fn new(logger: Option<slog::Logger>) -> MemoryMap {
        let log = unwrap_logger!(logger);
        let memlog = log.new(o!("cartridge" => false));
        MemoryMap {
            ram: mem::Fixed::new(0x0800),
            cart: None,
            log: log,
            memlog: memlog
        }
    }


    /// Loads the provided cartridge into the `MemoryMap`, releasing the cartridge previously
    /// loaded, if any
    pub fn load(&mut self, cart: nes::Cartridge) {
        info!(self.log,
            "mapper" => cart.mapper.name();
            "Loaded {} cartridge", cart.mapper.name());
        self.cart = Some(cart);
    }

    /// Releases the cartridge currently loaded, if any
    pub fn eject(&mut self) {
        let old_cart = self.cart.take();
        if old_cart.is_none() {
            panic!("Can't eject cartridge, there is no cartridge loaded!");
        }
        let old_cart = old_cart.unwrap();

        info!(self.log,
            "mapper" => old_cart.mapper.name();
            "Ejecting {} cartridge", old_cart.mapper.name());
    }
}

impl mem::Memory for MemoryMap {
    fn len(&self) -> u64 { 0xFFFF }

    fn get_u8(&self, addr: u64) -> mem::Result<u8> {
        if addr < 0x2000 {
            let eaddr = addr % 0x0800;
            trace!(self.memlog,
                "read";
                "vaddr" => format!("${:04X}", addr),
                "paddr" => format!("${:04X}", eaddr),
                "target" => "RAM",
                "action" => "read");
            self.ram.get_u8(eaddr)
        }
        else if addr < 0x4000 {
            let eaddr = (addr - 0x2000) % 0x0008;
            trace!(self.memlog,
                "read";
                "vaddr" => format!("${:04X}", addr),
                "paddr" => format!("${:04X}", eaddr),
                "target" => "PPU",
                "action" => "read");
            // Todo: Do something!
            Ok(0)
        }
        else if addr < 0x4200 {
            let eaddr = addr - 0x4000;
            trace!(self.memlog,
                "read";
                "vaddr" => format!("${:04X}", addr),
                "paddr" => format!("${:04X}", eaddr),
                "target" => "APU/IO",
                "action" => "read");
            // Todo: Do something!
            Ok(0)
        } else {
            match self.cart {
                None => {
                    error!(self.memlog,
                        "error";
                        "vaddr" => format!("${:04X}", addr),
                        "target" => "Cartridge",
                        "action" => "read",
                        "error" => stringify!(mem::ErrorKind::MemoryNotPresent));
                    Err(mem::Error::new(
                            mem::ErrorKind::MemoryNotPresent,
                            "Attempted to read from cartridge memory, but there is no cartridge present"))
                },
                Some(ref cart) => {
                    // Cartridge has it's own logging
                    cart.mapper.prg().get_u8(addr)
                }
            }
        }
    }

    fn set_u8(&mut self, addr: u64, val: u8) -> mem::Result<()> {
        if addr < 0x2000 {
            let eaddr = addr % 0x0800;
            trace!(self.memlog,
                "write";
                "vaddr" => format!("${:04X}", addr),
                "paddr" => format!("${:04X}", eaddr),
                "target" => "RAM",
                "action" => "write");
            self.ram.set_u8(eaddr, val)
        }
        else if addr < 0x4000 {
            let eaddr = (addr - 0x2000) % 0x0008;
            trace!(self.memlog,
                "write";
                "vaddr" => format!("${:04X}", addr),
                "paddr" => format!("${:04X}", eaddr),
                "target" => "PPU",
                "action" => "write");
            // Todo: Do something!
            Ok(())
        }
        else if addr < 0x4200 {
            let eaddr = addr - 0x4000;
            trace!(self.memlog,
                "write";
                "vaddr" => format!("${:04X}", addr),
                "paddr" => format!("${:04X}", eaddr),
                "target" => "APU/IO",
                "action" => "write");
            // Todo: Do something!
            Ok(())
        } else {
            match self.cart {
                None => {
                    error!(self.memlog,
                        "error";
                        "vaddr" => format!("${:04X}", addr),
                        "target" => "Cartridge",
                        "action" => "write",
                        "error" => stringify!(mem::ErrorKind::MemoryNotPresent));
                    Err(mem::Error::new(
                            mem::ErrorKind::MemoryNotPresent,
                            "Attempted to write to cartridge memory, but there is no cartridge present"))
                },
                Some(ref mut cart) => {
                    // Cartridge has it's own logging
                    cart.mapper.prg_mut().set_u8(addr, val)
                }
            }
        }
    }
}
