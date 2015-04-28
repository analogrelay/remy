use mem;

use instr;

use cpus::mos6502::exec;
use cpus::mos6502::{Mos6502,Operand};

use std::{convert,fmt,io};

/// Represents an instruction that can be executed on a `Mos6502` processor
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    operand: Option<Operand>,
    bytes: Vec<u8>
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
        self.opcode.into()
    }

    fn decode<R>(reader: R) -> super::decoder::Result<Instruction> where R: io::Read {
        super::decode(reader)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // TODO: Clean this up... I don't like having the trait and struct be the same...
        use instr::Instruction as InstrTrait;

        match self.operand {
            Some(ref op) => format!("{} {}", self.mnemonic(), self.operand.unwrap()),
            None => self.mnemonic().to_string()
        }
    }
}

/// Represents an operation that can be performed on a `Mos6502` processor
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum Opcode {
    ADC,
    AHX,
    ALR,
    AND,
    ANC,
    ASL,
    ARR,
    AXS,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DCP,
    DEC,
    DEX,
    DEY,
    EOR,
    HLT,
    IGN,
    INC,
    INX,
    INY,
    ISC,
    JMP,
    JSR,
    LAS,
    LAX,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    RLA,
    ROL,
    ROR,
    RRA,
    RTI,
    RTS,
    SAX,
    SBC,
    SEC,
    SED,
    SEI,
    SHY,
    SHX,
    SKB,
    SLO,
    SRE,
    STA,
    STX,
    STY,
    TAS,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    XAA
}

impl convert::From<&'static str> for Option<Opcode> {
    fn from(mnemonic: &'static str) -> Option<Opcode> {
        Some(match mnemonic {
            "ADC" => Opcode::ADC,
            "AHX" => Opcode::AHX,
            "ALR" => Opcode::ALR,
            "ANC" => Opcode::ANC,
            "AND" => Opcode::AND,
            "ARR" => Opcode::ARR,
            "ASL" => Opcode::ASL,
            "AXS" => Opcode::AXS,
            "BCC" => Opcode::BCC,
            "BCS" => Opcode::BCS,
            "BEQ" => Opcode::BEQ,
            "BIT" => Opcode::BIT,
            "BMI" => Opcode::BMI,
            "BNE" => Opcode::BNE,
            "BPL" => Opcode::BPL,
            "BRK" => Opcode::BRK,
            "BVC" => Opcode::BVC,
            "BVS" => Opcode::BVS,
            "CLC" => Opcode::CLC,
            "CLD" => Opcode::CLD,
            "CLI" => Opcode::CLI,
            "CLV" => Opcode::CLV,
            "CMP" => Opcode::CMP,
            "CPX" => Opcode::CPX,
            "CPY" => Opcode::CPY,
            "DCP" => Opcode::DCP,
            "DEC" => Opcode::DEC,
            "DEX" => Opcode::DEX,
            "DEY" => Opcode::DEY,
            "EOR" => Opcode::EOR,
            "HLT" => Opcode::HLT,
            "IGN" => Opcode::IGN,
            "INC" => Opcode::INC,
            "INX" => Opcode::INX,
            "INY" => Opcode::INY,
            "ISC" => Opcode::ISC,
            "JMP" => Opcode::JMP,
            "JSR" => Opcode::JSR,
            "LAS" => Opcode::LAS,
            "LAX" => Opcode::LAX,
            "LDA" => Opcode::LDA,
            "LDX" => Opcode::LDX,
            "LDY" => Opcode::LDY,
            "LSR" => Opcode::LSR,
            "NOP" => Opcode::NOP,
            "ORA" => Opcode::ORA,
            "PHA" => Opcode::PHA,
            "PHP" => Opcode::PHP,
            "PLA" => Opcode::PLA,
            "PLP" => Opcode::PLP,
            "RLA" => Opcode::RLA,
            "ROL" => Opcode::ROL,
            "ROR" => Opcode::ROR,
            "RRA" => Opcode::RRA,
            "RTI" => Opcode::RTI,
            "RTS" => Opcode::RTS,
            "SAX" => Opcode::SAX,
            "SBC" => Opcode::SBC,
            "SEC" => Opcode::SEC,
            "SED" => Opcode::SED,
            "SEI" => Opcode::SEI,
            "SHY" => Opcode::SHY,
            "SHX" => Opcode::SHX,
            "SKB" => Opcode::SKB,
            "SLO" => Opcode::SLO,
            "SRE" => Opcode::SRE,
            "STA" => Opcode::STA,
            "STX" => Opcode::STX,
            "STY" => Opcode::STY,
            "TAS" => Opcode::TAS,
            "TAX" => Opcode::TAX,
            "TAY" => Opcode::TAY,
            "TSX" => Opcode::TSX,
            "TXA" => Opcode::TXA,
            "TXS" => Opcode::TXS,
            "TYA" => Opcode::TYA,
            "XAA" => Opcode::XAA,
            _     => return None
        })
    }
}

