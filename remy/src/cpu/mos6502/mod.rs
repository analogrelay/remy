pub use self::instr::{Instruction,ExecError};
pub use self::operand::{Operand,OperandError};
pub use self::cpu::*;

mod cpu;
mod instr;
mod operand;

