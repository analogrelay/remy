use std::{error,io};

use systems::nes;

const HEADER_SIZE: usize = 16;
const PRG_BANK_SIZE: usize = 16384;
const CHR_BANK_SIZE: usize = 8192;

/// Represents the result of an operation performed on a ROM file
pub type Result<T> = ::std::result::Result<T, Error>;

/// Represents an error that occurs while operating on a ROM file
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Indicates that the head of the ROM file is invalid
    InvalidHeader,

    /// Indicates that the signature in the ROM file is invalid
    InvalidSignature,

    /// Indicates that an unexpected end-of-file was reached while reading a ROM bank
    EndOfFileDuringBank,

    /// Indicates that an I/O error occurred while reading/writing to a ROM file
    IoError(io::Error)
}

impl error::FromError<io::Error> for Error {
    fn from_error(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

/// Describes the television system expected by the ROM
#[derive(Copy, Debug, PartialEq, Eq)]
pub enum TvSystem {
    /// Indicates that the television system is not known 
    Unknown,

    /// Indicates that the ROM requires the NTSC television system
    NTSC,

    /// Indicates that the ROM requires the PAL television system
    PAL,

    /// Indicates that the ROM is compatible with either the NTSC or the PAL television system
    Dual
}

/// Describes the version of a ROM
#[derive(Copy, Debug, PartialEq, Eq)]
pub enum Version {
    /// Indicates that the ROM is in INES format
    INES,

    /// Indicates that the ROM is in NES 2.0 format
    NES2
}

/// Describes the size of a RAM bank
#[derive(Debug)]
pub struct RamSize {
    /// Indicates the amount of RAM that is battery-backed
    pub battery_backed: u16,

    /// Indicates the total amount of RAM (sum of battery-backed and non-battery-backed RAM)
    pub total: u16
}

impl RamSize {
    /// Creates a `RamSize` indicating no RAM
    pub fn empty() -> RamSize {
        RamSize { battery_backed: 0, total: 0 }
    }

    /// Creates a `RamSize` based on the RAM size byte of the iNES/NES2.0 header
    pub fn from_header_byte(val: u8, version: Version) -> RamSize {
        match version {
            Version::INES => RamSize::empty(),
            Version::NES2 => {
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

/// Represents the header values of an iNES/NES2.0 ROM
#[derive(Debug)]
pub struct Header {
    /// The size of the PRG ROM in 16K Banks
    pub prg_rom_size: u16,

    /// The size of the CHR ROM in 8K Banks
    pub chr_rom_size: u16,

    /// The size of the PRG RAM, if any
    pub prg_ram_size: RamSize,

    /// The size of the CHR RAM, if any
    pub chr_ram_size: RamSize,

    /// The Cartridge to use to emulate cartridge hardware
    pub cartridge: nes::Cartridge,

    /// The version of the ROM
    pub version: Version,

    /// Indicates if Vertical Arrangement should be used
    pub vertical_arrangement: bool,

    /// Indicates if a 4-screen VRAM should be used
    pub four_screen_vram: bool,

    /// Indicates if the SRAM is battery backed
    pub sram_battery_backed: bool,

    /// Indicates if the SRAM is present
    pub sram_present: bool,

    /// Indicates if a trainer is present
    pub trainer_present: bool,

    /// Indicates if this ROM was designed for the Vs. Unisystem
    pub vs_unisystem: bool,

    /// Indicates if this ROM was designed for the PlayChoice-10
    pub playchoice_10: bool,

    /// Indicates the TV system that this ROM was designed for
    pub tv_system: TvSystem,
}

/// Represents an NES ROM, loaded from the iNES/NES2.0 format
pub struct Rom {
    /// Contains the information read from the ROM header
    pub header: Header,

    /// Contains each of the 16KB PRG ROM Banks contained in the ROM file
    pub prg_banks: Vec<Vec<u8>>,

    /// Contains each of the 8KB CHR ROM Banks contained in the ROM file
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

/// Reads an iNES/NES2.0 Rom file from the provided reader
/// 
/// # Arguments
/// * `input` - The `std::io::Read` instance to read the ROM data from
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
        Version::NES2
    } else {
        Version::INES
    };

    // Read ROM sizes 
    let prg_size = match version {
        Version::INES => header[4] as u16,
        Version::NES2 => header[4] as u16 | ((header[9] & 0x0F) as u16)
    };
    let chr_size = match version {
        Version::INES => header[5] as u16,
        Version::NES2 => header[5] as u16 | (((header[9] & 0xF0) >> 4) as u16)
    };


    // Load mapper number
    let mut mapper = ((header[6] & 0xF0) >> 4) as u16;
    let mut submapper : u8 = 0;
    let archaic = header[12..15].iter().all(|i| { *i == 0 });

    // If this isn't Archaic iNES, read the second nybble
    if version == Version::INES || !archaic {
        mapper = (mapper | ((header[7] as u16 & 0xF0))) as u16;
    }

    // If this is NES 2.0, read the third nybble and submapper
    if version == Version::NES2 {
        mapper = (mapper | ((header[8] as u16 & 0x0F) << 8)) as u16;
        submapper = (header[8] & 0xF0) << 4;
    }

    // Read TV System
    let tv_system = match version {
        Version::INES if archaic => TvSystem::Unknown,
        Version::INES => if header[9] & 0x01 == 0 { TvSystem::NTSC } else { TvSystem::PAL },
        Version::NES2 => if header[12] & 0x02 != 0 { 
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
        cartridge: nes::Cartridge::new(mapper, submapper, (header[10] & 0x20) != 0),
        version: version,
        vertical_arrangement: (header[6] & 0x01) == 0,
        four_screen_vram: (header[6] & 0x08) != 0,
        sram_battery_backed: (header[6] & 0x02) != 0,
        sram_present: (header[10] & 0x10) != 0,
        trainer_present: (header[6] & 0x04) != 0,
        vs_unisystem: (header[7] & 0x01) != 0,
        playchoice_10: (header[7] & 0x02) != 0,
        tv_system: tv_system
    })
}

fn verify_signature(sig: &[u8]) -> bool {
    sig.len() == 4 &&
        sig[0] == 0x4E && // 'N'
        sig[1] == 0x45 && // 'E'
        sig[2] == 0x53 && // 'S'
        sig[3] == 0x1A    // EOF
}
