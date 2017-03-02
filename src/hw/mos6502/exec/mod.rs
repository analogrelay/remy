use slog;
use std::{error,fmt};

use mem;

use hw::mos6502::{cpu,operand,Mos6502,Flags,Instruction};

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
#[derive(Clone,PartialEq,Eq,Debug)]
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

serialize_via_debug!(Error);

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
pub fn dispatch<M>(inst: Instruction, cpu: &mut Mos6502, mem: &mut M, logger: Option<slog::Logger>) -> Result where M: mem::Memory {
    let log = unwrap_logger!(logger).new(o!(
        "inst" => inst
    ));

    trace!(log, "executing"; "cpu" => cpu);

    // Tick the base cycle count of the instruction
    cpu.clock.tick(inst.base_cycles());
    let result = match inst {
        Instruction::ADC(op) => adc::exec(cpu, mem, op, &log),
        Instruction::AHX(op) => store::ahx(cpu, mem, op, &log),
        Instruction::ALR(op) => { try_log!(and::exec(cpu, mem, op, true, &log), log); lsr::exec(cpu, mem, operand::Operand::Accumulator, &log) },
        Instruction::AND(op) => and::exec(cpu, mem, op, false, &log),
        Instruction::ANC(op) => and::exec(cpu, mem, op, true, &log),
        Instruction::ARR(op) => {
            try_log!(and::exec(cpu, mem, op, true, &log), log);
            try_log!(rotate::right(cpu, mem, operand::Operand::Accumulator, &log), log);

            // Correct the flags
            let val = try_log!(op.get_u8(cpu, mem), log);
            let bit6 = val & 0x40 != 0;
            let bit5 = val & 0x20 != 0;
            cpu.flags.set_if(Flags::CARRY(), bit6);
            cpu.flags.set_if(Flags::OVERFLOW(), bit6 ^ bit5);
            Ok(())
        },
        Instruction::ASL(op) => asl::exec(cpu, mem, op, &log),
        Instruction::AXS(op) => axs::exec(cpu, mem, op, &log),
        Instruction::BCC(op) => branch::if_clear(cpu, op, Flags::CARRY(), &log),
        Instruction::BCS(op) => branch::if_set(cpu, op, Flags::CARRY(), &log),
        Instruction::BEQ(op) => branch::if_set(cpu, op, Flags::ZERO(), &log),
        Instruction::BIT(op) => bit::exec(cpu, mem, op, &log),
        Instruction::BMI(op) => branch::if_set(cpu, op, Flags::SIGN(), &log),
        Instruction::BNE(op) => branch::if_clear(cpu, op, Flags::ZERO(), &log),
        Instruction::BPL(op) => branch::if_clear(cpu, op, Flags::SIGN(), &log),
        Instruction::BVC(op) => branch::if_clear(cpu, op, Flags::OVERFLOW(), &log),
        Instruction::BVS(op) => branch::if_set(cpu, op, Flags::OVERFLOW(), &log),
        Instruction::CMP(op) => compare::exec(cpu, mem, cpu::RegisterName::A, op, &log),
        Instruction::CPX(op) => compare::exec(cpu, mem, cpu::RegisterName::X, op, &log),
        Instruction::CPY(op) => compare::exec(cpu, mem, cpu::RegisterName::Y, op, &log),
        Instruction::DCP(op) => {
            try_log!(dec::mem(cpu, mem, op, &log), log);
            let _x = cpu.clock.suspend();
            compare::exec(cpu, mem, cpu::RegisterName::A, op, &log)
        },
        Instruction::DEC(op) => dec::mem(cpu, mem, op, &log),
        Instruction::EOR(op) => eor::exec(cpu, mem, op, &log),
        Instruction::IGN(op) => { try_log!(op.get_u8(cpu, mem), log); debug!(log, "executing"); Ok(()) }, // Read the byte to get the side effects
        Instruction::INC(op) => inc::mem(cpu, mem, op, &log),
        Instruction::ISB(op) => { 
            try_log!(inc::mem(cpu, mem, op, &log), log);
            let _x = cpu.clock.suspend();
            sbc::exec(cpu, mem, op, &log)
        },
        Instruction::JMP(op) => jmp::exec(cpu, mem, op, &log),
        Instruction::JSR(op) => jsr::exec(cpu, mem, op, &log),
        Instruction::LAS(op) => load::las(cpu, mem, op, &log),
        Instruction::LAX(op) => { try_log!(load::exec(cpu, mem, cpu::RegisterName::A, op, &log), log); transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::X, &log) },
        Instruction::LDA(op) => load::exec(cpu, mem, cpu::RegisterName::A, op, &log),
        Instruction::LDX(op) => load::exec(cpu, mem, cpu::RegisterName::X, op, &log),
        Instruction::LDY(op) => load::exec(cpu, mem, cpu::RegisterName::Y, op, &log),
        Instruction::LSR(op) => lsr::exec(cpu, mem, op, &log),
        Instruction::ORA(op) => ora::exec(cpu, mem, op, &log),
        Instruction::RLA(op) => {
            let _x = cpu.clock.suspend();
            try_log!(rotate::left(cpu, mem, op, &log), log);
            and::exec(cpu, mem, op, false, &log)
        },
        Instruction::ROL(op) => rotate::left(cpu, mem, op, &log),
        Instruction::ROR(op) => rotate::right(cpu, mem, op, &log),
        Instruction::RRA(op) => { 
            let _x = cpu.clock.suspend();
            try_log!(rotate::right(cpu, mem, op, &log), log);
            adc::exec(cpu, mem, op, &log)
        },
        Instruction::SAX(op) => store::sax(cpu, mem, op, &log),
        Instruction::SBC(op) | Instruction::SBCX(op) => sbc::exec(cpu, mem, op, &log),
        Instruction::SHY(op) => store::sh(cpu, mem, cpu::RegisterName::X, op, &log),
        Instruction::SHX(op) => store::sh(cpu, mem, cpu::RegisterName::X, op, &log),
        Instruction::SKB(op) => { try_log!(op.get_u8(cpu, mem), log); debug!(log, "executing"); Ok(()) },
        Instruction::SLO(op) => {
            let _x = cpu.clock.suspend();
            try_log!(asl::exec(cpu, mem, op, &log), log);
            ora::exec(cpu, mem, op, &log)
        },
        Instruction::SRE(op) => {
            let _x = cpu.clock.suspend();
            try_log!(lsr::exec(cpu, mem, op, &log), log);
            eor::exec(cpu, mem, op, &log)
        },
        Instruction::STA(op) => store::exec(cpu, mem, cpu::RegisterName::A, op, &log),
        Instruction::STX(op) => store::exec(cpu, mem, cpu::RegisterName::X, op, &log),
        Instruction::STY(op) => store::exec(cpu, mem, cpu::RegisterName::Y, op, &log),
        Instruction::TAS(op) => store::tas(cpu, mem, op, &log),
        Instruction::XAA(op) => and::xaa(cpu, mem, op, &log),
        Instruction::BRK => brk::exec(cpu, mem, &log),
        Instruction::CLC => clear_flag::exec(cpu, Flags::CARRY(), &log),
        Instruction::CLD => clear_flag::exec(cpu, Flags::BCD(), &log),
        Instruction::CLI => clear_flag::exec(cpu, Flags::INTERRUPT(), &log),
        Instruction::CLV => clear_flag::exec(cpu, Flags::OVERFLOW(), &log),
        Instruction::DEX => dec::reg(cpu, cpu::RegisterName::X, &log),
        Instruction::DEY => dec::reg(cpu, cpu::RegisterName::Y, &log),
        Instruction::HLT => Err(Error::HaltInstruction),
        Instruction::INX => inc::reg(cpu, cpu::RegisterName::X, &log),
        Instruction::INY => inc::reg(cpu, cpu::RegisterName::Y, &log),
        Instruction::NOP | Instruction::NOPX => Ok(()),
        Instruction::PHA => push::exec(cpu, mem, cpu::RegisterName::A, &log),
        Instruction::PHP => push::exec(cpu, mem, cpu::RegisterName::P, &log),
        Instruction::PLA => pull::exec(cpu, mem, cpu::RegisterName::A, &log),
        Instruction::PLP => pull::exec(cpu, mem, cpu::RegisterName::P, &log),
        Instruction::RTI => ret::from_interrupt(cpu, mem, &log),
        Instruction::RTS => ret::from_sub(cpu, mem, &log),
        Instruction::SEC => set_flag::exec(cpu, Flags::CARRY(), &log),
        Instruction::SED => set_flag::exec(cpu, Flags::BCD(), &log),
        Instruction::SEI => set_flag::exec(cpu, Flags::INTERRUPT(), &log),
        Instruction::TAX => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::X, &log),
        Instruction::TAY => transfer::exec(cpu, cpu::RegisterName::A, cpu::RegisterName::Y, &log),
        Instruction::TSX => transfer::exec(cpu, cpu::RegisterName::S, cpu::RegisterName::X, &log),
        Instruction::TXA => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::A, &log),
        Instruction::TXS => transfer::exec(cpu, cpu::RegisterName::X, cpu::RegisterName::S, &log),
        Instruction::TYA => transfer::exec(cpu, cpu::RegisterName::Y, cpu::RegisterName::A, &log),
    };

    debug!(log, "executed");

    result
}