impl convert::Into<&'static str> for Opcode {
    fn into(self) -> &'static str {
        match self {
            Opcode::ADC => "ADC",
            Opcode::AHX => "AHX",
            Opcode::ALR => "ALR",
            Opcode::ANC => "ANC",
            Opcode::AND => "AND",
            Opcode::ARR => "ARR",
            Opcode::ASL => "ASL",
            Opcode::AXS => "AXS",
            Opcode::BCC => "BCC",
            Opcode::BCS => "BCS",
            Opcode::BEQ => "BEQ",
            Opcode::BIT => "BIT",
            Opcode::BMI => "BMI",
            Opcode::BNE => "BNE",
            Opcode::BPL => "BPL",
            Opcode::BRK => "BRK",
            Opcode::BVC => "BVC",
            Opcode::BVS => "BVS",
            Opcode::CLC => "CLC",
            Opcode::CLD => "CLD",
            Opcode::CLI => "CLI",
            Opcode::CLV => "CLV",
            Opcode::CMP => "CMP",
            Opcode::CPX => "CPX",
            Opcode::CPY => "CPY",
            Opcode::DCP => "DCP",
            Opcode::DEC => "DEC",
            Opcode::DEX => "DEX",
            Opcode::DEY => "DEY",
            Opcode::EOR => "EOR",
            Opcode::HLT => "HLT",
            Opcode::IGN => "IGN",
            Opcode::INC => "INC",
            Opcode::INX => "INX",
            Opcode::INY => "INY",
            Opcode::ISC => "ISC",
            Opcode::JMP => "JMP",
            Opcode::JSR => "JSR",
            Opcode::LAS => "LAS",
            Opcode::LAX => "LAX",
            Opcode::LDA => "LDA",
            Opcode::LDX => "LDX",
            Opcode::LDY => "LDY",
            Opcode::LSR => "LSR",
            Opcode::NOP => "NOP",
            Opcode::ORA => "ORA",
            Opcode::PHA => "PHA",
            Opcode::PHP => "PHP",
            Opcode::PLA => "PLA",
            Opcode::PLP => "PLP",
            Opcode::RLA => "RLA",
            Opcode::ROL => "ROL",
            Opcode::ROR => "ROR",
            Opcode::RRA => "RRA",
            Opcode::RTI => "RTI",
            Opcode::RTS => "RTS",
            Opcode::SAX => "SAX",
            Opcode::SBC => "SBC",
            Opcode::SEC => "SEC",
            Opcode::SED => "SED",
            Opcode::SEI => "SEI",
            Opcode::SHY => "SHY",
            Opcode::SHX => "SHX",
            Opcode::SKB => "SKB",
            Opcode::SLO => "SLO",
            Opcode::SRE => "SRE",
            Opcode::STA => "STA",
            Opcode::STX => "STX",
            Opcode::STY => "STY",
            Opcode::TAS => "TAS",
            Opcode::TAX => "TAX",
            Opcode::TAY => "TAY",
            Opcode::TSX => "TSX",
            Opcode::TXA => "TXA",
            Opcode::TXS => "TXS",
            Opcode::TYA => "TYA",
            Opcode::XAA => "XAA"
        }
    }
}
