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
pub fn dispatch<M>(inst: Instruction, cpu: &mut Mos6502<M>) -> Result where M: mem::Memory {
    match inst {
        Instruction::ADC(op) => adc::exec(cpu, op),
        Instruction::AHX(op) => store::ahx(cpu, op),
        Instruction::ALR(op) => { try!(and::exec(cpu, op, true)); lsr::exec(cpu, operand::Operand::Accumulator) },
        Instruction::AND(op) => and::exec(cpu, op, false),
        Instruction::ANC(op) => and::exec(cpu, op, true),
        Instruction::ARR(op) => { try!(and::exec(cpu, op, true)); rotate::right(cpu, operand::Operand::Accumulator) },
        Instruction::ASL(op) => asl::exec(cpu, op), 
        Instruction::AXS(op) => axs::exec(cpu, op),
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
        Instruction::DCP(op) => { try!(dec::mem(cpu, op)); compare::exec(cpu, cpu::RegisterName::A, op) },
        Instruction::DEC(op) => dec::mem(cpu, op),
        Instruction::DEX => dec::reg(cpu, cpu::RegisterName::X),
        Instruction::DEY => dec::reg(cpu, cpu::RegisterName::Y),
        Instruction::EOR(op) => eor::exec(cpu, op),
        Instruction::HLT => Err(Error::HaltInstruction),
        Instruction::IGN(op) => { try!(op.get_u8(cpu)); Ok(()) }, // Read the byte to get the side effects
        Instruction::INC(op) => inc::mem(cpu, op),
        Instruction::INX => inc::reg(cpu, cpu::RegisterName::X),
        Instruction::INY => inc::reg(cpu, cpu::RegisterName::Y),
        Instruction::ISC(op) => { try!(inc::mem(cpu, op)); sbc::exec(cpu, op) },
        Instruction::JMP(op) => jmp::exec(cpu, op),
        Instruction::JSR(addr) => jsr::exec(cpu, addr),
        Instruction::LAS(op) => load::las(cpu, op),
        Instruction::LAX(op) => { try!(load::exec(cpu, cpu::RegisterName::A, op)); load::exec(cpu, cpu::RegisterName::X, op) },
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
        Instruction::RLA(op) => { try!(rotate::left(cpu, op)); and::exec(cpu, op, true) },
        Instruction::ROL(op) => rotate::left(cpu, op),
        Instruction::ROR(op) => rotate::right(cpu, op),
        Instruction::RRA(op) => { try!(rotate::right(cpu, op)); adc::exec(cpu, op) },
        Instruction::RTI => ret::from_interrupt(cpu),
        Instruction::RTS => ret::from_sub(cpu),
        Instruction::SAX(op) => store::sax(cpu, op),
        Instruction::SBC(op) => sbc::exec(cpu, op),
        Instruction::SEC => set_flag::exec(cpu, Flags::CARRY()),
        Instruction::SED => set_flag::exec(cpu, Flags::BCD()),
        Instruction::SEI => set_flag::exec(cpu, Flags::INTERRUPT()),
        Instruction::SHY(op) => store::sh(cpu, cpu::RegisterName::X, op),
        Instruction::SHX(op) => store::sh(cpu, cpu::RegisterName::X, op),
        Instruction::SKB(_) => Ok(()),
        Instruction::SLO(op) => { try!(asl::exec(cpu, op)); ora::exec(cpu, op) },
        Instruction::SRE(op) => { try!(lsr::exec(cpu, op)); eor::exec(cpu, op) },
        Instruction::STA(op) => store::exec(cpu, cpu::RegisterName::A, op),
        Instruction::STX(op) => store::exec(cpu, cpu::RegisterName::X, op),
        Instruction::STY(op) => store::exec(cpu, cpu::RegisterName::Y, op),
        Instruction::TAS(op) => store::tas(cpu, op),
        Instruction::TAX => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::X),
        Instruction::TAY => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::Y),
        Instruction::TSX => transfer::exec(cpu, cpu::RegisterName::S, cpu::RegisterName::X),
        Instruction::TXA => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::A),
        Instruction::TXS => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::S),
        Instruction::TYA => transfer::exec(cpu, cpu::RegisterName::Y, cpu::RegisterName::A),
        Instruction::XAA(op) => and::xaa(cpu, op)
    }
}
