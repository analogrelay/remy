use slog;

use mem;
use systems::nes;

pub use self::nrom::NRom;

mod nrom;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum Error {
    UnknownMapper(u16, u8)
}

/// Represents a cartridge that has been loaded into the system
pub struct Cartridge {
    header: nes::RomHeader,
    pub mapper: Box<Mapper>
}

impl Cartridge {
    pub fn header(&self) -> &nes::RomHeader {
        &self.header
    }
}

pub trait Mapper {
    fn name(&self) -> &'static str;

    /// Gets a `Memory` representing the active PRG banks
    fn prg(&self) -> &mem::Memory;

    /// Gets a mutable `Memory` representing the active PRG banks
    fn prg_mut(&mut self) -> &mut mem::Memory;

    /// Gets a `Memory` representing the active CHR banks
    fn chr(&self) -> &mem::Memory;

    /// Gets a mutable `Memory` representing the active CHR banks
    fn chr_mut(&mut self) -> &mut mem::Memory;
}

impl Cartridge {
    pub fn new(header: nes::RomHeader, mapper: Box<Mapper>) -> Cartridge {
        Cartridge {
            header: header,
            mapper: mapper,
        }
    }

    /// Consumes the provided `Rom` and uses it to build a `Cartridge` to execute
    pub fn load(rom: nes::Rom, logger: Option<slog::Logger>) -> Result<Cartridge> {
        let log = unwrap_logger!(logger);

        // Pull apart the rom
        let nes::Rom { header, prg, chr } = rom;

        let mapper = create_mapper(&header, prg, chr, log.clone());

        match mapper {
            Some(m) => {
                info!(log,
                    "mapper_id" => header.cartridge.mapper,
                    "submapper_id" => header.cartridge.submapper,
                    "mapper" => m.name();
                    "loaded mapper {}.{} {}", header.cartridge.mapper, header.cartridge.submapper, m.name());

                Ok(Cartridge { header: header, mapper: m })
            },
            None => {
                error!(log,
                    "mapper_id" => header.cartridge.mapper,
                    "submapper_id" => header.cartridge.submapper,
                    "error" => stringify!(Error::UnknownMapper);
                    "unknown mapper {}.{}", header.cartridge.mapper, header.cartridge.submapper);
                Err(Error::UnknownMapper(header.cartridge.mapper, header.cartridge.submapper))
            }
        }
    }
}

fn create_mapper(header: &nes::RomHeader, prg: Vec<u8>, _chr: Vec<u8>, log: slog::Logger) -> Option<Box<Mapper>> {
    match (header.cartridge.mapper, header.cartridge.submapper) {
        (0, _) => Some(Box::new(NRom::new(0x2000, prg, Some(log)))),
        _ => None
    }
}