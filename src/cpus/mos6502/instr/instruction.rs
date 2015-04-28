use mem;

use instr;

use cpus::mos6502::exec;
use cpus::mos6502::{Mos6502,Operand};

use std::{io,fmt};

/// Represents an instruction that can be executed on a `Mos6502` processor
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum Instruction {
    ADC(Operand),
    AHX(Operand),
    ALR(Operand),
    AND(Operand),
    ANC(Operand),
    ASL(Operand),
    ARR(Operand),
    AXS(Operand),
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
    DCP(Operand),
    DEC(Operand),
    DEX,
    DEY,
    EOR(Operand),
    HLT,
    IGN(Operand),
    INC(Operand),
    INX,
    INY,
    ISC(Operand),
    JMP(Operand),
    JSR(u16),
    LAS(Operand),
    LAX(Operand),
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
    RLA(Operand),
    ROL(Operand),
    ROR(Operand),
    RRA(Operand),
    RTI,
    RTS,
    SAX(Operand),
    SBC(Operand),
    SEC,
    SED,
    SEI,
    SHY(Operand),
    SHX(Operand),
    SKB(Operand),
    SLO(Operand),
    SRE(Operand),
    STA(Operand),
    STX(Operand),
    STY(Operand),
    TAS(Operand),
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    XAA(Operand)
}

impl Instruction {
    /// Executes the instruction against the provided CPU
    ///
    /// # Arguments
    ///
    /// * `cpu` - The process on which to execute the instruction
    pub fn exec<M>(self, cpu: &mut Mos6502, mem: &mut M) -> Result<(), exec::Error> where M: mem::Memory {
        exec::dispatch(self, cpu, mem)
    }
}

impl instr::Instruction for Instruction {
    type DecodeError = super::decoder::Error;
    fn mnemonic(&self) -> &'static str {
        match self {
            &Instruction::ADC(_) => "ADC",
            &Instruction::AHX(_) => "AHX",
            &Instruction::ALR(_) => "ALR",
            &Instruction::ANC(_) => "ANC",
            &Instruction::AND(_) => "AND",
            &Instruction::ARR(_) => "ARR",
            &Instruction::ASL(_) => "ASL",
            &Instruction::AXS(_) => "AXS",
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
            &Instruction::DCP(_) => "DCP",
            &Instruction::DEC(_) => "DEC",
            &Instruction::DEX => "DEX",
            &Instruction::DEY => "DEY",
            &Instruction::EOR(_) => "EOR",
            &Instruction::HLT => "HLT",
            &Instruction::IGN(_) => "IGN",
            &Instruction::INC(_) => "INC",
            &Instruction::INX => "INX",
            &Instruction::INY => "INY",
            &Instruction::ISC(_) => "ISC",
            &Instruction::JMP(_) => "JMP",
            &Instruction::JSR(_) => "JSR",
            &Instruction::LAS(_) => "LAS",
            &Instruction::LAX(_) => "LAX",
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
            &Instruction::RLA(_) => "RLA",
            &Instruction::ROL(_) => "ROL",
            &Instruction::ROR(_) => "ROR",
            &Instruction::RRA(_) => "RRA",
            &Instruction::RTI => "RTI",
            &Instruction::RTS => "RTS",
            &Instruction::SAX(_) => "SAX",
            &Instruction::SBC(_) => "SBC",
            &Instruction::SEC => "SEC",
            &Instruction::SED => "SED",
            &Instruction::SEI => "SEI",
            &Instruction::SHY(_) => "SHY",
            &Instruction::SHX(_) => "SHX",
            &Instruction::SKB(_) => "SKB",
            &Instruction::SLO(_) => "SLO",
            &Instruction::SRE(_) => "SRE",
            &Instruction::STA(_) => "STA",
            &Instruction::STX(_) => "STX",
            &Instruction::STY(_) => "STY",
            &Instruction::TAS(_) => "TAS",
            &Instruction::TAX => "TAX",
            &Instruction::TAY => "TAY",
            &Instruction::TSX => "TSX",
            &Instruction::TXA => "TXA",
            &Instruction::TXS => "TXS",
            &Instruction::TYA => "TYA",
            &Instruction::XAA(_) => "XAA"
        }
    }

    fn decode<R>(reader: R) -> super::decoder::Result<Instruction> where R: io::Read {
        super::decode(reader)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // TODO: Clean this up... I don't like having the trait and struct be the same...
        use instr::Instruction as InstrTrait;

        match self {
            // Instructions with operands
            &Instruction::ADC(op) |
            &Instruction::AHX(op) |
            &Instruction::ALR(op) |
            &Instruction::AND(op) |
            &Instruction::ANC(op) |
            &Instruction::ARR(op) |
            &Instruction::ASL(op) |
            &Instruction::AXS(op) |
            &Instruction::BIT(op) |
            &Instruction::STA(op) |
            &Instruction::STX(op) |
            &Instruction::STY(op) |
            &Instruction::CMP(op) |
            &Instruction::CPX(op) |
            &Instruction::CPY(op) |
            &Instruction::DCP(op) |
            &Instruction::DEC(op) |
            &Instruction::EOR(op) |
            &Instruction::IGN(op) |
            &Instruction::INC(op) |
            &Instruction::ISC(op) |
            &Instruction::JMP(op) |
            &Instruction::LAS(op) |
            &Instruction::LDA(op) |
            &Instruction::LDX(op) |
            &Instruction::LDY(op) |
            &Instruction::LSR(op) |
            &Instruction::ORA(op) |
            &Instruction::RLA(op) |
            &Instruction::ROL(op) |
            &Instruction::ROR(op) |
            &Instruction::RRA(op) |
            &Instruction::SAX(op) |
            &Instruction::SBC(op) |
            &Instruction::SHY(op) |
            &Instruction::SHX(op) |
            &Instruction::SKB(op) |
            &Instruction::SLO(op) |
            &Instruction::SRE(op) |
            &Instruction::TAS(op) |
            &Instruction::XAA(op) => formatter.write_fmt(format_args!("{} {}", self.mnemonic(), op)), 

            // Instructions with signed offsets
            &Instruction::BCC(x) |
            &Instruction::BCS(x) |
            &Instruction::BEQ(x) |
            &Instruction::BMI(x) |
            &Instruction::BNE(x) |
            &Instruction::BVC(x) |
            &Instruction::BVS(x) |
            &Instruction::BPL(x) => formatter.write_fmt(format_args!(
                    "{} {}${:X}",
                    self.mnemonic(),
                    if x < 0 { "-" } else { "+" },
                    x)),

            // Instructions with absolute addresses
            &Instruction::JSR(x) => formatter.write_fmt(format_args!("{} ${:X}", self.mnemonic(), x)), 

            // Instructions with no operands (others)
            _ => formatter.write_str(self.mnemonic())
        }
    }
}
