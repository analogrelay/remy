pub use cpus::mos6502::operand::Operand;
pub use cpus::mos6502::instr::Instruction;
pub use cpus::mos6502::cpu::{Mos6502,Flags,RegisterName};
pub use cpus::mos6502::exec::dispatch;

/// Defines the instructions that can be executed on the processor
pub mod instr;

/// Provides implementations of all the instructions
pub mod exec;

/// Defines the CPU state objects
pub mod cpu;

/// Defines operands that can be provided to instructions
pub mod operand;

/// Indicates the start of the MOS 6502 Stack
const STACK_START   : u64 = 0x0100;

#[cfg(test)]
pub mod tests {
    pub mod clock;
}
