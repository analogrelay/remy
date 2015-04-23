use std::{error,io,fmt};

use cpus::mos6502::{Operand,Instruction,RegisterName};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnknownOpcode(u8),
    EndOfFile,
    IoError(io::Error)
}

impl Error {
    fn from_io(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::UnknownOpcode(_) => "unknown opcode",
            &Error::EndOfFile        => "end of file reached",
            &Error::IoError(_)       => "i/o error"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::IoError(ref err) => Some(err),
            _                        => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::UnknownOpcode(opcode) => write!(fmt, "unknown opcode: 0x{:02X}", opcode),
            &Error::EndOfFile             => write!(fmt, "{}", "end of file reached"),
            &Error::IoError(ref err)      => write!(fmt, "i/o error: {}", err)
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::from_io(err)
    }
}

pub fn decode<R>(mut reader: R) -> Result<Instruction> where R: io::Read {
    // Read the opcode
    let opcode = try!(read_byte(&mut reader));

    // Determine the next step based on the instruction
    let instr = match opcode {
        0x00 => Instruction::BRK,
        0x01 => Instruction::ORA(try!(read_ind_x(&mut reader))),
        0x02 => Instruction::HLT,
        0x03 => Instruction::SLO(try!(read_ind_x(&mut reader))),
        0x04 => Instruction::IGN(try!(read_zp(&mut reader))),
        0x05 => Instruction::ORA(try!(read_zp(&mut reader))),
        0x06 => Instruction::ASL(try!(read_zp(&mut reader))),
        0x07 => Instruction::SLO(try!(read_zp(&mut reader))),
        0x08 => Instruction::PHP,
        0x09 => Instruction::ORA(try!(read_imm(&mut reader))),
        0x0A => Instruction::ASL(Operand::Accumulator),
        0x0B => Instruction::ANC(try!(read_imm(&mut reader))),
        0x0C => Instruction::IGN(try!(read_abs(&mut reader))),
        0x0D => Instruction::ORA(try!(read_abs(&mut reader))),
        0x0E => Instruction::ASL(try!(read_abs(&mut reader))),
        0x0F => Instruction::SLO(try!(read_abs(&mut reader))),

        0x10 => Instruction::BPL(try!(read_byte(&mut reader)) as i8),
        0x11 => Instruction::ORA(try!(read_ind_y(&mut reader))),
        0x12 => Instruction::HLT,
        0x13 => Instruction::SLO(try!(read_ind_y(&mut reader))),
        0x14 => Instruction::IGN(try!(read_zp_x(&mut reader))),
        0x15 => Instruction::ORA(try!(read_zp_x(&mut reader))),
        0x16 => Instruction::ASL(try!(read_zp_x(&mut reader))),
        0x17 => Instruction::SLO(try!(read_zp_x(&mut reader))),
        0x18 => Instruction::CLC,
        0x19 => Instruction::ORA(try!(read_abs_y(&mut reader))),
        0x1A => Instruction::NOP,
        0x1B => Instruction::SLO(try!(read_abs_y(&mut reader))),
        0x1C => Instruction::IGN(try!(read_abs_x(&mut reader))),
        0x1D => Instruction::ORA(try!(read_abs_x(&mut reader))),
        0x1E => Instruction::ASL(try!(read_abs_x(&mut reader))),
        0x1F => Instruction::SLO(try!(read_abs_x(&mut reader))),

        0x20 => Instruction::JSR(try!(read_u16(&mut reader))),
        0x21 => Instruction::AND(try!(read_ind_x(&mut reader))),
        0x22 => Instruction::HLT,
        0x23 => Instruction::RLA(try!(read_ind_x(&mut reader))),
        0x24 => Instruction::BIT(try!(read_zp(&mut reader))),
        0x25 => Instruction::AND(try!(read_zp(&mut reader))),
        0x26 => Instruction::ROL(try!(read_zp(&mut reader))),
        0x27 => Instruction::RLA(try!(read_zp(&mut reader))),
        0x28 => Instruction::PLP,
        0x29 => Instruction::AND(try!(read_imm(&mut reader))),
        0x2A => Instruction::ROL(Operand::Accumulator),
        0x2B => Instruction::ANC(try!(read_imm(&mut reader))),
        0x2C => Instruction::BIT(try!(read_abs(&mut reader))),
        0x2D => Instruction::AND(try!(read_abs(&mut reader))),
        0x2E => Instruction::ROL(try!(read_abs(&mut reader))),
        0x2F => Instruction::RLA(try!(read_abs(&mut reader))),

        0x30 => Instruction::BMI(try!(read_byte(&mut reader)) as i8),
        0x31 => Instruction::AND(try!(read_ind_y(&mut reader))),
        0x32 => Instruction::HLT,
        0x33 => Instruction::RLA(try!(read_ind_y(&mut reader))),
        0x34 => Instruction::IGN(try!(read_zp_x(&mut reader))),
        0x35 => Instruction::AND(try!(read_zp_x(&mut reader))),
        0x36 => Instruction::ROL(try!(read_zp_x(&mut reader))),
        0x37 => Instruction::RLA(try!(read_zp_x(&mut reader))),
        0x38 => Instruction::SEC,
        0x39 => Instruction::AND(try!(read_abs_y(&mut reader))),
        0x3A => Instruction::NOP,
        0x3B => Instruction::RLA(try!(read_abs_y(&mut reader))),
        0x3C => Instruction::IGN(try!(read_abs_x(&mut reader))),
        0x3D => Instruction::AND(try!(read_abs_x(&mut reader))),
        0x3E => Instruction::ROL(try!(read_abs_x(&mut reader))),
        0x3F => Instruction::RLA(try!(read_abs_x(&mut reader))),

        0x40 => Instruction::RTI,
        0x41 => Instruction::EOR(try!(read_ind_x(&mut reader))),
        0x42 => Instruction::HLT,
        0x43 => Instruction::SRE(try!(read_ind_x(&mut reader))),
        0x44 => Instruction::IGN(try!(read_zp(&mut reader))),
        0x45 => Instruction::EOR(try!(read_zp(&mut reader))),
        0x46 => Instruction::LSR(try!(read_zp(&mut reader))),
        0x47 => Instruction::SRE(try!(read_zp(&mut reader))),
        0x48 => Instruction::PHA,
        0x49 => Instruction::EOR(try!(read_imm(&mut reader))),
        0x4A => Instruction::LSR(Operand::Accumulator),
        0x4B => Instruction::ALR(try!(read_imm(&mut reader))),
        0x4C => Instruction::JMP(try!(read_abs(&mut reader))),
        0x4D => Instruction::EOR(try!(read_abs(&mut reader))),
        0x4E => Instruction::LSR(try!(read_abs(&mut reader))),
        0x4F => Instruction::SRE(try!(read_abs(&mut reader))),

        0x50 => Instruction::BVC(try!(read_byte(&mut reader)) as i8),
        0x51 => Instruction::EOR(try!(read_ind_y(&mut reader))),
        0x52 => Instruction::HLT,
        0x53 => Instruction::SRE(try!(read_ind_y(&mut reader))),
        0x54 => Instruction::IGN(try!(read_zp_x(&mut reader))),
        0x55 => Instruction::EOR(try!(read_zp_x(&mut reader))),
        0x56 => Instruction::LSR(try!(read_zp_x(&mut reader))),
        0x57 => Instruction::SRE(try!(read_zp_x(&mut reader))),
        0x58 => Instruction::CLI,
        0x59 => Instruction::EOR(try!(read_abs_y(&mut reader))),
        0x5A => Instruction::NOP,
        0x5B => Instruction::SRE(try!(read_abs_y(&mut reader))),
        0x5C => Instruction::IGN(try!(read_abs_x(&mut reader))),
        0x5D => Instruction::EOR(try!(read_abs_x(&mut reader))),
        0x5E => Instruction::LSR(try!(read_abs_x(&mut reader))),
        0x5F => Instruction::SRE(try!(read_abs_x(&mut reader))),

        0x60 => Instruction::RTS,
        0x61 => Instruction::ADC(try!(read_ind_x(&mut reader))),
        0x62 => Instruction::HLT,
        0x63 => Instruction::RRA(try!(read_ind_x(&mut reader))),
        0x64 => Instruction::IGN(try!(read_zp(&mut reader))),
        0x65 => Instruction::ADC(try!(read_zp(&mut reader))),
        0x66 => Instruction::ROR(try!(read_zp(&mut reader))),
        0x67 => Instruction::RRA(try!(read_zp(&mut reader))),
        0x68 => Instruction::PLA,
        0x69 => Instruction::ADC(try!(read_imm(&mut reader))),
        0x6A => Instruction::ROR(Operand::Accumulator),
        0x6B => Instruction::ARR(try!(read_imm(&mut reader))),
        0x6C => Instruction::JMP(Operand::Indirect(try!(read_u16(&mut reader)))),
        0x6D => Instruction::ADC(try!(read_abs(&mut reader))),
        0x6E => Instruction::ROR(try!(read_abs(&mut reader))),
        0x6F => Instruction::RRA(try!(read_abs(&mut reader))),

        0x70 => Instruction::BVS(try!(read_byte(&mut reader)) as i8),
        0x71 => Instruction::ADC(try!(read_ind_y(&mut reader))),
        0x72 => Instruction::HLT,
        0x73 => Instruction::RRA(try!(read_ind_y(&mut reader))),
        0x74 => Instruction::IGN(try!(read_zp_x(&mut reader))),
        0x75 => Instruction::ADC(try!(read_zp_x(&mut reader))),
        0x76 => Instruction::ROR(try!(read_zp_x(&mut reader))),
        0x77 => Instruction::RRA(try!(read_zp_x(&mut reader))),
        0x78 => Instruction::SEI,
        0x79 => Instruction::ADC(try!(read_abs_y(&mut reader))),
        0x7A => Instruction::NOP,
        0x7B => Instruction::RRA(try!(read_abs_y(&mut reader))),
        0x7C => Instruction::IGN(try!(read_abs_x(&mut reader))),
        0x7D => Instruction::ADC(try!(read_abs_x(&mut reader))),
        0x7E => Instruction::ROR(try!(read_abs_x(&mut reader))),
        0x7F => Instruction::RRA(try!(read_abs_x(&mut reader))),

        0x80 => Instruction::SKB(try!(read_imm(&mut reader))),
        0x81 => Instruction::STA(try!(read_ind_x(&mut reader))),
        0x82 => Instruction::SKB(try!(read_imm(&mut reader))),
        0x83 => Instruction::SAX(try!(read_ind_x(&mut reader))),
        0x84 => Instruction::STY(try!(read_zp(&mut reader))),
        0x85 => Instruction::STA(try!(read_zp(&mut reader))),
        0x86 => Instruction::STX(try!(read_zp(&mut reader))),
        0x87 => Instruction::SAX(try!(read_zp(&mut reader))),
        0x88 => Instruction::DEY,
        0x89 => Instruction::SKB(try!(read_imm(&mut reader))),
        0x8A => Instruction::TXA,
        0x8B => Instruction::XAA(try!(read_imm(&mut reader))),
        0x8C => Instruction::STY(try!(read_abs(&mut reader))),
        0x8D => Instruction::STA(try!(read_abs(&mut reader))),
        0x8E => Instruction::STX(try!(read_abs(&mut reader))),
        0x8F => Instruction::SAX(try!(read_abs(&mut reader))),

        0x90 => Instruction::BCC(try!(read_byte(&mut reader)) as i8),
        0x91 => Instruction::STA(try!(read_ind_y(&mut reader))),
        0x92 => Instruction::HLT,
        0x93 => Instruction::AHX(try!(read_ind_y(&mut reader))),
        0x94 => Instruction::STY(try!(read_zp_x(&mut reader))),
        0x95 => Instruction::STA(try!(read_zp_x(&mut reader))),
        0x96 => Instruction::STX(try!(read_zp_y(&mut reader))),
        0x97 => Instruction::SAX(try!(read_zp_y(&mut reader))),
        0x98 => Instruction::TYA,
        0x99 => Instruction::STA(try!(read_abs_y(&mut reader))),
        0x9A => Instruction::TXS,
        0x9B => Instruction::TAS(try!(read_abs_y(&mut reader))),
        0x9C => Instruction::SHY(try!(read_abs_x(&mut reader))),
        0x9D => Instruction::STA(try!(read_abs_x(&mut reader))),
        0x9E => Instruction::SHX(try!(read_abs_y(&mut reader))),
        0x9F => Instruction::AHX(try!(read_abs_y(&mut reader))),

        0xA0 => Instruction::LDY(try!(read_imm(&mut reader))),
        0xA1 => Instruction::LDA(try!(read_ind_x(&mut reader))),
        0xA2 => Instruction::LDX(try!(read_imm(&mut reader))),
        0xA3 => Instruction::LAX(try!(read_ind_x(&mut reader))),
        0xA4 => Instruction::LDY(try!(read_zp(&mut reader))),
        0xA5 => Instruction::LDA(try!(read_zp(&mut reader))),
        0xA6 => Instruction::LDX(try!(read_zp(&mut reader))),
        0xA7 => Instruction::LAX(try!(read_zp(&mut reader))),
        0xA8 => Instruction::TAY,
        0xA9 => Instruction::LDA(try!(read_imm(&mut reader))),
        0xAA => Instruction::TAX,
        0xAB => Instruction::LAX(try!(read_imm(&mut reader))),
        0xAC => Instruction::LDY(try!(read_abs(&mut reader))),
        0xAD => Instruction::LDA(try!(read_abs(&mut reader))),
        0xAE => Instruction::LDX(try!(read_abs(&mut reader))),
        0xAF => Instruction::LAX(try!(read_abs(&mut reader))),

        0xB0 => Instruction::BCS(try!(read_byte(&mut reader)) as i8),
        0xB1 => Instruction::LDA(try!(read_ind_y(&mut reader))),
        0xB2 => Instruction::HLT,
        0xB3 => Instruction::LAX(try!(read_ind_y(&mut reader))),
        0xB4 => Instruction::LDY(try!(read_zp_x(&mut reader))),
        0xB5 => Instruction::LDA(try!(read_zp_x(&mut reader))),
        0xB6 => Instruction::LDX(try!(read_zp_y(&mut reader))),
        0xB7 => Instruction::LAX(try!(read_zp_y(&mut reader))),
        0xB8 => Instruction::CLV,
        0xB9 => Instruction::LDA(try!(read_abs_y(&mut reader))),
        0xBA => Instruction::TSX,
        0xBB => Instruction::LAS(try!(read_abs_y(&mut reader))),
        0xBC => Instruction::LDY(try!(read_abs_x(&mut reader))),
        0xBD => Instruction::LDA(try!(read_abs_x(&mut reader))),
        0xBE => Instruction::LDX(try!(read_abs_y(&mut reader))),
        0xBF => Instruction::LAX(try!(read_abs_y(&mut reader))),

        0xC0 => Instruction::CPY(try!(read_imm(&mut reader))),
        0xC1 => Instruction::CMP(try!(read_ind_x(&mut reader))),
        0xC2 => Instruction::SKB(try!(read_imm(&mut reader))),
        0xC3 => Instruction::DCP(try!(read_ind_x(&mut reader))),
        0xC4 => Instruction::CPY(try!(read_zp(&mut reader))),
        0xC5 => Instruction::CMP(try!(read_zp(&mut reader))),
        0xC6 => Instruction::DEC(try!(read_zp(&mut reader))),
        0xC7 => Instruction::DCP(try!(read_zp(&mut reader))),
        0xC8 => Instruction::INY,
        0xC9 => Instruction::CMP(try!(read_imm(&mut reader))),
        0xCA => Instruction::DEX,
        0xCB => Instruction::AXS(try!(read_imm(&mut reader))),
        0xCC => Instruction::CPY(try!(read_abs(&mut reader))),
        0xCD => Instruction::CMP(try!(read_abs(&mut reader))),
        0xCE => Instruction::DEC(try!(read_abs(&mut reader))),
        0xCF => Instruction::DCP(try!(read_abs(&mut reader))),

        0xD0 => Instruction::BNE(try!(read_byte(&mut reader)) as i8),
        0xD1 => Instruction::CMP(try!(read_ind_y(&mut reader))),
        0xD2 => Instruction::HLT,
        0xD3 => Instruction::DCP(try!(read_ind_y(&mut reader))),
        0xD4 => Instruction::IGN(try!(read_zp_x(&mut reader))),
        0xD5 => Instruction::CMP(try!(read_zp_x(&mut reader))),
        0xD6 => Instruction::DEC(try!(read_zp_x(&mut reader))),
        0xD7 => Instruction::DCP(try!(read_zp_x(&mut reader))),
        0xD8 => Instruction::CLD,
        0xD9 => Instruction::CMP(try!(read_abs_y(&mut reader))),
        0xDA => Instruction::NOP,
        0xDB => Instruction::DCP(try!(read_abs_y(&mut reader))),
        0xDC => Instruction::IGN(try!(read_abs_x(&mut reader))),
        0xDD => Instruction::CMP(try!(read_abs_x(&mut reader))),
        0xDE => Instruction::DEC(try!(read_abs_x(&mut reader))),
        0xDF => Instruction::DCP(try!(read_abs_x(&mut reader))),

        0xE0 => Instruction::CPX(try!(read_imm(&mut reader))),
        0xE1 => Instruction::SBC(try!(read_ind_x(&mut reader))),
        0xE2 => Instruction::SKB(try!(read_imm(&mut reader))),
        0xE3 => Instruction::ISC(try!(read_ind_x(&mut reader))),
        0xE4 => Instruction::CPX(try!(read_zp(&mut reader))),
        0xE5 => Instruction::SBC(try!(read_zp(&mut reader))),
        0xE6 => Instruction::INC(try!(read_zp(&mut reader))),
        0xE7 => Instruction::ISC(try!(read_zp(&mut reader))),
        0xE8 => Instruction::INX,
        0xE9 => Instruction::SBC(try!(read_imm(&mut reader))),
        0xEA => Instruction::NOP,
        0xEB => Instruction::SBC(try!(read_imm(&mut reader))),
        0xEC => Instruction::CPX(try!(read_abs(&mut reader))),
        0xED => Instruction::SBC(try!(read_abs(&mut reader))),
        0xEE => Instruction::INC(try!(read_abs(&mut reader))),
        0xEF => Instruction::ISC(try!(read_abs(&mut reader))),

        0xF0 => Instruction::BEQ(try!(read_byte(&mut reader)) as i8),
        0xF1 => Instruction::SBC(try!(read_ind_y(&mut reader))),
        0xF2 => Instruction::HLT,
        0xF3 => Instruction::ISC(try!(read_ind_y(&mut reader))),
        0xF4 => Instruction::IGN(try!(read_zp_x(&mut reader))),
        0xF5 => Instruction::SBC(try!(read_zp_x(&mut reader))),
        0xF6 => Instruction::INC(try!(read_zp_x(&mut reader))),
        0xF7 => Instruction::ISC(try!(read_zp_x(&mut reader))),
        0xF8 => Instruction::SED,
        0xF9 => Instruction::SBC(try!(read_abs_y(&mut reader))),
        0xFA => Instruction::NOP,
        0xFB => Instruction::ISC(try!(read_abs_y(&mut reader))),
        0xFC => Instruction::IGN(try!(read_abs_x(&mut reader))),
        0xFD => Instruction::SBC(try!(read_abs_x(&mut reader))),
        0xFE => Instruction::INC(try!(read_abs_x(&mut reader))),
        0xFF => Instruction::ISC(try!(read_abs_x(&mut reader))),

        _ => return Err(Error::UnknownOpcode(opcode))
    };

    Ok(instr)
}

