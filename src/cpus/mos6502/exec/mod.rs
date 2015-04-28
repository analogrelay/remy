use std::{error,fmt};

use mem;

use cpus::mos6502::{cpu,operand,Mos6502,Flags,Instruction,Opcode};

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
mod utils;

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
    match inst.opcode {
        Opcode::ADC => adc::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::AHX => store::ahx(cpu, mem, inst.operand.unwrap()),
        Opcode::ALR => { try!(and::exec(cpu, mem, inst.operand.unwrap(), true)); lsr::exec(cpu, mem, operand::Operand::Accumulator) },
        Opcode::AND => and::exec(cpu, mem, inst.operand.unwrap(), false),
        Opcode::ANC => and::exec(cpu, mem, inst.operand.unwrap(), true),
        Opcode::ARR => { try!(and::exec(cpu, mem, inst.operand.unwrap(), true)); rotate::right(cpu, mem, operand::Operand::Accumulator) },
        Opcode::ASL => asl::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::AXS => axs::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::BCC => branch::if_clear(cpu, inst.operand.unwrap(), Flags::CARRY()),
        Opcode::BCS => branch::if_set(cpu, inst.operand.unwrap(), Flags::CARRY()),
        Opcode::BEQ => branch::if_set(cpu, inst.operand.unwrap(), Flags::ZERO()),
        Opcode::BIT => bit::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::BMI => branch::if_set(cpu, inst.operand.unwrap(), Flags::SIGN()),
        Opcode::BNE => branch::if_clear(cpu, inst.operand.unwrap(), Flags::ZERO()),
        Opcode::BPL => branch::if_clear(cpu, inst.operand.unwrap(), Flags::SIGN()),
        Opcode::BRK => brk::exec(cpu, mem),
        Opcode::BVC => branch::if_clear(cpu, inst.operand.unwrap(), Flags::OVERFLOW()),
        Opcode::BVS => branch::if_set(cpu, inst.operand.unwrap(), Flags::OVERFLOW()),
        Opcode::CLC => clear_flag::exec(cpu, Flags::CARRY()),
        Opcode::CLD => clear_flag::exec(cpu, Flags::BCD()),
        Opcode::CLI => clear_flag::exec(cpu, Flags::INTERRUPT()),
        Opcode::CLV => clear_flag::exec(cpu, Flags::OVERFLOW()),
        Opcode::CMP => compare::exec(cpu, mem, cpu::RegisterName::A, inst.operand.unwrap()),
        Opcode::CPX => compare::exec(cpu, mem, cpu::RegisterName::X, inst.operand.unwrap()),
        Opcode::CPY => compare::exec(cpu, mem, cpu::RegisterName::Y, inst.operand.unwrap()),
        Opcode::DCP => { try!(dec::mem(cpu, mem, inst.operand.unwrap())); compare::exec(cpu, mem, cpu::RegisterName::A, op) },
        Opcode::DEC => dec::mem(cpu, mem, inst.operand.unwrap()),
        Opcode::DEX => dec::reg(cpu, cpu::RegisterName::X),
        Opcode::DEY => dec::reg(cpu, cpu::RegisterName::Y),
        Opcode::EOR => eor::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::HLT => Err(Error::HaltInstruction),
        Opcode::IGN => { try!(inst.operand.unwrap().get_u8(cpu, mem)); Ok(()) }, // Read the byte to get the side effects
        Opcode::INC => inc::mem(cpu, mem, inst.operand.unwrap()),
        Opcode::INX => inc::reg(cpu, cpu::RegisterName::X),
        Opcode::INY => inc::reg(cpu, cpu::RegisterName::Y),
        Opcode::ISC => { try!(inc::mem(cpu, mem, inst.operand.unwrap())); sbc::exec(cpu, mem, op) },
        Opcode::JMP => jmp::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::JSR => jsr::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::LAS => load::las(cpu, mem, inst.operand.unwrap()),
        Opcode::LAX => { 
            try!(load::exec(cpu, mem, cpu::RegisterName::A, inst.operand.unwrap())); 
            load::exec(cpu, mem, cpu::RegisterName::X, inst.operand.unwrap())
        },
        Opcode::LDA => load::exec(cpu, mem, cpu::RegisterName::A, inst.operand.unwrap()),
        Opcode::LDX => load::exec(cpu, mem, cpu::RegisterName::X, inst.operand.unwrap()),
        Opcode::LDY => load::exec(cpu, mem, cpu::RegisterName::Y, inst.operand.unwrap()),
        Opcode::LSR => lsr::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::NOP => Ok(()),
        Opcode::ORA => ora::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::PHA => push::exec(cpu, mem, cpu::RegisterName::A),
        Opcode::PHP => push::exec(cpu, mem, cpu::RegisterName::P),
        Opcode::PLA => pull::exec(cpu, mem, cpu::RegisterName::A),
        Opcode::PLP => pull::exec(cpu, mem, cpu::RegisterName::P),
        Opcode::RLA => { try!(rotate::left(cpu, mem, inst.operand.unwrap())); and::exec(cpu, mem, op, true) },
        Opcode::ROL => rotate::left(cpu, mem, inst.operand.unwrap()),
        Opcode::ROR => rotate::right(cpu, mem, inst.operand.unwrap()),
        Opcode::RRA => { try!(rotate::right(cpu, mem, inst.operand.unwrap())); adc::exec(cpu, mem, op) },
        Opcode::RTI => ret::from_interrupt(cpu, mem),
        Opcode::RTS => ret::from_sub(cpu, mem),
        Opcode::SAX => store::sax(cpu, mem, inst.operand.unwrap()),
        Opcode::SBC => sbc::exec(cpu, mem, inst.operand.unwrap()),
        Opcode::SEC => set_flag::exec(cpu, Flags::CARRY()),
        Opcode::SED => set_flag::exec(cpu, Flags::BCD()),
        Opcode::SEI => set_flag::exec(cpu, Flags::INTERRUPT()),
        Opcode::SHY => store::sh(cpu, mem, cpu::RegisterName::X, inst.operand.unwrap()),
        Opcode::SHX => store::sh(cpu, mem, cpu::RegisterName::X, inst.operand.unwrap()),
        Opcode::SKB => Ok(()),
        Opcode::SLO => { try!(asl::exec(cpu, mem, inst.operand.unwrap())); ora::exec(cpu, mem, op) },
        Opcode::SRE => { try!(lsr::exec(cpu, mem, inst.operand.unwrap())); eor::exec(cpu, mem, op) },
        Opcode::STA => store::exec(cpu, mem, cpu::RegisterName::A, inst.operand.unwrap()),
        Opcode::STX => store::exec(cpu, mem, cpu::RegisterName::X, inst.operand.unwrap()),
        Opcode::STY => store::exec(cpu, mem, cpu::RegisterName::Y, inst.operand.unwrap()),
        Opcode::TAS => store::tas(cpu, mem, inst.operand.unwrap()),
        Opcode::TAX => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::X),
        Opcode::TAY => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::Y),
        Opcode::TSX => transfer::exec(cpu, cpu::RegisterName::S, cpu::RegisterName::X),
        Opcode::TXA => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::A),
        Opcode::TXS => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::S),
        Opcode::TYA => transfer::exec(cpu, cpu::RegisterName::Y, cpu::RegisterName::A),
        Opcode::XAA => and::xaa(cpu, mem, inst.operand.unwrap())
    }
}
