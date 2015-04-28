pub use self::instruction::{Opcode,Instruction};
pub use self::decoder::decode;

/// Code to decode Mos6502 instructions
pub mod decoder;

mod instruction;
