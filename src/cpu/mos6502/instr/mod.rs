use std::{error,fmt};

use mem;

use cpu::mos6502::{Mos6502,Operand,OperandError,RegisterName,Flags};

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
    BCC(i8),
    BCS(i8),
    BEQ(i8),
	BIT(Operand),
    BMI(i8),
    BNE(i8),
    BPL(i8),
	BRK,
    BVC(i8),
    BVS(i8),
    CLC,
    CLD,
    CLI,
    CLV,
    CMP(Operand),
    CPX(Operand),
    CPY(Operand),
	DEC(Operand),
    DEX,
    DEY,
	EOR(Operand),
	INC(Operand),
    INX,
    INY,
	JMP(Operand),
	JSR(u16),
    LDA(Operand),
    LDX(Operand),
    LDY(Operand),
	LSR(Operand),
	NOP,
	ORA(Operand),
    PHA,
    PHP,
    PLA,
    PLP,
	ROL(Operand),
	ROR(Operand),
	RTI,
	RTS,
	SBC(Operand),
    SEC,
    SED,
    SEI,
    STA(Operand),
    STX(Operand),
    STY(Operand),
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA
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

instruction_set!(
    ADC(op: Operand) => { adc::exec(cpu, op) },
    AND(op: Operand) => { and::exec(cpu, op) },
    ASL(op: Operand) => { asl::exec(cpu, op) },
    BCC(offset: i8) => { branch::if_clear(cpu, offset, Flags::CARRY()) }
)

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
            Instruction::CMP(op) => compare::exec(cpu, RegisterName::A, op),
            Instruction::CPX(op) => compare::exec(cpu, RegisterName::X, op),
            Instruction::CPY(op) => compare::exec(cpu, RegisterName::Y, op),
            Instruction::DEC(op) => dec::mem(cpu, op),
            Instruction::DEX => dec::reg(cpu, RegisterName::X),
            Instruction::DEY => dec::reg(cpu, RegisterName::Y),
            Instruction::EOR(op) => eor::exec(cpu, op),
            Instruction::INC(op) => inc::mem(cpu, op),
            Instruction::INX => inc::reg(cpu, RegisterName::X),
            Instruction::INY => inc::reg(cpu, RegisterName::Y),
            Instruction::JMP(op) => jmp::exec(cpu, op),
            Instruction::JSR(addr) => jsr::exec(cpu, addr),
            Instruction::LDA(op) => load::exec(cpu, RegisterName::A, op),
            Instruction::LDX(op) => load::exec(cpu, RegisterName::X, op),
            Instruction::LDY(op) => load::exec(cpu, RegisterName::Y, op),
            Instruction::LSR(op) => lsr::exec(cpu, op),
            Instruction::NOP => Ok(()),
            Instruction::ORA(op) => ora::exec(cpu, op),
            Instruction::PHA => push::exec(cpu, RegisterName::A),
            Instruction::PHP => push::exec(cpu, RegisterName::P),
            Instruction::PLA => pull::exec(cpu, RegisterName::A),
            Instruction::PLP => pull::exec(cpu, RegisterName::P),
            Instruction::ROL(op) => rotate::left(cpu, op),
            Instruction::ROR(op) => rotate::right(cpu, op),
            Instruction::RTI => ret::from_interrupt(cpu),
            Instruction::RTS => ret::from_sub(cpu),
            Instruction::SBC(op) => sbc::exec(cpu, op),
            Instruction::SEC => set_flag::exec(cpu, Flags::CARRY()),
            Instruction::SED => set_flag::exec(cpu, Flags::BCD()),
            Instruction::SEI => set_flag::exec(cpu, Flags::INTERRUPT()),
            Instruction::STA(op) => store::exec(cpu, RegisterName::A, op),
            Instruction::STX(op) => store::exec(cpu, RegisterName::X, op),
            Instruction::STY(op) => store::exec(cpu, RegisterName::Y, op),
            Instruction::TAX => transfer::exec(cpu, RegisterName::A, RegisterName::X),
            Instruction::TAY => transfer::exec(cpu, RegisterName::A, RegisterName::Y),
            Instruction::TSX => transfer::exec(cpu, RegisterName::S, RegisterName::X),
            Instruction::TXA => transfer::exec(cpu, RegisterName::X, RegisterName::A),
            Instruction::TXS => transfer::exec(cpu, RegisterName::X, RegisterName::S),
            Instruction::TYA => transfer::exec(cpu, RegisterName::Y, RegisterName::A)
		}
	}

    pub fn mnemonic(&self) -> &'static str {
<<<<<<< master:src/cpu/mos6502/instr/mod.rs
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
    }
}

