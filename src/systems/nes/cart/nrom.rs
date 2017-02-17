use slog;

use mem;
use systems::nes;

struct Prg {
    ram: mem::Fixed,
    rom: mem::Fixed,
    log: slog::Logger,
}

pub struct NRom {
    prg: Prg,
    chr: mem::Empty
}

impl NRom {
    pub fn new(ram_size: usize, rom: Vec<u8>, logger: Option<slog::Logger>) -> NRom {
        NRom {
            prg: Prg {
                ram: mem::Fixed::new(ram_size),
                rom: mem::Fixed::from_contents(rom),
                log: unwrap_logger!(logger).new(o!("mapper" => "NRom", "cartridge" => true))
            },
            chr: mem::Empty
        }
    }
}

impl nes::Mapper for NRom {
    fn name(&self) -> &'static str { "NRom" }

    fn prg(&self) -> &mem::Memory
    {
        return &self.prg;
    }

    fn prg_mut(&mut self) -> &mut mem::Memory
    {
        return &mut self.prg;
    }

    fn chr(&self) -> &mem::Memory
    {
        return &self.chr;
    }

    fn chr_mut(&mut self) -> &mut mem::Memory
    {
        return &mut self.chr;
    }
}

impl mem::Memory for Prg {
    fn len(&self) -> u64 { 0xA000 }

    fn get_u8(&self, addr: u64) -> mem::Result<u8> {
        if addr < 0x6000 {
            // Out of range!
            error!(self.log,
                "error";
                "vaddr" => format!("${:04X}", addr),
                "error" => stringify!(mem::ErrorKind::OutOfBounds),
                "action" => "read");
            Err(mem::Error::with_detail(
                    mem::ErrorKind::OutOfBounds,
                    "memory access out of range addressable on NROM cartridge",
                    format!("${:4X} is below the addressable range of 0x6000-0xFFFF", addr)))
        } else if addr < 0x8000 {
            // RAM! Mirrored as needed
            let eaddr = (addr - 0x6000) % self.ram.len();
            trace!(self.log,
                "read";
                "vaddr" => format!("${:04X}", addr),
                "paddr" => format!("${:04X}", eaddr),
                "target" => "RAM",
                "action" => "read");
            self.ram.get_u8(eaddr)
        } else {
            // ROM! Mirrored again as needed
            let eaddr = (addr - 0x8000) % self.rom.len();
            trace!(self.log,
                "read";
                "vaddr" => format!("${:04X}", addr),
                "paddr" => format!("${:04X}", eaddr),
                "target" => "ROM",
                "action" => "read");
            self.rom.get_u8(eaddr)
        }
    }

    fn set_u8(&mut self, addr: u64, val: u8) -> mem::Result<()> {
        if addr < 0x6000 {
            // Out of range!
            error!(self.log,
                "error";
                "vaddr" => format!("${:04X}", addr),
                "error" => stringify!(mem::ErrorKind::OutOfBounds),
                "action" => "write");
            Err(mem::Error::with_detail(
                mem::ErrorKind::OutOfBounds,
                "memory access out of range addressable on NROM cartridge",
                format!("${:4X} is below the addressable range on NROM cartridge", addr)))
        } else if addr < 0x8000 {
            // RAM! Mirrored as needed
            let eaddr = (addr - 0x6000) % self.ram.len();
            trace!(self.log,
                "write";
                "vaddr" => format!("${:04X}", addr),
                "paddr" => format!("${:04X}", eaddr),
                "target" => "RAM",
                "action" => "write");
            self.ram.set_u8(eaddr, val)
        } else {
            // ROM! Can't write to that!
            error!(self.log,
                "write";
                "vaddr" => format!("${:04X}", addr),
                "error" => stringify!(mem::ErrorKind::MemoryNotWritable),
                "action" => "write");
            Err(mem::Error::with_detail(
                mem::ErrorKind::MemoryNotWritable,
                "cannot write to cartridge PRG ROM",
                format!("${:4X} is in the read-only memory on NROM cartridge", addr)))
        }
    }
}
