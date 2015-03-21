use std::io;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    kind: ErrorKind
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind: kind
        }
    }
}

pub enum ErrorKind {
    InvalidHeader,
    InvalidSignature
}

pub enum RomVersion {
    ArchaicINES,
    INES,
    NES2
}

pub enum TvSystem {
    NTSC,
    PAL,
    Dual
}

pub struct Header {
    prg_rom_size: u8,
    chr_rom_size: u8,
    prg_ram_size: u8,
    mirroring: MirroringType,
    four_screen_vram: bool,
    sram_battery_backed: bool,
    trainer_present: bool,
    mapper: u8,
    vs_unisystem: bool,
    playchoice_10: bool,
    version: RomVersion,
    tv_system: TvSystem,
    bus_conflicts: bool,
    sram_present: bool
}

const HEADER_SIZE: usize = 16;

pub fn read_header<R>(input: &mut R) -> Result<Header> where R: io::Read {
    // Read the header into memory
    let mut header = [0u8; HEADER_SIZE];
    let read = try!(input.read(&mut sig));
    if read != HEADER_SIZE {
        return Err(Error::new(ErrorKind::InvalidHeader));
    } else if !verify_signature(sig) {
        return Err(Error::new(ErrorKind::InvalidSignature))
    }

    // Read header values
    let prg_size = header[4];
    let chr_size = header[5];
    let flags6 = header[6];
    let flags7 = header[7];
    let prg_ram_size = header[8];
    let flags9 = header[9];
    let flags10 = header[10];

    unimplemented!();
}

fn verify_signature(sig: &[u8]) -> bool {
    sig.len() == 4 &&
        sig[0] == 0x4E && // 'N'
        sig[1] == 0x45 && // 'E'
        sig[2] == 0x53 && // 'S'
        sig[3] == 0x1A    // EOF
}
