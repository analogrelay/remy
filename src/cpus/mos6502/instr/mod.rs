use std::{error,string};

use mem;

use cpus::mos6502::{Mos6502,Operand,OperandError,RegisterName,Flags};

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

/// Represents an instruction that can be executed on a `Mos6502` processor
#[derive(Copy,Debug,Eq,PartialEq)]
pub enum Instruction {
	ADC(Operand),
	AND(Operand),
	ASL(Operand),
    BranchIfClear(i8, Flags),
    BranchIfSet(i8, Flags),
	BIT(Operand),
	BRK,
    ClearFlag(Flags),
    Compare(RegisterName, Operand),
	DEC(Operand),
	EOR(Operand),
	INC(Operand),
	JMP(Operand),
	JSR(u16),
	Load(RegisterName, Operand),
	LSR(Operand),
	NOP,
	ORA(Operand),
    Push(RegisterName),
    Pull(RegisterName),
	ROL(Operand),
	ROR(Operand),
	RTI,
	RTS,
	SBC(Operand),
    SetFlag(Flags),
    Store(RegisterName, Operand),
    Transfer(RegisterName, RegisterName)
}

/// Represents an error that can occur while executing an instruction
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum ExecError {
    /// Indicates that an error occurred retrieving an operand attached to the instruction
	ErrorRetrievingOperand(OperandError),
    /// Indicates that an error occurred reading or writing memory
	ErrorReadingMemory(mem::MemoryError),
    /// Indicates that a provided operand is illegal for use with the executed instruction
    IllegalOperand
}

impl error::FromError<OperandError> for ExecError {
	fn from_error(err: OperandError) -> ExecError {
		ExecError::ErrorRetrievingOperand(err)
	}
}

impl error::FromError<mem::MemoryError> for ExecError {
	fn from_error(err: mem::MemoryError) -> ExecError {
		ExecError::ErrorReadingMemory(err)
	}
}

impl Instruction {
    /// Executes the instruction against the provided CPU
    ///
    /// # Arguments
    ///
    /// * `cpu` - The process on which to execute the instruction
	pub fn exec<M>(self, cpu: &mut Mos6502<M>) -> Result<(), ExecError> where M: mem::Memory {
		match self {
			Instruction::ADC(op) => adc::exec(cpu, op),
			Instruction::AND(op) => and::exec(cpu, op),
			Instruction::ASL(op) => asl::exec(cpu, op), 
            Instruction::BranchIfClear(offset, flag_selector) => branch::if_clear(cpu, offset, flag_selector),
            Instruction::BranchIfSet(offset, flag_selector) => branch::if_set(cpu, offset, flag_selector),
			Instruction::BIT(op) => bit::exec(cpu, op), 
			Instruction::BRK => brk::exec(cpu), 
            Instruction::ClearFlag(flag_selector) => clear_flag::exec(cpu, flag_selector),
            Instruction::Compare(reg, op) => compare::exec(cpu, reg, op),
            Instruction::DEC(op) => dec::exec(cpu, op),
            Instruction::EOR(op) => eor::exec(cpu, op),
            Instruction::INC(op) => inc::exec(cpu, op),
            Instruction::JMP(op) => jmp::exec(cpu, op),
            Instruction::JSR(addr) => jsr::exec(cpu, addr),
            Instruction::Load(reg, op) => load::exec(cpu, reg, op),
            Instruction::LSR(op) => lsr::exec(cpu, op),
            Instruction::NOP => Ok(()),
            Instruction::ORA(op) => ora::exec(cpu, op),
            Instruction::Push(r) => push::exec(cpu, r),
            Instruction::Pull(r) => pull::exec(cpu, r),
            Instruction::ROL(op) => rotate::left(cpu, op),
            Instruction::ROR(op) => rotate::right(cpu, op),
            Instruction::RTI => ret::from_interrupt(cpu),
            Instruction::RTS => ret::from_sub(cpu),
            Instruction::SBC(op) => sbc::exec(cpu, op),
            Instruction::SetFlag(flag_selector) => set_flag::exec(cpu, flag_selector),
            Instruction::Store(reg, op) => store::exec(cpu, reg, op),
            Instruction::Transfer(src, dst) => transfer::exec(cpu, src, dst),
		}
	}