fn read_abs<R>(reader: &mut R) -> Result<Operand> where R: io::Read {
    Ok(Operand::Absolute(try!(read_u16(reader))))
}

fn read_abs_x<R>(reader: &mut R) -> Result<Operand> where R: io::Read {
    Ok(Operand::Indexed(try!(read_u16(reader)), RegisterName::X))
}

fn read_abs_y<R>(reader: &mut R) -> Result<Operand> where R: io::Read {
    Ok(Operand::Indexed(try!(read_u16(reader)), RegisterName::Y))
}

fn read_u16<R>(reader: &mut R) -> Result<u16> where R: io::Read {
    let low : u16 = try!(read_byte(reader)) as u16;
    let high : u16 = try!(read_byte(reader)) as u16;
    Ok((high << 8) | low)
}

fn read_imm<R>(reader: &mut R) -> Result<Operand> where R: io::Read {
    Ok(Operand::Immediate(try!(read_byte(reader))))
}

fn read_zp<R>(reader: &mut R) -> Result<Operand> where R: io::Read {
    let zp = try!(read_byte(reader));
    Ok(Operand::Absolute(zp as u16))
}

fn read_zp_x<R>(reader: &mut R) -> Result<Operand> where R: io::Read {
    let zp = try!(read_byte(reader));
    Ok(Operand::Indexed(zp as u16, RegisterName::X))
}

