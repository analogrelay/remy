pub use self::instr::{Instruction,ExecError};
pub use self::operand::{Operand,OperandError};
pub use self::cpu::{Mos6502,Registers,RegisterName,Flags,STACK_START,STACK_END};

/// Defines the instructions that can be executed on the processor
pub mod instr;

mod cpu;
mod operand;

