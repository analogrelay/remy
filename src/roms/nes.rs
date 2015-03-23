use std::{error,io};

const HEADER_SIZE: usize = 16;
const PRG_BANK_SIZE: usize = 16384;
const CHR_BANK_SIZE: usize = 8192;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidHeader,
    InvalidSignature,
    EndOfFileDuringBank,
    IoError(io::Error)
}

impl error::FromError<io::Error> for Error {
    fn from_error(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

#[derive(Copy, Debug, PartialEq, Eq)]
pub enum RomVersion {
    INES,
    NES2
}

#[derive(Copy, Debug, PartialEq, Eq)]
pub enum TvSystem {
    Unknown,
    NTSC,
    PAL,
    Dual
}

#[derive(Debug)]
pub struct RamSize {
    battery_backed: u16,
    total: u16
}

impl RamSize {
    pub fn empty() -> RamSize {
        RamSize { battery_backed: 0, total: 0 }
    }

    pub fn from_header_byte(val: u8, version: RomVersion) -> RamSize {
        match version {
            RomVersion::INES => RamSize::empty(),
            RomVersion::NES2 => {
                let bat = get_full_size(((val & 0xF0) >> 4) as u16);
                let non_bat = get_full_size((val & 0x0F) as u16);

                RamSize {
                    battery_backed: bat,
                    total: bat + non_bat
                }
            }
        }
    }
}

fn get_full_size(inp: u16) -> u16 {
    use std::num::Int;
    match inp {
        0 => 0,
        _ => 2.pow(6 + inp as u32)
    }
}

#[derive(Debug)]
pub struct Header {
    prg_rom_size: u16,
    chr_rom_size: u16,
    prg_ram_size: RamSize,
    chr_ram_size: RamSize,
    mapper: u16,
    submapper: u8,
    version: RomVersion,
    vertical_arrangement: bool,
    four_screen_vram: bool,
    sram_battery_backed: bool,
    sram_present: bool,
    trainer_present: bool,
    vs_unisystem: bool,
    playchoice_10: bool,
    tv_system: TvSystem,
    bus_conflicts: bool
}

pub struct Rom {
    header: Header,
    pub prg_banks: Vec<Vec<u8>>,
    pub chr_banks: Vec<Vec<u8>>
}

impl ::std::fmt::Debug for Rom {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        fmt.debug_struct("Rom")
            .field("header", &self.header)
            .field("prg_banks", &self.prg_banks.len())
            .field("chr_banks", &self.chr_banks.len())
            .finish()
    }
}

pub fn read<R>(input: &mut R) -> Result<Rom> where R: io::Read {
    // Load header
    let header = try!(read_header(input));

    // Read rom banks
    let prg_banks = try!(read_banks(input, header.prg_rom_size, PRG_BANK_SIZE)); 
    let chr_banks = try!(read_banks(input, header.chr_rom_size, CHR_BANK_SIZE));

    Ok(Rom {
        header: header,
        prg_banks: prg_banks,
        chr_banks: chr_banks
    })
}

fn read_banks<R>(input: &mut R, bank_count: u16, bank_size: usize) -> Result<Vec<Vec<u8>>> where R: io::Read {
    let mut banks = Vec::with_capacity(bank_count as usize);
    for _ in (0..bank_count) {
        banks.push(try!(read_bank(input, bank_size)));
    }
    Ok(banks)
}

fn read_bank<R>(input: &mut R, bank_size: usize) -> Result<Vec<u8>> where R: io::Read {
    use std::io::Read;
    Ok(try!(input.take(bank_size as u64).bytes().collect()))
}

fn read_header<R>(input: &mut R) -> Result<Header> where R: io::Read {
    // Read the header into memory
    let mut header = [0u8; HEADER_SIZE];
    let read = try!(input.read(&mut header));
    if read != HEADER_SIZE {
        return Err(Error::InvalidHeader);
    } else if !verify_signature(&header[0..4]) {
        return Err(Error::InvalidSignature)
    }

    // Detect version
    // Based on algorithm in http://wiki.nesdev.com/w/index.php/INES#Variant_comparison
    let version = if header[7] & 0x0C == 0x08 {
        RomVersion::NES2
    } else {
        RomVersion::INES
    };

    // Read ROM sizes 
    let prg_size = match version {
        RomVersion::INES => header[4] as u16,
        RomVersion::NES2 => header[4] as u16 | ((header[9] & 0x0F) as u16)
    };
    let chr_size = match version {
        RomVersion::INES => header[5] as u16,
        RomVersion::NES2 => header[5] as u16 | (((header[9] & 0xF0) >> 4) as u16)
    };


    // Load mapper number
    let mut mapper = ((header[6] & 0xF0) >> 4) as u16;
    let mut submapper : u8 = 0;
    let archaic = header[12..15].iter().all(|i| { *i == 0 });

    // If this isn't Archaic iNES, read the second nybble
    if version == RomVersion::INES || !archaic {
        mapper = (mapper | ((header[7] as u16 & 0xF0))) as u16;
    }

    // If this is NES 2.0, read the third nybble and submapper
    if version == RomVersion::NES2 {
        mapper = (mapper | ((header[8] as u16 & 0x0F) << 8)) as u16;
        submapper = (header[8] & 0xF0) << 4;
    }

    // Read TV System
    let tv_system = match version {
        RomVersion::INES if archaic => TvSystem::Unknown,
        RomVersion::INES => if header[9] & 0x01 == 0 { TvSystem::NTSC } else { TvSystem::PAL },
        RomVersion::NES2 => if header[12] & 0x02 != 0 { 
            TvSystem::Dual
        } else if header[12] & 0x01 != 0 {
            TvSystem::PAL
        } else {
            TvSystem::NTSC
        }
    };

    // Read Ram Sizes
    let prg_ram = RamSize::from_header_byte(header[10], version);
    let chr_ram = RamSize::from_header_byte(header[11], version);

    Ok(Header {
        prg_rom_size: prg_size,
        chr_rom_size: chr_size,
        prg_ram_size: prg_ram,
        chr_ram_size: chr_ram,
        mapper: mapper,
        submapper: submapper,
        version: version,
        vertical_arrangement: (header[6] & 0x01) == 0,
        four_screen_vram: (header[6] & 0x08) != 0,
        sram_battery_backed: (header[6] & 0x02) != 0,
        sram_present: (header[10] & 0x10) != 0,
        trainer_present: (header[6] & 0x04) != 0,
        vs_unisystem: (header[7] & 0x01) != 0,
        playchoice_10: (header[7] & 0x02) != 0,
        tv_system: tv_system,
        bus_conflicts: (header[10] & 0x20) != 0
    })
}

fn verify_signature(sig: &[u8]) -> bool {
    sig.len() == 4 &&
        sig[0] == 0x4E && // 'N'
        sig[1] == 0x45 && // 'E'
        sig[2] == 0x53 && // 'S'
        sig[3] == 0x1A    // EOF
}
