use mem;

use cpus::mos6502::exec;
use cpus::mos6502::{Mos6502,Operand};

use std::fmt;

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

impl Instruction {
    /// Executes the instruction against the provided CPU
    ///
    /// # Arguments
    ///
    /// * `cpu` - The process on which to execute the instruction
	pub fn exec<M>(self, cpu: &mut Mos6502<M>) -> Result<(), exec::Error> where M: mem::Memory {
        exec::dispatch(self, cpu)
    }

    pub fn mnemonic(&self) -> &'static str {
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
		match self {
            // Instructions with operands
			&Instruction::ADC(op) |
            &Instruction::AND(op) |
            &Instruction::ASL(op) |
            &Instruction::BIT(op) |
            &Instruction::STA(op) |
            &Instruction::STX(op) |
            &Instruction::STY(op) |
            &Instruction::CMP(op) |
            &Instruction::CPX(op) |
            &Instruction::CPY(op) |
            &Instruction::DEC(op) |
            &Instruction::EOR(op) |
            &Instruction::INC(op) |
            &Instruction::JMP(op) |
            &Instruction::LDA(op) |
            &Instruction::LDX(op) |
            &Instruction::LDY(op) |
            &Instruction::LSR(op) |
            &Instruction::ORA(op) |
            &Instruction::ROL(op) |
            &Instruction::ROR(op) |
            &Instruction::SBC(op) => formatter.write_fmt(format_args!("{} {}", self.mnemonic(), op)), 

            // Instructions with signed offsets
            &Instruction::BCC(x) |
            &Instruction::BCS(x) |
            &Instruction::BEQ(x) |
            &Instruction::BMI(x) |
            &Instruction::BNE(x) |
            &Instruction::BVC(x) |
            &Instruction::BVS(x) |
            &Instruction::BPL(x) => formatter.write_fmt(format_args!(
                    "{} ${:X}",
                    self.mnemonic(),
                    x)),

            // Instructions with absolute addresses
            &Instruction::JSR(x) => formatter.write_fmt(format_args!("{} ${:X}", self.mnemonic(), x)), 

            // Instructions with no operands (others)
            _ => formatter.write_str(self.mnemonic())
        }
    }
}
