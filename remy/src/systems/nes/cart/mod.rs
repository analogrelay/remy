use mem;
use systems::nes;

pub mod nrom;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Error {
    UnknownMapper
}

/// Represents a cartridge that has been loaded into the system
pub struct Cartridge {
    pub header: nes::RomHeader,
    pub prg: Box<mem::Memory>,
    pub chr: Box<mem::Memory>
}

pub fn load(rom: nes::Rom) -> Result<Cartridge> {
    // Pull apart the rom
    let nes::Rom { header, prg, chr } = rom;

    if let Some((prg_rom, chr_rom)) = create_mappers(&header, prg, chr) {
        Ok(Cartridge { header: header, prg: prg_rom, chr: chr_rom })
    } else {
        Err(Error::UnknownMapper)
    }
}

fn create_mappers(header: &nes::RomHeader, prg: Vec<u8>, chr: Vec<u8>) -> Option<(Box<mem::Memory>, Box<mem::Memory>)> {
    match (header.cartridge.mapper, header.cartridge.submapper) {
        (0, _) => Some((Box::new(nrom::prg(0x2000, prg)), Box::new(nrom::chr(chr)))),
        _ => None
    }
}
