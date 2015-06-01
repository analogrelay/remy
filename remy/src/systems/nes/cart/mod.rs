use mem;
use systems::nes;

pub use self::nrom;

mod nrom;

pub type Result<T> = ::std::result::Result<T, Error>;

pub enum Error {
    UnknownMapper
}

/// Represents a cartridge that has been loaded into the system
pub struct Cartridge {
    pub header: nes::RomHeader,
    pub prg: Box<mem::Memory>,
    pub chr: Box<mem::Memory>
}

pub fn load(rom: nes::Rom) -> Option<Cartridge> {
    // Pull apart the rom
    let nes::Rom { header, prg_rom, chr_rom } = rom;

    let (prg, chr) = create_mapper(&header, prg_rom, chr_rom);

    match mapper {
        Some(m) => Ok(Cartridge { header: header, prg: prg, chr: chr }),
        None => Err(Error::UnknownMapper)
    }
}

fn create_mappers(header: &nes::RomHeader, prg: Vec<u8>, chr: Vec<u8>) -> (Box<mem::Memory>, Box<mem::Memory>)
    match (header.cartridge.mapper, header.cartridge.submapper) {
        (0, _) => Some((Box::new(nrom::prg(0x2000, prg)), Box::new(nrom::chr(chr)))),
        _ => None
    }
}
