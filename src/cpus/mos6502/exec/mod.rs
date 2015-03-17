use std::error;

use mem;

use cpus::mos6502::{cpu,operand,Mos6502,Flags,Instruction};

mod adc;
mod and;
mod asl;
mod bit;
mod branch;
mod brk;
mod clear_flag;
mod compare;
mod dec;
mod eor;
mod inc;
mod jmp;
mod jsr;
mod load;
mod lsr;
mod ora;
mod push;
mod pull;
mod rotate;
mod ret;
mod sbc;
mod set_flag;
mod store;
mod transfer;

mod utils {
    pub fn bcd_to_int(bcd: isize) -> isize {
        (((bcd & 0xF0) >> 4) * 10) + (bcd & 0x0F)
    }

    pub fn int_to_bcd(int: isize) -> isize {
        let mut v = if int > 99 {
            int - 100
        } else {
            int
        };
        if v > 99 || v < -99 {
            panic!("bcd overflow!");
        }
        if v < 0 {
            // Wrap around
            v = v + 100;
        }
        let h = (v / 10) as u8;
        let l = (v % 10) as u8;
        
        ((h << 4) | l) as isize
    }
}

/// Represents an error that can occur while executing an instruction
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Error {
    /// Indicates that an error occurred retrieving an operand attached to the instruction
	ErrorRetrievingOperand(operand::Error),
    /// Indicates that an error occurred reading or writing memory
	ErrorReadingMemory(mem::MemoryError),
    /// Indicates that a provided operand is illegal for use with the executed instruction
    IllegalOperand
}

impl error::FromError<operand::Error> for Error {
	fn from_error(err: operand::Error) -> Error {
		Error::ErrorRetrievingOperand(err)
	}
}

impl error::FromError<mem::MemoryError> for Error {
	fn from_error(err: mem::MemoryError) -> Error {
		Error::ErrorReadingMemory(err)
	}
}

/// Executes the instruction against the provided CPU
///
/// # Arguments
///
/// * `inst` - The instruction to execute
/// * `cpu` - The process on which to execute the instruction
pub fn dispatch<M>(inst: Instruction, cpu: &mut Mos6502<M>) -> Result<(), Error> where M: mem::Memory {
    match inst {
        Instruction::ADC(op) => adc::exec(cpu, op),
        Instruction::AND(op) => and::exec(cpu, op),
        Instruction::ASL(op) => asl::exec(cpu, op), 
        Instruction::BCC(offset) => branch::if_clear(cpu, offset, Flags::CARRY()),
        Instruction::BCS(offset) => branch::if_set(cpu, offset, Flags::CARRY()),
        Instruction::BEQ(offset) => branch::if_set(cpu, offset, Flags::ZERO()),
        Instruction::BIT(op) => bit::exec(cpu, op), 
        Instruction::BMI(offset) => branch::if_set(cpu, offset, Flags::SIGN()),
        Instruction::BNE(offset) => branch::if_clear(cpu, offset, Flags::ZERO()),
        Instruction::BPL(offset) => branch::if_clear(cpu, offset, Flags::SIGN()),
        Instruction::BRK => brk::exec(cpu), 
        Instruction::BVC(offset) => branch::if_clear(cpu, offset, Flags::OVERFLOW()),
        Instruction::BVS(offset) => branch::if_set(cpu, offset, Flags::OVERFLOW()),
        Instruction::CLC => clear_flag::exec(cpu, Flags::CARRY()),
        Instruction::CLD => clear_flag::exec(cpu, Flags::BCD()),
        Instruction::CLI => clear_flag::exec(cpu, Flags::INTERRUPT()),
        Instruction::CLV => clear_flag::exec(cpu, Flags::OVERFLOW()),
        Instruction::CMP(op) => compare::exec(cpu, cpu::RegisterName::A, op),
        Instruction::CPX(op) => compare::exec(cpu, cpu::RegisterName::X, op),
        Instruction::CPY(op) => compare::exec(cpu, cpu::RegisterName::Y, op),
        Instruction::DEC(op) => dec::mem(cpu, op),
        Instruction::DEX => dec::reg(cpu, cpu::RegisterName::X),
        Instruction::DEY => dec::reg(cpu, cpu::RegisterName::Y),
        Instruction::EOR(op) => eor::exec(cpu, op),
        Instruction::INC(op) => inc::mem(cpu, op),
        Instruction::INX => inc::reg(cpu, cpu::RegisterName::X),
        Instruction::INY => inc::reg(cpu, cpu::RegisterName::Y),
        Instruction::JMP(op) => jmp::exec(cpu, op),
        Instruction::JSR(addr) => jsr::exec(cpu, addr),
        Instruction::LDA(op) => load::exec(cpu, cpu::RegisterName::A, op),
        Instruction::LDX(op) => load::exec(cpu, cpu::RegisterName::X, op),
        Instruction::LDY(op) => load::exec(cpu, cpu::RegisterName::Y, op),
        Instruction::LSR(op) => lsr::exec(cpu, op),
        Instruction::NOP => Ok(()),
        Instruction::ORA(op) => ora::exec(cpu, op),
        Instruction::PHA => push::exec(cpu, cpu::RegisterName::A),
        Instruction::PHP => push::exec(cpu, cpu::RegisterName::P),
        Instruction::PLA => pull::exec(cpu, cpu::RegisterName::A),
        Instruction::PLP => pull::exec(cpu, cpu::RegisterName::P),
        Instruction::ROL(op) => rotate::left(cpu, op),
        Instruction::ROR(op) => rotate::right(cpu, op),
        Instruction::RTI => ret::from_interrupt(cpu),
        Instruction::RTS => ret::from_sub(cpu),
        Instruction::SBC(op) => sbc::exec(cpu, op),
        Instruction::SEC => set_flag::exec(cpu, Flags::CARRY()),
        Instruction::SED => set_flag::exec(cpu, Flags::BCD()),
        Instruction::SEI => set_flag::exec(cpu, Flags::INTERRUPT()),
        Instruction::STA(op) => store::exec(cpu, cpu::RegisterName::A, op),
        Instruction::STX(op) => store::exec(cpu, cpu::RegisterName::X, op),
        Instruction::STY(op) => store::exec(cpu, cpu::RegisterName::Y, op),
        Instruction::TAX => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::X),
        Instruction::TAY => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::Y),
        Instruction::TSX => transfer::exec(cpu, cpu::RegisterName::S, cpu::RegisterName::X),
        Instruction::TXA => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::A),
        Instruction::TXS => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::S),
        Instruction::TYA => transfer::exec(cpu, cpu::RegisterName::Y, cpu::RegisterName::A)
    }
}
