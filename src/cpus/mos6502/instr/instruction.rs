use mem::{self,MemoryExt};

use instr;

use cpus::mos6502::exec;
use cpus::mos6502::{Mos6502,Operand,operand};

use std::{convert,fmt,io};
use byteorder::LittleEndian;

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
    ISB(Operand),
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
    SBCX(Operand),
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
    NOP, NOPX,
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

    /// Get the base number of cycles, EXCLUDING additional cycles cause by "oops cycles" (where
    /// indexed memory accesses hopped a page) and cycles lost during branching and jumping
    pub fn base_cycles(&self) -> u64 {
        match self {
            &Instruction::ADC(Operand::Immediate(_)) => 2,
            &Instruction::ADC(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::ADC(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::ADC(Operand::Absolute(_)) => 4,
            &Instruction::ADC(Operand::Indexed(..)) => 4,
            &Instruction::ADC(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::ADC(Operand::PostIndexedIndirect(_)) => 5,

            &Instruction::AHX(Operand::Absolute(_)) => 5,
            &Instruction::AHX(Operand::PostIndexedIndirect(_)) => 6,

            &Instruction::ALR(..) => 2,

            &Instruction::AND(Operand::Immediate(_)) => 2,
            &Instruction::AND(Operand::Absolute(addr)) if addr < 0x0100 => 2,
            &Instruction::AND(Operand::Indexed(addr, _)) if addr < 0x0100 => 3,
            &Instruction::AND(Operand::Absolute(_)) => 4,
            &Instruction::AND(Operand::Indexed(..)) => 4,
            &Instruction::AND(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::AND(Operand::PostIndexedIndirect(_)) => 5,

            &Instruction::ANC(..) => 2,

            &Instruction::ASL(Operand::Accumulator) => 2,
            &Instruction::ASL(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::ASL(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::ASL(Operand::Absolute(_)) => 6,
            &Instruction::ASL(Operand::Indexed(..)) => 7,

            &Instruction::ARR(..) => 2,
            &Instruction::AXS(..) => 2,

            &Instruction::BCC(..) => 2,
            &Instruction::BCS(..) => 2,
            &Instruction::BEQ(..) => 2,

            &Instruction::BIT(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::BIT(Operand::Absolute(_)) => 4,

            &Instruction::BMI(..) => 2,
            &Instruction::BNE(..) => 2,
            &Instruction::BPL(..) => 2,

            &Instruction::BVC(..) => 2,
            &Instruction::BVS(..) => 2,

            &Instruction::CMP(Operand::Immediate(_)) => 2,
            &Instruction::CMP(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::CMP(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::CMP(Operand::Absolute(_)) => 4,
            &Instruction::CMP(Operand::Indexed(..)) => 4,
            &Instruction::CMP(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::CMP(Operand::PostIndexedIndirect(_)) => 5,

            &Instruction::CPX(Operand::Immediate(_)) => 2,
            &Instruction::CPX(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::CPX(Operand::Absolute(_)) => 4,

            &Instruction::CPY(Operand::Immediate(_)) => 2,
            &Instruction::CPY(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::CPY(Operand::Absolute(_)) => 4,

            &Instruction::DCP(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::DCP(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::DCP(Operand::Absolute(_)) => 6,
            &Instruction::DCP(Operand::Indexed(..)) => 7,
            &Instruction::DCP(Operand::PreIndexedIndirect(_)) => 8,
            &Instruction::DCP(Operand::PostIndexedIndirect(_)) => 8,

            &Instruction::DEC(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::DEC(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::DEC(Operand::Absolute(_)) => 6,
            &Instruction::DEC(Operand::Indexed(..)) => 7,

            &Instruction::EOR(Operand::Immediate(_)) => 2,
            &Instruction::EOR(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::EOR(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::EOR(Operand::Absolute(_)) => 4,
            &Instruction::EOR(Operand::Indexed(..)) => 4,
            &Instruction::EOR(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::EOR(Operand::PostIndexedIndirect(_)) => 5,

            &Instruction::IGN(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::IGN(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::IGN(Operand::Absolute(_)) => 4,
            &Instruction::IGN(Operand::Indexed(..)) => 4,

            &Instruction::INC(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::INC(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::INC(Operand::Absolute(_)) => 6,
            &Instruction::INC(Operand::Indexed(..)) => 7,

            &Instruction::ISB(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::ISB(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::ISB(Operand::Absolute(_)) => 6,
            &Instruction::ISB(Operand::Indexed(..)) => 7,
            &Instruction::ISB(Operand::PreIndexedIndirect(_)) => 8,
            &Instruction::ISB(Operand::PostIndexedIndirect(_)) => 8,

            &Instruction::JMP(Operand::Absolute(_)) => 3,
            &Instruction::JMP(..) => 5,

            &Instruction::JSR(..) => 6,
            &Instruction::LAS(..) => 4,

            &Instruction::LAX(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::LAX(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::LAX(Operand::Absolute(_)) => 4,
            &Instruction::LAX(Operand::Indexed(..)) => 4,
            &Instruction::LAX(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::LAX(Operand::PostIndexedIndirect(_)) => 5,

            &Instruction::LDA(Operand::Immediate(_)) => 2,
            &Instruction::LDA(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::LDA(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::LDA(Operand::Absolute(_)) => 4,
            &Instruction::LDA(Operand::Indexed(..)) => 4,
            &Instruction::LDA(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::LDA(Operand::PostIndexedIndirect(_)) => 5,

            &Instruction::LDX(Operand::Immediate(_)) => 2,
            &Instruction::LDX(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::LDX(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::LDX(Operand::Absolute(_)) => 4,
            &Instruction::LDX(Operand::Indexed(..)) => 4,

            &Instruction::LDY(Operand::Immediate(_)) => 2,
            &Instruction::LDY(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::LDY(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::LDY(Operand::Absolute(_)) => 4,
            &Instruction::LDY(Operand::Indexed(..)) => 4,

            &Instruction::LSR(Operand::Accumulator) => 2,
            &Instruction::LSR(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::LSR(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::LSR(Operand::Absolute(_)) => 6,
            &Instruction::LSR(Operand::Indexed(..)) => 7,

            &Instruction::ORA(Operand::Immediate(_)) => 2,
            &Instruction::ORA(Operand::Absolute(addr)) if addr < 0x0100 => 2,
            &Instruction::ORA(Operand::Indexed(addr, _)) if addr < 0x0100 => 3,
            &Instruction::ORA(Operand::Absolute(_)) => 4,
            &Instruction::ORA(Operand::Indexed(..)) => 4,
            &Instruction::ORA(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::ORA(Operand::PostIndexedIndirect(_)) => 5,

            &Instruction::RLA(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::RLA(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::RLA(Operand::Absolute(_)) => 6,
            &Instruction::RLA(Operand::Indexed(..)) => 7,
            &Instruction::RLA(Operand::PreIndexedIndirect(_)) => 8,
            &Instruction::RLA(Operand::PostIndexedIndirect(_)) => 8,

            &Instruction::ROL(Operand::Accumulator) => 2,
            &Instruction::ROL(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::ROL(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::ROL(Operand::Absolute(_)) => 6,
            &Instruction::ROL(Operand::Indexed(..)) => 7,

            &Instruction::ROR(Operand::Accumulator) => 2,
            &Instruction::ROR(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::ROR(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::ROR(Operand::Absolute(_)) => 6,
            &Instruction::ROR(Operand::Indexed(..)) => 7,

            &Instruction::RRA(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::RRA(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::RRA(Operand::Absolute(_)) => 6,
            &Instruction::RRA(Operand::Indexed(..)) => 7,
            &Instruction::RRA(Operand::PreIndexedIndirect(_)) => 8,
            &Instruction::RRA(Operand::PostIndexedIndirect(_)) => 8,

            &Instruction::SAX(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::SAX(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::SAX(Operand::Absolute(_)) => 4,
            &Instruction::SAX(Operand::PreIndexedIndirect(_)) => 6,

            &Instruction::SBC(Operand::Immediate(_)) => 2,
            &Instruction::SBC(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::SBC(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::SBC(Operand::Absolute(_)) => 4,
            &Instruction::SBC(Operand::Indexed(..)) => 4,
            &Instruction::SBC(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::SBC(Operand::PostIndexedIndirect(_)) => 5,
            &Instruction::SBCX(Operand::Immediate(_)) => 2,
            &Instruction::SBCX(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::SBCX(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::SBCX(Operand::Absolute(_)) => 4,
            &Instruction::SBCX(Operand::Indexed(..)) => 4,
            &Instruction::SBCX(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::SBCX(Operand::PostIndexedIndirect(_)) => 5,

            &Instruction::SHY(..) => 5,
            &Instruction::SHX(..) => 5,
            &Instruction::SKB(..) => 2,

            &Instruction::SLO(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::SLO(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::SLO(Operand::Absolute(_)) => 6,
            &Instruction::SLO(Operand::Indexed(..)) => 7,
            &Instruction::SLO(Operand::PreIndexedIndirect(_)) => 8,
            &Instruction::SLO(Operand::PostIndexedIndirect(_)) => 8,

            &Instruction::SRE(Operand::Absolute(addr)) if addr < 0x0100 => 5,
            &Instruction::SRE(Operand::Indexed(addr, _)) if addr < 0x0100 => 6,
            &Instruction::SRE(Operand::Absolute(_)) => 6,
            &Instruction::SRE(Operand::Indexed(..)) => 7,
            &Instruction::SRE(Operand::PreIndexedIndirect(_)) => 8,
            &Instruction::SRE(Operand::PostIndexedIndirect(_)) => 8,

            &Instruction::STA(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::STA(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::STA(Operand::Absolute(_)) => 4,
            &Instruction::STA(Operand::Indexed(..)) => 5,
            &Instruction::STA(Operand::PreIndexedIndirect(_)) => 6,
            &Instruction::STA(Operand::PostIndexedIndirect(_)) => 6,

            &Instruction::STX(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::STX(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::STX(Operand::Absolute(_)) => 4,

            &Instruction::STY(Operand::Absolute(addr)) if addr < 0x0100 => 3,
            &Instruction::STY(Operand::Indexed(addr, _)) if addr < 0x0100 => 4,
            &Instruction::STY(Operand::Absolute(_)) => 4,

            &Instruction::TAS(..) => 5,
            &Instruction::XAA(..) => 2,

            &Instruction::BRK => 7,
            &Instruction::CLC => 2,
            &Instruction::CLD => 2,
            &Instruction::CLI => 2,
            &Instruction::CLV => 2,
            &Instruction::DEX => 2,
            &Instruction::DEY => 2,
            &Instruction::INX => 2,
            &Instruction::INY => 2,
            &Instruction::NOP | &Instruction::NOPX => 2,
            &Instruction::PHA => 3,
            &Instruction::PHP => 3,
            &Instruction::PLA => 4,
            &Instruction::PLP => 4,
            &Instruction::RTI => 6,
            &Instruction::RTS => 6,
            &Instruction::SEC => 2,
            &Instruction::SED => 2,
            &Instruction::SEI => 2,
            &Instruction::TAX => 2,
            &Instruction::TAY => 2,
            &Instruction::TSX => 2,
            &Instruction::TXA => 2,
            &Instruction::TXS => 2,
            &Instruction::TYA => 2,

            s => panic!("Base cycle count for {:?} unknown", s)
        }
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
            Instruction::ISB(op) |
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
            Instruction::SBC(op) | Instruction::SBCX(op) |
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
            Instruction::NOP | Instruction::NOPX |
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

    pub fn undocumented(&self) -> bool {
        match self {
            &Instruction::ALR(_) |
            &Instruction::ANC(_) |
            &Instruction::ARR(_) |
            &Instruction::AXS(_) |
            &Instruction::LAX(_) |
            &Instruction::SAX(_) |
            &Instruction::DCP(_) |
            &Instruction::ISB(_) |
            &Instruction::RLA(_) |
            &Instruction::RRA(_) |
            &Instruction::SLO(_) |
            &Instruction::SRE(_) |
            &Instruction::IGN(_) |
            &Instruction::SKB(_) |
            &Instruction::SBCX(_) |
            &Instruction::NOPX => true,

            _ => false
        }
    }

    /// Get a string in the form of the nestest "golden log" output
    pub fn get_log_string<M>(&self, cpu: &Mos6502, mem: &M) -> operand::Result<String> where M: mem::Memory {
        use instr::Instruction as InstrTrait;

        Ok(format!(
            "{}{}",
            match self {
                i if i.undocumented() => format!("*{}", self.mnemonic()),
                _ => format!(" {}", self.mnemonic())
            },
            match self {
                &Instruction::JMP(op) |
                    &Instruction::JSR(op) =>
                        match op {
                            // Technically this isn't the way the indirect address is calculated,
                            // but it is now nestest.log displays it
                            Operand::Indirect(addr) => format!(" {} = {:04X}", op, try!(mem.get_u16::<LittleEndian>(addr as u64))),
                            _                       => format!(" {}", op)
                        },
                _ => match self.operand() {
                    Some(op) => {
                        format!(" {}", try!(op.get_log_string(cpu, mem)))
                    },
                    None => convert::Into::into("")
                }
            }))
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
            &Instruction::IGN(_) => "NOP",
            &Instruction::INC(_) => "INC",
            &Instruction::ISB(_) => "ISB",
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
            &Instruction::SBC(_) | &Instruction::SBCX(_) => "SBC",
            &Instruction::SHY(_) => "SHY",
            &Instruction::SHX(_) => "SHX",
            &Instruction::SKB(_) => "NOP",
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
            &Instruction::NOP | &Instruction::NOPX => "NOP",
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