    /*pub fn mnemonic(&self) -> &'static str {
        match self {
            Instruction::ADC(_)                                         => "ADC",
            Instruction::AND(_)                                         => "AND",
            Instruction::ASL(_)                                         => "ASL",
            Instruction::BranchIfClear(_, Flags::CARRY())               => "BCC",
            Instruction::BranchIfSet(_, Flags::CARRY())                 => "BCS",
            Instruction::BranchIfSet(_, Flags::ZERO())                  => "BEQ",
            Instruction::BIT(_)                                         => "BIT",
            Instruction::BranchIfSet(_, Flags::SIGN())                  => "BMI",
            Instruction::BranchIfClear(_, Flags::ZERO())                => "BNE",
            Instruction::BranchIfClear(_, Flags::SIGN())                => "BPL",
            Instruction::BRK                                            => "BRK",
            Instruction::BranchIfClear(_, Flags::OVERFLOW())            => "BVC",
            Instruction::BranchIfSet(_, Flags::OVERFLOW())              => "BVS",
            Instruction::ClearFlag(Flags::CARRY())                      => "CLC",
            Instruction::ClearFlag(Flags::BCD())                        => "CLD",
            Instruction::ClearFlag(Flags::INTERRUPT())                  => "CLI",
            Instruction::ClearFlag(Flags::OVERFLOW())                   => "CLV",
            Instruction::Compare(RegisterName::A, _)                    => "CMP",
            Instruction::Compare(RegisterName::X, _)                    => "CPX",
            Instruction::Compare(RegisterName::Y, _)                    => "CPY",
            Instruction::DEC(Operand::Register(RegisterName::X))        => "DEX",
            Instruction::DEC(Operand::Register(RegisterName::Y))        => "DEY",
            Instruction::DEC(_)                                         => "DEC",
            Instruction::EOR(_)                                         => "EOR",
            Instruction::INC(Operand::Register(RegisterName::X))        => "INX",
            Instruction::INC(Operand::Register(RegisterName::Y))        => "INY",
            Instruction::INC(_)                                         => "INC",
            Instruction::JMP(_)                                         => "JMP",
            Instruction::JSR(_)                                         => "JSR",
            Instruction::Load(RegisterName::A, _)                       => "LDA",
            Instruction::Load(RegisterName::X, _)                       => "LDX",
            Instruction::Load(RegisterName::Y, _)                       => "LDY",
            Instruction::LSR(_)                                         => "LSR",
            Instruction::NOP                                            => "NOP",
            Instruction::ORA(_)                                         => "ORA",
            Instruction::Push(RegisterName::A)                          => "PHA",
            Instruction::Push(RegisterName::P)                          => "PHP",
            Instruction::Pull(RegisterName::A)                          => "PLA",
            Instruction::Pull(RegisterName::P)                          => "PLP",
            Instruction::ROL(_)                                         => "ROL",
            Instruction::ROR(_)                                         => "ROR",
            Instruction::RTI                                            => "RTI",
            Instruction::RTS                                            => "RTS",
            Instruction::SBC(_)                                         => "SBC",
            Instruction::SetFlag(Flags::CARRY())                        => "SEC",
            Instruction::SetFlag(Flags::BCD())                          => "SED",
            Instruction::SetFlag(Flags::INTERRUPT())                    => "SEI",
            Instruction::Store(RegisterName::A, _)                      => "STA",
            Instruction::Store(RegisterName::X, _)                      => "STX",
            Instruction::Store(RegisterName::Y, _)                      => "STY",
            Instruction::Transfer(RegisterName::A, RegisterName::X)     => "TAX",
            Instruction::Transfer(RegisterName::A, RegisterName::Y)     => "TAY",
            Instruction::Transfer(RegisterName::S, RegisterName::X)     => "TSX",
            Instruction::Transfer(RegisterName::X, RegisterName::A)     => "TXA",
            Instruction::Transfer(RegisterName::X, RegisterName::S)     => "TXS",
            Instruction::Transfer(RegisterName::Y, RegisterName::A)     => "TYA",
            _                                                           => "UNKNOWN"
        }
    }*/
}

impl string::ToString for Instruction {
    /// Returns a string representing the instruction
    fn to_string(&self) -> String {
        unimplemented!()
    }
}
