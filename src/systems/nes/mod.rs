pub use self::cart::Cartridge;

/// Contains code to load and manipulate ROMs in the iNES and NES 2.0 formats
pub mod rom; 

/// Contains code to emulate cartridge hardware (Mappers, etc.)
pub mod cart;

