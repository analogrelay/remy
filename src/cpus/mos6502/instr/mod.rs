pub use self::instruction::Instruction;
pub use self::decoder::decode;

/// Code to decode Mos6502 instructions
pub mod decoder;

mod instruction;