impl string::ToString for Instruction {
    /// Returns a string representing the instruction
    pub fn to_string(&self) -> String {
=======
		match self {
			&Instruction::ADC(_) => "ADC",
			&Instruction::AND(_) => "AND",
			&Instruction::ASL(_) => "ASL",
            &Instruction::BCC(_) => "BCC",
            &Instruction::BCS(_) => "BCS",
            &Instruction::BEQ(_) => "BEQ",
			&Instruction::BIT(_) => "BIT",
            &Instruction::BMI(_) => "BMI",
            &Instruction::BNE(_) => "BNE",
            &Instruction::BPL(_) => "BPL",
			&Instruction::BRK => "BRK",
            &Instruction::BVC(_) => "BVC",
            &Instruction::BVS(_) => "BVS",
            &Instruction::CLC => "CLC",
            &Instruction::CLD => "CLD",
            &Instruction::CLI => "CLI",
            &Instruction::CLV => "CLV",
            &Instruction::CMP(_) => "CMP",
            &Instruction::CPX(_) => "CPX",
            &Instruction::CPY(_) => "CPY",
            &Instruction::DEC(_) => "DEC",
            &Instruction::DEX => "DEX",
            &Instruction::DEY => "DEY",
            &Instruction::EOR(_) => "EOR",
            &Instruction::INC(_) => "INC",
            &Instruction::INX => "INX",
            &Instruction::INY => "INY",
            &Instruction::JMP(_) => "JMP",
            &Instruction::JSR(_) => "JSR",
            &Instruction::LDA(_) => "LDA",
            &Instruction::LDX(_) => "LDX",
            &Instruction::LDY(_) => "LDY",
            &Instruction::LSR(_) => "LSR",
            &Instruction::NOP => "NOP",
            &Instruction::ORA(_) => "ORA",
            &Instruction::PHA => "PHA",
            &Instruction::PHP => "PHP",
            &Instruction::PLA => "PLA",
            &Instruction::PLP => "PLP",
            &Instruction::ROL(_) => "ROL",
            &Instruction::ROR(_) => "ROR",
            &Instruction::RTI => "RTI",
            &Instruction::RTS => "RTS",
            &Instruction::SBC(_) => "SBC",
            &Instruction::SEC => "SEC",
            &Instruction::SED => "SED",
            &Instruction::SEI => "SEI",
            &Instruction::STA(_) => "STA",
            &Instruction::STX(_) => "STX",
            &Instruction::STY(_) => "STY",
            &Instruction::TAX => "TAX",
            &Instruction::TAY => "TAY",
            &Instruction::TSX => "TSX",
            &Instruction::TXA => "TXA",
            &Instruction::TXS => "TXS",
            &Instruction::TYA => "TYA",
		}
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use std::num::SignedInt;
		match self {
            // Instructions with operands
			i @ &Instruction::ADC(op) |
            i @ &Instruction::AND(op) |
            i @ &Instruction::ASL(op) |
            i @ &Instruction::BIT(op) |
            i @ &Instruction::STA(op) |
            i @ &Instruction::STX(op) |
            i @ &Instruction::STY(op) |
            i @ &Instruction::CMP(op) |
            i @ &Instruction::CPX(op) |
            i @ &Instruction::CPY(op) |
            i @ &Instruction::DEC(op) |
            i @ &Instruction::EOR(op) |
            i @ &Instruction::INC(op) |
            i @ &Instruction::JMP(op) |
            i @ &Instruction::LDA(op) |
            i @ &Instruction::LDX(op) |
            i @ &Instruction::LDY(op) |
            i @ &Instruction::LSR(op) |
            i @ &Instruction::ORA(op) |
            i @ &Instruction::ROL(op) |
            i @ &Instruction::ROR(op) |
            i @ &Instruction::SBC(op) => formatter.write_fmt(format_args!("{} {}", i.mnemonic(), op)), 

            // Instructions with signed offsets
            i @ &Instruction::BCC(x) |
            i @ &Instruction::BCS(x) |
            i @ &Instruction::BEQ(x) |
            i @ &Instruction::BMI(x) |
            i @ &Instruction::BNE(x) |
            i @ &Instruction::BVC(x) |
            i @ &Instruction::BVS(x) |
            i @ &Instruction::BPL(x) => formatter.write_fmt(format_args!(
                    "{} ${:X}",
                    i.mnemonic(),
                    x)),

            // Instructions with absolute addresses
            i @ &Instruction::JSR(x) => formatter.write_fmt(format_args!("{} ${:X}", i.mnemonic(), x)), 

            // Instructions with no operands (others)
            x => formatter.write_str(x.mnemonic())
        }
>>>>>>> local:src/cpus/mos6502/instr/mod.rs
    }
}