fn read_zp_y<R>(reader: &mut R) -> Result<Operand> where R: io::Read {
    let zp = try!(read_byte(reader));
    Ok(Operand::Indexed(zp as u16, RegisterName::Y))
}

fn read_ind_x<R>(reader: &mut R) -> Result<Operand> where R: io::Read {
    Ok(Operand::PreIndexedIndirect(try!(read_byte(reader))))
}

fn read_ind_y<R>(reader: &mut R) -> Result<Operand> where R: io::Read {
    Ok(Operand::PostIndexedIndirect(try!(read_byte(reader))))
}

fn read_byte<R>(reader: &mut R) -> Result<u8> where R: io::Read {
    let mut buf : [u8; 1] = [0; 1];
    
    match reader.read(&mut buf) {
        Ok(0) => Err(Error::EndOfFile),
        Ok(_) => Ok(buf[0]),
        Err(e) => Err(Error::from_io(e))
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use cpus::mos6502::{Operand,Instruction,RegisterName};
    use cpus::mos6502::instr::decode;

    #[test]
    pub fn can_decode_adc() {
        decoder_test(vec![0x69, 0x42], Instruction::ADC(Operand::Immediate(0x42)));
        decoder_test(vec![0x65, 0xAB], Instruction::ADC(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x75, 0xAB], Instruction::ADC(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x6D, 0xCD, 0xAB], Instruction::ADC(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x7D, 0xCD, 0xAB], Instruction::ADC(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0x79, 0xCD, 0xAB], Instruction::ADC(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0x61, 0xAB], Instruction::ADC(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x71, 0xAB], Instruction::ADC(Operand::PostIndexedIndirect(0xAB)));
    }

    #[test]
    pub fn can_decode_and() {
        decoder_test(vec![0x29, 0x42], Instruction::AND(Operand::Immediate(0x42)));
        decoder_test(vec![0x25, 0xAB], Instruction::AND(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x35, 0xAB], Instruction::AND(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x2D, 0xCD, 0xAB], Instruction::AND(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x3D, 0xCD, 0xAB], Instruction::AND(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0x39, 0xCD, 0xAB], Instruction::AND(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0x21, 0xAB], Instruction::AND(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x31, 0xAB], Instruction::AND(Operand::PostIndexedIndirect(0xAB)));
    }

    #[test]
    pub fn can_decode_asl() {
        decoder_test(vec![0x0A], Instruction::ASL(Operand::Accumulator));
        decoder_test(vec![0x06, 0xAB], Instruction::ASL(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x16, 0xAB], Instruction::ASL(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x0E, 0xCD, 0xAB], Instruction::ASL(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x1E, 0xCD, 0xAB], Instruction::ASL(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_branches() {
        decoder_test(vec![0x90, 0x82], Instruction::BCC(-126));
        decoder_test(vec![0xB0, 0x82], Instruction::BCS(-126));
        decoder_test(vec![0xF0, 0x82], Instruction::BEQ(-126));
        decoder_test(vec![0x30, 0x82], Instruction::BMI(-126));
        decoder_test(vec![0xD0, 0x82], Instruction::BNE(-126));
        decoder_test(vec![0x10, 0x82], Instruction::BPL(-126));
        decoder_test(vec![0x50, 0x82], Instruction::BVC(-126));
        decoder_test(vec![0x70, 0x82], Instruction::BVS(-126));
    }

    #[test]
    pub fn can_decode_brk() {
        decoder_test(vec![0x00], Instruction::BRK);
    }

    #[test]
    pub fn can_decode_bit() {
        decoder_test(vec![0x24, 0xAB], Instruction::BIT(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x2C, 0xCD, 0xAB], Instruction::BIT(Operand::Absolute(0xABCD)));
    }

    #[test]
    pub fn can_decode_clear_flags() {
        decoder_test(vec![0x18], Instruction::CLC);
        decoder_test(vec![0xD8], Instruction::CLD);
        decoder_test(vec![0x58], Instruction::CLI);
        decoder_test(vec![0xB8], Instruction::CLV);
    }

    #[test]
    pub fn can_decode_cmp() {
        decoder_test(vec![0xC9, 0x42], Instruction::CMP(Operand::Immediate(0x42)));
        decoder_test(vec![0xC5, 0xAB], Instruction::CMP(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xD5, 0xAB], Instruction::CMP(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xCD, 0xCD, 0xAB], Instruction::CMP(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xDD, 0xCD, 0xAB], Instruction::CMP(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0xD9, 0xCD, 0xAB], Instruction::CMP(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0xC1, 0xAB], Instruction::CMP(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0xD1, 0xAB], Instruction::CMP(Operand::PostIndexedIndirect(0xAB)));
    }

    #[test]
    pub fn can_decode_cpx() {
        decoder_test(vec![0xE0, 0x42], Instruction::CPX(Operand::Immediate(0x42)));
        decoder_test(vec![0xE4, 0xAB], Instruction::CPX(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xEC, 0xCD, 0xAB], Instruction::CPX(Operand::Absolute(0xABCD)));
    }

    #[test]
    pub fn can_decode_cpy() {
        decoder_test(vec![0xC0, 0x42], Instruction::CPY(Operand::Immediate(0x42)));
        decoder_test(vec![0xC4, 0xAB], Instruction::CPY(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xCC, 0xCD, 0xAB], Instruction::CPY(Operand::Absolute(0xABCD)));
    }

    #[test]
    pub fn can_decode_dec() {
        decoder_test(vec![0xC6, 0xAB], Instruction::DEC(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xD6, 0xAB], Instruction::DEC(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xCE, 0xCD, 0xAB], Instruction::DEC(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xDE, 0xCD, 0xAB], Instruction::DEC(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_dex() {
        decoder_test(vec![0xCA], Instruction::DEX);
    }

    #[test]
    pub fn can_decode_dey() {
        decoder_test(vec![0x88], Instruction::DEY);
    }

    #[test]
    pub fn can_decode_eor() {
        decoder_test(vec![0x49, 0x42], Instruction::EOR(Operand::Immediate(0x42)));
        decoder_test(vec![0x45, 0xAB], Instruction::EOR(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x55, 0xAB], Instruction::EOR(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x4D, 0xCD, 0xAB], Instruction::EOR(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x5D, 0xCD, 0xAB], Instruction::EOR(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0x59, 0xCD, 0xAB], Instruction::EOR(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0x41, 0xAB], Instruction::EOR(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x51, 0xAB], Instruction::EOR(Operand::PostIndexedIndirect(0xAB)));
    }

    #[test]
    pub fn can_decode_inc() {
        decoder_test(vec![0xE6, 0xAB], Instruction::INC(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xF6, 0xAB], Instruction::INC(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xEE, 0xCD, 0xAB], Instruction::INC(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xFE, 0xCD, 0xAB], Instruction::INC(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_inx() {
        decoder_test(vec![0xE8], Instruction::INX);
    }

    #[test]
    pub fn can_decode_iny() {
        decoder_test(vec![0xC8], Instruction::INY);
    }

    #[test]
    pub fn can_decode_jmp() {
        decoder_test(vec![0x4C, 0xCD, 0xAB], Instruction::JMP(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x6C, 0xCD, 0xAB], Instruction::JMP(Operand::Indirect(0xABCD)));
    }

    #[test]
    pub fn can_decode_jsr() {
        decoder_test(vec![0x20, 0xCD, 0xAB], Instruction::JSR(0xABCD));
    }

    #[test]
    pub fn can_decode_lda() {
        decoder_test(vec![0xA9, 0x42], Instruction::LDA(Operand::Immediate(0x42)));
        decoder_test(vec![0xA5, 0xAB], Instruction::LDA(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xB5, 0xAB], Instruction::LDA(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xAD, 0xCD, 0xAB], Instruction::LDA(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xBD, 0xCD, 0xAB], Instruction::LDA(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0xB9, 0xCD, 0xAB], Instruction::LDA(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0xA1, 0xAB], Instruction::LDA(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0xB1, 0xAB], Instruction::LDA(Operand::PostIndexedIndirect(0xAB)));
    }

    #[test]
    pub fn can_decode_ldx() {
        decoder_test(vec![0xA2, 0x42], Instruction::LDX(Operand::Immediate(0x42)));
        decoder_test(vec![0xA6, 0xAB], Instruction::LDX(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xB6, 0xAB], Instruction::LDX(Operand::Indexed(0x00AB, RegisterName::Y)));
        decoder_test(vec![0xAE, 0xCD, 0xAB], Instruction::LDX(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xBE, 0xCD, 0xAB], Instruction::LDX(Operand::Indexed(0xABCD, RegisterName::Y)));
    }

    #[test]
    pub fn can_decode_ldy() {
        decoder_test(vec![0xA0, 0x42], Instruction::LDY(Operand::Immediate(0x42)));
        decoder_test(vec![0xA4, 0xAB], Instruction::LDY(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xB4, 0xAB], Instruction::LDY(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xAC, 0xCD, 0xAB], Instruction::LDY(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xBC, 0xCD, 0xAB], Instruction::LDY(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_lsr() {
        decoder_test(vec![0x4A], Instruction::LSR(Operand::Accumulator));
        decoder_test(vec![0x46, 0xAB], Instruction::LSR(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x56, 0xAB], Instruction::LSR(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x4E, 0xCD, 0xAB], Instruction::LSR(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x5E, 0xCD, 0xAB], Instruction::LSR(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_nop() {
        decoder_test(vec![0xEA], Instruction::NOP);

        // Unofficial NOPs
        decoder_test(vec![0x1A], Instruction::NOP);
        decoder_test(vec![0x3A], Instruction::NOP);
        decoder_test(vec![0x5A], Instruction::NOP);
        decoder_test(vec![0x7A], Instruction::NOP);
        decoder_test(vec![0xDA], Instruction::NOP);
        decoder_test(vec![0xFA], Instruction::NOP);
    }

    #[test]
    pub fn can_decode_ora() {
        decoder_test(vec![0x09, 0x42], Instruction::ORA(Operand::Immediate(0x42)));
        decoder_test(vec![0x05, 0xAB], Instruction::ORA(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x15, 0xAB], Instruction::ORA(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x0D, 0xCD, 0xAB], Instruction::ORA(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x1D, 0xCD, 0xAB], Instruction::ORA(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0x19, 0xCD, 0xAB], Instruction::ORA(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0x01, 0xAB], Instruction::ORA(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x11, 0xAB], Instruction::ORA(Operand::PostIndexedIndirect(0xAB)));
    }

    #[test]
    pub fn can_decode_pha() {
        decoder_test(vec![0x48], Instruction::PHA);
    }

    #[test]
    pub fn can_decode_php() {
        decoder_test(vec![0x08], Instruction::PHP);
    }

    #[test]
    pub fn can_decode_pla() {
        decoder_test(vec![0x68], Instruction::PLA);
    }

    #[test]
    pub fn can_decode_plp() {
        decoder_test(vec![0x28], Instruction::PLP);
    }

    #[test]
    pub fn can_decode_rol() {
        decoder_test(vec![0x2A], Instruction::ROL(Operand::Accumulator));
        decoder_test(vec![0x26, 0xAB], Instruction::ROL(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x36, 0xAB], Instruction::ROL(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x2E, 0xCD, 0xAB], Instruction::ROL(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x3E, 0xCD, 0xAB], Instruction::ROL(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_ror() {
        decoder_test(vec![0x6A], Instruction::ROR(Operand::Accumulator));
        decoder_test(vec![0x66, 0xAB], Instruction::ROR(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x76, 0xAB], Instruction::ROR(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x6E, 0xCD, 0xAB], Instruction::ROR(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x7E, 0xCD, 0xAB], Instruction::ROR(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_rti() {
        decoder_test(vec![0x40], Instruction::RTI);
    }

    #[test]
    pub fn can_decode_rts() {
        decoder_test(vec![0x60], Instruction::RTS);
    }

    #[test]
    pub fn can_decode_sbc() {
        decoder_test(vec![0xE9, 0x42], Instruction::SBC(Operand::Immediate(0x42)));
        decoder_test(vec![0xE5, 0xAB], Instruction::SBC(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xF5, 0xAB], Instruction::SBC(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xED, 0xCD, 0xAB], Instruction::SBC(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xFD, 0xCD, 0xAB], Instruction::SBC(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0xF9, 0xCD, 0xAB], Instruction::SBC(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0xE1, 0xAB], Instruction::SBC(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0xF1, 0xAB], Instruction::SBC(Operand::PostIndexedIndirect(0xAB)));

        // Unofficial SBC variant
        decoder_test(vec![0xEB, 0x42], Instruction::SBC(Operand::Immediate(0x42)));
    }

    #[test]
    pub fn can_decode_set_flags() {
        decoder_test(vec![0x38], Instruction::SEC);
        decoder_test(vec![0xF8], Instruction::SED);
        decoder_test(vec![0x78], Instruction::SEI);
    }

    #[test]
    pub fn can_decode_sta() {
        decoder_test(vec![0x85, 0xAB], Instruction::STA(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x95, 0xAB], Instruction::STA(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x8D, 0xCD, 0xAB], Instruction::STA(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x9D, 0xCD, 0xAB], Instruction::STA(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0x99, 0xCD, 0xAB], Instruction::STA(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0x81, 0xAB], Instruction::STA(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x91, 0xAB], Instruction::STA(Operand::PostIndexedIndirect(0xAB)));
    }

    #[test]
    pub fn can_decode_stx() {
        decoder_test(vec![0x86, 0xAB], Instruction::STX(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x96, 0xAB], Instruction::STX(Operand::Indexed(0x00AB, RegisterName::Y)));
        decoder_test(vec![0x8E, 0xCD, 0xAB], Instruction::STX(Operand::Absolute(0xABCD)));
    }

    #[test]
    pub fn can_decode_sty() {
        decoder_test(vec![0x84, 0xAB], Instruction::STY(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x94, 0xAB], Instruction::STY(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x8C, 0xCD, 0xAB], Instruction::STY(Operand::Absolute(0xABCD)));
    }

    #[test]
    pub fn can_decode_transfers() {
        decoder_test(vec![0xAA], Instruction::TAX);
        decoder_test(vec![0xA8], Instruction::TAY);
        decoder_test(vec![0xBA], Instruction::TSX);
        decoder_test(vec![0x8A], Instruction::TXA);
        decoder_test(vec![0x9A], Instruction::TXS);
        decoder_test(vec![0x98], Instruction::TYA);
    }

    // Unofficial opcodes
    // http://wiki.nesdev.com/w/index.php/Programming_with_unofficial_opcodes
    
    #[test]
    pub fn can_decode_alr() {
        decoder_test(vec![0x4B, 0x42], Instruction::ALR(Operand::Immediate(0x42)));
    }

    #[test]
    pub fn can_decode_anc() {
        decoder_test(vec![0x0B, 0x42], Instruction::ANC(Operand::Immediate(0x42)));
        decoder_test(vec![0x2B, 0x42], Instruction::ANC(Operand::Immediate(0x42)));
    }

    #[test]
    pub fn can_decode_arr() {
        decoder_test(vec![0x6B, 0x42], Instruction::ARR(Operand::Immediate(0x42)));
    }

    #[test]
    pub fn can_decode_axs() {
        decoder_test(vec![0xCB, 0x42], Instruction::AXS(Operand::Immediate(0x42)));
    }

    #[test]
    pub fn can_decode_lax() {
        decoder_test(vec![0xAB, 0x42], Instruction::LAX(Operand::Immediate(0x42)));
        decoder_test(vec![0xA3, 0xAB], Instruction::LAX(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0xA7, 0xAB], Instruction::LAX(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xAF, 0xCD, 0xAB], Instruction::LAX(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xB3, 0xAB], Instruction::LAX(Operand::PostIndexedIndirect(0xAB)));
        decoder_test(vec![0xB7, 0xAB], Instruction::LAX(Operand::Indexed(0x00AB, RegisterName::Y)));
        decoder_test(vec![0xBF, 0xCD, 0xAB], Instruction::LAX(Operand::Indexed(0xABCD, RegisterName::Y)));
    }

    #[test]
    pub fn can_decode_sax() {
        decoder_test(vec![0x83, 0xAB], Instruction::SAX(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x87, 0xAB], Instruction::SAX(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x8F, 0xCD, 0xAB], Instruction::SAX(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x97, 0xAB], Instruction::SAX(Operand::Indexed(0x00AB, RegisterName::Y)));
    }

    #[test]
    pub fn can_decode_dcp() {
        decoder_test(vec![0xC3, 0xAB], Instruction::DCP(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0xC7, 0xAB], Instruction::DCP(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xCF, 0xCD, 0xAB], Instruction::DCP(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xD3, 0xAB], Instruction::DCP(Operand::PostIndexedIndirect(0xAB)));
        decoder_test(vec![0xD7, 0xAB], Instruction::DCP(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xDB, 0xCD, 0xAB], Instruction::DCP(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0xDF, 0xCD, 0xAB], Instruction::DCP(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_isc() {
        decoder_test(vec![0xE3, 0xAB], Instruction::ISC(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0xE7, 0xAB], Instruction::ISC(Operand::Absolute(0x00AB)));
        decoder_test(vec![0xEF, 0xCD, 0xAB], Instruction::ISC(Operand::Absolute(0xABCD)));
        decoder_test(vec![0xF3, 0xAB], Instruction::ISC(Operand::PostIndexedIndirect(0xAB)));
        decoder_test(vec![0xF7, 0xAB], Instruction::ISC(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xFB, 0xCD, 0xAB], Instruction::ISC(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0xFF, 0xCD, 0xAB], Instruction::ISC(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_rla() {
        decoder_test(vec![0x23, 0xAB], Instruction::RLA(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x27, 0xAB], Instruction::RLA(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x2F, 0xCD, 0xAB], Instruction::RLA(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x33, 0xAB], Instruction::RLA(Operand::PostIndexedIndirect(0xAB)));
        decoder_test(vec![0x37, 0xAB], Instruction::RLA(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x3B, 0xCD, 0xAB], Instruction::RLA(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0x3F, 0xCD, 0xAB], Instruction::RLA(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_rra() {
        decoder_test(vec![0x63, 0xAB], Instruction::RRA(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x67, 0xAB], Instruction::RRA(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x6F, 0xCD, 0xAB], Instruction::RRA(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x73, 0xAB], Instruction::RRA(Operand::PostIndexedIndirect(0xAB)));
        decoder_test(vec![0x77, 0xAB], Instruction::RRA(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x7B, 0xCD, 0xAB], Instruction::RRA(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0x7F, 0xCD, 0xAB], Instruction::RRA(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_slo() {
        decoder_test(vec![0x03, 0xAB], Instruction::SLO(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x07, 0xAB], Instruction::SLO(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x0F, 0xCD, 0xAB], Instruction::SLO(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x13, 0xAB], Instruction::SLO(Operand::PostIndexedIndirect(0xAB)));
        decoder_test(vec![0x17, 0xAB], Instruction::SLO(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x1B, 0xCD, 0xAB], Instruction::SLO(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0x1F, 0xCD, 0xAB], Instruction::SLO(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_sre() {
        decoder_test(vec![0x43, 0xAB], Instruction::SRE(Operand::PreIndexedIndirect(0xAB)));
        decoder_test(vec![0x47, 0xAB], Instruction::SRE(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x4F, 0xCD, 0xAB], Instruction::SRE(Operand::Absolute(0xABCD)));
        decoder_test(vec![0x53, 0xAB], Instruction::SRE(Operand::PostIndexedIndirect(0xAB)));
        decoder_test(vec![0x57, 0xAB], Instruction::SRE(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x5B, 0xCD, 0xAB], Instruction::SRE(Operand::Indexed(0xABCD, RegisterName::Y)));
        decoder_test(vec![0x5F, 0xCD, 0xAB], Instruction::SRE(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_skb() {
        decoder_test(vec![0x80, 0x42], Instruction::SKB(Operand::Immediate(0x42)));
        decoder_test(vec![0x82, 0x42], Instruction::SKB(Operand::Immediate(0x42)));
        decoder_test(vec![0x89, 0x42], Instruction::SKB(Operand::Immediate(0x42)));
        decoder_test(vec![0xC2, 0x42], Instruction::SKB(Operand::Immediate(0x42)));
        decoder_test(vec![0xE2, 0x42], Instruction::SKB(Operand::Immediate(0x42)));
    }

    #[test]
    pub fn can_decode_ign() {
        decoder_test(vec![0x0C, 0xCD, 0xAB], Instruction::IGN(Operand::Absolute(0xABCD)));

        decoder_test(vec![0x1C, 0xCD, 0xAB], Instruction::IGN(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0x3C, 0xCD, 0xAB], Instruction::IGN(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0x5C, 0xCD, 0xAB], Instruction::IGN(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0x7C, 0xCD, 0xAB], Instruction::IGN(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0xDC, 0xCD, 0xAB], Instruction::IGN(Operand::Indexed(0xABCD, RegisterName::X)));
        decoder_test(vec![0xFC, 0xCD, 0xAB], Instruction::IGN(Operand::Indexed(0xABCD, RegisterName::X)));

        decoder_test(vec![0x04, 0xAB], Instruction::IGN(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x44, 0xAB], Instruction::IGN(Operand::Absolute(0x00AB)));
        decoder_test(vec![0x64, 0xAB], Instruction::IGN(Operand::Absolute(0x00AB)));

        decoder_test(vec![0x14, 0xAB], Instruction::IGN(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x34, 0xAB], Instruction::IGN(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x54, 0xAB], Instruction::IGN(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0x74, 0xAB], Instruction::IGN(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xD4, 0xAB], Instruction::IGN(Operand::Indexed(0x00AB, RegisterName::X)));
        decoder_test(vec![0xF4, 0xAB], Instruction::IGN(Operand::Indexed(0x00AB, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_hlt() {
        decoder_test(vec![0x12], Instruction::HLT);
        decoder_test(vec![0x22], Instruction::HLT);
        decoder_test(vec![0x32], Instruction::HLT);
        decoder_test(vec![0x42], Instruction::HLT);
        decoder_test(vec![0x52], Instruction::HLT);
        decoder_test(vec![0x62], Instruction::HLT);
        decoder_test(vec![0x72], Instruction::HLT);
        decoder_test(vec![0x92], Instruction::HLT);
        decoder_test(vec![0xB2], Instruction::HLT);
        decoder_test(vec![0xD2], Instruction::HLT);
        decoder_test(vec![0xF2], Instruction::HLT);
    }

    #[test]
    pub fn can_decode_xaa() {
        decoder_test(vec![0x8B, 0x42], Instruction::XAA(Operand::Immediate(0x42)));
    }

    #[test]
    pub fn can_decode_ahx() {
        decoder_test(vec![0x93, 0xAB], Instruction::AHX(Operand::PostIndexedIndirect(0xAB)));
        decoder_test(vec![0x9F, 0xCD, 0xAB], Instruction::AHX(Operand::Indexed(0xABCD, RegisterName::Y)));
    }

    #[test]
    pub fn can_decode_tas() {
        decoder_test(vec![0x9B, 0xCD, 0xAB], Instruction::TAS(Operand::Indexed(0xABCD, RegisterName::Y)));
    }

    #[test]
    pub fn can_decode_shy() {
        decoder_test(vec![0x9C, 0xCD, 0xAB], Instruction::SHY(Operand::Indexed(0xABCD, RegisterName::X)));
    }

    #[test]
    pub fn can_decode_shx() {
        decoder_test(vec![0x9E, 0xCD, 0xAB], Instruction::SHX(Operand::Indexed(0xABCD, RegisterName::Y)));
    }

    #[test]
    pub fn can_decode_las() {
        decoder_test(vec![0xBB, 0xCD, 0xAB], Instruction::LAS(Operand::Indexed(0xABCD, RegisterName::Y)));
    }

    fn decoder_test(bytes: Vec<u8>, expected: Instruction) {
        let result = decode(&mut Cursor::new(bytes.as_ref()));
        match result {
            Ok(actual) => if actual != expected {
                panic!("Decoding of 0x{:X} was [{}] but expected [{}]", bytes[0], actual, expected);
            },
            x => panic!("Decoding of 0x{:X} failed: {:?}", bytes[0], x)
        }
    }
}
