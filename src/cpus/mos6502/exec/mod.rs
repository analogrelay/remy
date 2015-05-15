use std::{error,fmt};

use mem;

use cpus::mos6502::{cpu,operand,Mos6502,Flags,Instruction};

mod adc;
mod and;
mod asl;
mod axs;
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

pub type Result = ::std::result::Result<(), Error>;

/// Represents an error that can occur while executing an instruction
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Error {
    /// Indicates that an error occurred retrieving an operand attached to the instruction
    ErrorRetrievingOperand(operand::Error),
    /// Indicates that an error occurred reading or writing memory
    ErrorReadingMemory(mem::Error),
    /// Indicates that a provided operand is illegal for use with the executed instruction
    IllegalOperand,
    /// Indicates that the HLT instruction was invoked
    HaltInstruction
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::ErrorRetrievingOperand(_) => "error retrieving operand",
            &Error::ErrorReadingMemory(_)     => "error reading from memory",
            &Error::IllegalOperand            => "operand is illegal for use with the executed instruction",
            &Error::HaltInstruction           => "the instruction caused the processor to halt"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::ErrorRetrievingOperand(ref err) => Some(err),
            &Error::ErrorReadingMemory(ref err)     => Some(err),
            _                                       => None
        }
    }
}

impl From<operand::Error> for Error {
    fn from(err: operand::Error) -> Error {
        Error::ErrorRetrievingOperand(err)
    }
}

impl From<mem::Error> for Error {
    fn from(err: mem::Error) -> Error {
        Error::ErrorReadingMemory(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::ErrorRetrievingOperand(ref err) => write!(fmt, "error retrieving operand: {}", err),
            &Error::ErrorReadingMemory(ref err)     => write!(fmt, "error reading from memory: {}", err),
            _                                       => error::Error::description(self).fmt(fmt)
        }
    }
}

