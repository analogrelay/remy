use mem;

use instr;

use cpus::mos6502::exec;
use cpus::mos6502::{Mos6502,Operand};

use std::{convert,fmt,io};

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
    BCC(Operand),
    BCS(Operand),
    BEQ(Operand),
    BIT(Operand),
    BMI(Operand),
    BNE(Operand),
    BPL(Operand),
    BVC(Operand),
    BVS(Operand),
    CMP(Operand),
    CPX(Operand),
    CPY(Operand),
    DCP(Operand),
    DEC(Operand),
    EOR(Operand),
    IGN(Operand),
    INC(Operand),
    ISC(Operand),
    JMP(Operand),
    JSR(Operand),
    LAS(Operand),
    LAX(Operand),
    LDA(Operand),
    LDX(Operand),
    LDY(Operand),
    LSR(Operand),
    ORA(Operand),
    RLA(Operand),
    ROL(Operand),
    ROR(Operand),
    RRA(Operand),
    SAX(Operand),
    SBC(Operand),
    SHY(Operand),
    SHX(Operand),
    SKB(Operand),
    SLO(Operand),
    SRE(Operand),
    STA(Operand),
    STX(Operand),
    STY(Operand),
    TAS(Operand),
    XAA(Operand),
    BRK,
    CLC,
    CLD,
    CLI,
    CLV,
    DEX,
    DEY,
    HLT,
    INX,
    INY,
    NOP,
    PHA,
    PHP,
    PLA,
    PLP,
    RTI,
    RTS,
    SEC,
    SED,
    SEI,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
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

    pub fn operand(self) -> Option<Operand> {
        match self {
            // Instructions with operands
            Instruction::ADC(op) |
            Instruction::AHX(op) |
            Instruction::ALR(op) |
            Instruction::AND(op) |
            Instruction::ANC(op) |
            Instruction::ASL(op) |
            Instruction::ARR(op) |
            Instruction::AXS(op) |
            Instruction::BCC(op) |
            Instruction::BCS(op) |
            Instruction::BEQ(op) |
            Instruction::BIT(op) |
            Instruction::BMI(op) |
            Instruction::BNE(op) |
            Instruction::BPL(op) |
            Instruction::BVC(op) |
            Instruction::BVS(op) |
            Instruction::CMP(op) |
            Instruction::CPX(op) |
            Instruction::CPY(op) |
            Instruction::DCP(op) |
            Instruction::DEC(op) |
            Instruction::EOR(op) |
            Instruction::IGN(op) |
            Instruction::INC(op) |
            Instruction::ISC(op) |
            Instruction::JMP(op) |
            Instruction::JSR(op) |
            Instruction::LAS(op) |
            Instruction::LAX(op) |
            Instruction::LDA(op) |
            Instruction::LDX(op) |
            Instruction::LDY(op) |
            Instruction::LSR(op) |
            Instruction::ORA(op) |
            Instruction::RLA(op) |
            Instruction::ROL(op) |
            Instruction::ROR(op) |
            Instruction::RRA(op) |
            Instruction::SAX(op) |
            Instruction::SBC(op) |
            Instruction::SHY(op) |
            Instruction::SHX(op) |
            Instruction::SKB(op) |
            Instruction::SLO(op) |
            Instruction::SRE(op) |
            Instruction::STA(op) |
            Instruction::STX(op) |
            Instruction::STY(op) |
            Instruction::TAS(op) |
            Instruction::XAA(op) => Some(op),

            // Why not use _ here? Because I want to be absolutely sure I'm being exhaustive.
            Instruction::BRK |
            Instruction::CLC |
            Instruction::CLD |
            Instruction::CLI |
            Instruction::CLV |
            Instruction::DEX |
            Instruction::DEY |
            Instruction::HLT |
            Instruction::INX |
            Instruction::INY |
            Instruction::NOP |
            Instruction::PHA |
            Instruction::PHP |
            Instruction::PLA |
            Instruction::PLP |
            Instruction::RTI |
            Instruction::RTS |
            Instruction::SEC |
            Instruction::SED |
            Instruction::SEI |
            Instruction::TAX |
            Instruction::TAY |
            Instruction::TSX |
            Instruction::TXA |
            Instruction::TXS |
            Instruction::TYA => None,
        }
    }

    /// Get a string in the form of the nestest "golden log" output
    pub fn get_log_string<M>(&self, cpu: &Mos6502, mem: &M) -> String where M: mem::Memory {
        use instr::Instruction as InstrTrait;

        format!(
            "{}{}",
            self.mnemonic(),
            match self {
                &Instruction::JMP(op) | &Instruction::JSR(op) => format!(" {}", op),
                _ => match self.operand() {
                    Some(op) => format!(" {}", op.get_log_string(cpu, mem)),
                    None => convert::Into::into("")
                }
            })
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
            &Instruction::BVC(_) => "BVC",
            &Instruction::BVS(_) => "BVS",
            &Instruction::CMP(_) => "CMP",
            &Instruction::CPX(_) => "CPX",
            &Instruction::CPY(_) => "CPY",
            &Instruction::DCP(_) => "DCP",
            &Instruction::DEC(_) => "DEC",
            &Instruction::EOR(_) => "EOR",
            &Instruction::IGN(_) => "IGN",
            &Instruction::INC(_) => "INC",
            &Instruction::ISC(_) => "ISC",
            &Instruction::JMP(_) => "JMP",
            &Instruction::JSR(_) => "JSR",
            &Instruction::LAS(_) => "LAS",
            &Instruction::LAX(_) => "LAX",
            &Instruction::LDA(_) => "LDA",
            &Instruction::LDX(_) => "LDX",
            &Instruction::LDY(_) => "LDY",
            &Instruction::LSR(_) => "LSR",
            &Instruction::ORA(_) => "ORA",
            &Instruction::RLA(_) => "RLA",
            &Instruction::ROL(_) => "ROL",
            &Instruction::ROR(_) => "ROR",
            &Instruction::RRA(_) => "RRA",
            &Instruction::SAX(_) => "SAX",
            &Instruction::SBC(_) => "SBC",
            &Instruction::SHY(_) => "SHY",
            &Instruction::SHX(_) => "SHX",
            &Instruction::SKB(_) => "SKB",
            &Instruction::SLO(_) => "SLO",
            &Instruction::SRE(_) => "SRE",
            &Instruction::STA(_) => "STA",
            &Instruction::STX(_) => "STX",
            &Instruction::STY(_) => "STY",
            &Instruction::TAS(_) => "TAS",
            &Instruction::XAA(_) => "XAA",
            &Instruction::BRK => "BRK",
            &Instruction::CLC => "CLC",
            &Instruction::CLD => "CLD",
            &Instruction::CLI => "CLI",
            &Instruction::CLV => "CLV",
            &Instruction::DEX => "DEX",
            &Instruction::DEY => "DEY",
            &Instruction::HLT => "HLT",
            &Instruction::INX => "INX",
            &Instruction::INY => "INY",
            &Instruction::NOP => "NOP",
            &Instruction::PHA => "PHA",
            &Instruction::PHP => "PHP",
            &Instruction::PLA => "PLA",
            &Instruction::PLP => "PLP",
            &Instruction::RTI => "RTI",
            &Instruction::RTS => "RTS",
            &Instruction::SEC => "SEC",
            &Instruction::SED => "SED",
            &Instruction::SEI => "SEI",
            &Instruction::TAX => "TAX",
            &Instruction::TAY => "TAY",
            &Instruction::TSX => "TSX",
            &Instruction::TXA => "TXA",
            &Instruction::TXS => "TXS",
            &Instruction::TYA => "TYA",
        }
    }

    fn decode<R>(reader: R) -> super::decoder::Result<Instruction> where R: io::Read {
        super::decode(reader)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use instr::Instruction;

        match self.operand() {
            Some(op) => formatter.write_fmt(format_args!("{} {}", self.mnemonic(), op)),
            None => formatter.pad(self.mnemonic())
        }
    }
}