/// Executes the instruction against the provided CPU
///
/// # Arguments
///
/// * `inst` - The instruction to execute
/// * `cpu` - The process on which to execute the instruction
pub fn dispatch<M>(inst: Instruction, cpu: &mut Mos6502, mem: &mut M) -> Result where M: mem::Memory {
    // Tick the base cycle count of the instruction
    cpu.clock.tick(inst.base_cycles());
    match inst {
        Instruction::ADC(op) => adc::exec(cpu, mem, op),
        Instruction::AHX(op) => store::ahx(cpu, mem, op),
        Instruction::ALR(op) => { try!(and::exec(cpu, mem, op, true)); lsr::exec(cpu, mem, operand::Operand::Accumulator) },
        Instruction::AND(op) => and::exec(cpu, mem, op, false),
        Instruction::ANC(op) => and::exec(cpu, mem, op, true),
        Instruction::ARR(op) => { try!(and::exec(cpu, mem, op, true)); rotate::right(cpu, mem, operand::Operand::Accumulator) },
        Instruction::ASL(op) => asl::exec(cpu, mem, op),
        Instruction::AXS(op) => axs::exec(cpu, mem, op),
        Instruction::BCC(op) => branch::if_clear(cpu, op, Flags::CARRY()),
        Instruction::BCS(op) => branch::if_set(cpu, op, Flags::CARRY()),
        Instruction::BEQ(op) => branch::if_set(cpu, op, Flags::ZERO()),
        Instruction::BIT(op) => bit::exec(cpu, mem, op),
        Instruction::BMI(op) => branch::if_set(cpu, op, Flags::SIGN()),
        Instruction::BNE(op) => branch::if_clear(cpu, op, Flags::ZERO()),
        Instruction::BPL(op) => branch::if_clear(cpu, op, Flags::SIGN()),
        Instruction::BVC(op) => branch::if_clear(cpu, op, Flags::OVERFLOW()),
        Instruction::BVS(op) => branch::if_set(cpu, op, Flags::OVERFLOW()),
        Instruction::CMP(op) => compare::exec(cpu, mem, cpu::RegisterName::A, op),
        Instruction::CPX(op) => compare::exec(cpu, mem, cpu::RegisterName::X, op),
        Instruction::CPY(op) => compare::exec(cpu, mem, cpu::RegisterName::Y, op),
        Instruction::DCP(op) => {
            try!(dec::mem(cpu, mem, op));
            let _x = cpu.clock.suspend();
            compare::exec(cpu, mem, cpu::RegisterName::A, op)
        },
        Instruction::DEC(op) => dec::mem(cpu, mem, op),
        Instruction::EOR(op) => eor::exec(cpu, mem, op),
        Instruction::IGN(op) => { try!(op.get_u8(cpu, mem)); Ok(()) }, // Read the byte to get the side effects
        Instruction::INC(op) => inc::mem(cpu, mem, op),
        Instruction::ISB(op) => { 
            try!(inc::mem(cpu, mem, op));
            let _x = cpu.clock.suspend();
            sbc::exec(cpu, mem, op)
        },
        Instruction::JMP(op) => jmp::exec(cpu, mem, op),
        Instruction::JSR(op) => jsr::exec(cpu, mem, op),
        Instruction::LAS(op) => load::las(cpu, mem, op),
        Instruction::LAX(op) => { try!(load::exec(cpu, mem, cpu::RegisterName::A, op)); transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::X) },
        Instruction::LDA(op) => load::exec(cpu, mem, cpu::RegisterName::A, op),
        Instruction::LDX(op) => load::exec(cpu, mem, cpu::RegisterName::X, op),
        Instruction::LDY(op) => load::exec(cpu, mem, cpu::RegisterName::Y, op),
        Instruction::LSR(op) => lsr::exec(cpu, mem, op),
        Instruction::ORA(op) => ora::exec(cpu, mem, op),
        Instruction::RLA(op) => {
            let _x = cpu.clock.suspend();
            try!(rotate::left(cpu, mem, op));
            and::exec(cpu, mem, op, false)
        },
        Instruction::ROL(op) => rotate::left(cpu, mem, op),
        Instruction::ROR(op) => rotate::right(cpu, mem, op),
        Instruction::RRA(op) => { 
            let _x = cpu.clock.suspend();
            try!(rotate::right(cpu, mem, op));
            adc::exec(cpu, mem, op)
        },
        Instruction::SAX(op) => store::sax(cpu, mem, op),
        Instruction::SBC(op) | Instruction::SBCX(op) => sbc::exec(cpu, mem, op),
        Instruction::SHY(op) => store::sh(cpu, mem, cpu::RegisterName::X, op),
        Instruction::SHX(op) => store::sh(cpu, mem, cpu::RegisterName::X, op),
        Instruction::SKB(op) => { try!(op.get_u8(cpu, mem)); Ok(()) },
        Instruction::SLO(op) => {
            let _x = cpu.clock.suspend();
            try!(asl::exec(cpu, mem, op));
            ora::exec(cpu, mem, op)
        },
        Instruction::SRE(op) => {
            let _x = cpu.clock.suspend();
            try!(lsr::exec(cpu, mem, op));
            eor::exec(cpu, mem, op)
        },
        Instruction::STA(op) => store::exec(cpu, mem, cpu::RegisterName::A, op),
        Instruction::STX(op) => store::exec(cpu, mem, cpu::RegisterName::X, op),
        Instruction::STY(op) => store::exec(cpu, mem, cpu::RegisterName::Y, op),
        Instruction::TAS(op) => store::tas(cpu, mem, op),
        Instruction::XAA(op) => and::xaa(cpu, mem, op),
        Instruction::BRK => brk::exec(cpu, mem),
        Instruction::CLC => clear_flag::exec(cpu, Flags::CARRY()),
        Instruction::CLD => clear_flag::exec(cpu, Flags::BCD()),
        Instruction::CLI => clear_flag::exec(cpu, Flags::INTERRUPT()),
        Instruction::CLV => clear_flag::exec(cpu, Flags::OVERFLOW()),
        Instruction::DEX => dec::reg(cpu, cpu::RegisterName::X),
        Instruction::DEY => dec::reg(cpu, cpu::RegisterName::Y),
        Instruction::HLT => Err(Error::HaltInstruction),
        Instruction::INX => inc::reg(cpu, cpu::RegisterName::X),
        Instruction::INY => inc::reg(cpu, cpu::RegisterName::Y),
        Instruction::NOP | Instruction::NOPX => Ok(()),
        Instruction::PHA => push::exec(cpu, mem, cpu::RegisterName::A),
        Instruction::PHP => push::exec(cpu, mem, cpu::RegisterName::P),
        Instruction::PLA => pull::exec(cpu, mem, cpu::RegisterName::A),
        Instruction::PLP => pull::exec(cpu, mem, cpu::RegisterName::P),
        Instruction::RTI => ret::from_interrupt(cpu, mem),
        Instruction::RTS => ret::from_sub(cpu, mem),
        Instruction::SEC => set_flag::exec(cpu, Flags::CARRY()),
        Instruction::SED => set_flag::exec(cpu, Flags::BCD()),
        Instruction::SEI => set_flag::exec(cpu, Flags::INTERRUPT()),
        Instruction::TAX => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::X),
        Instruction::TAY => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::Y),
        Instruction::TSX => transfer::exec(cpu, cpu::RegisterName::S, cpu::RegisterName::X),
        Instruction::TXA => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::A),
        Instruction::TXS => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::S),
        Instruction::TYA => transfer::exec(cpu, cpu::RegisterName::Y, cpu::RegisterName::A),
    }
}
