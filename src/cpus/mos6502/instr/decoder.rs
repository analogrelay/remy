use std::{error,io};

use cpus::mos6502::{Operand,Instruction,RegisterName};

#[derive(Debug,Eq,PartialEq)]
pub enum Error {
    UnknownOpcode,
    IoError(io::Error)
}

impl error::FromError<io::Error> for Error {
    fn from_error(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

pub fn decode<R>(reader: &mut R) -> Result<Instruction, Error> where R: io::Read {
    // Read the opcode
    let opcode = try!(read_byte(reader));

    // Determine the next step based on the instruction
    let instr = match opcode {
        0x69 => Instruction::ADC(try!(read_imm(reader))),
        0x65 => Instruction::ADC(try!(read_zp(reader))),
        0x75 => Instruction::ADC(try!(read_zp_x(reader))),
        0x6D => Instruction::ADC(try!(read_abs(reader))),
        0x7D => Instruction::ADC(try!(read_abs_x(reader))),
        0x79 => Instruction::ADC(try!(read_abs_y(reader))),
        0x61 => Instruction::ADC(try!(read_ind_x(reader))),
        0x71 => Instruction::ADC(try!(read_ind_y(reader))),

        0x29 => Instruction::AND(try!(read_imm(reader))),
        0x25 => Instruction::AND(try!(read_zp(reader))),
        0x35 => Instruction::AND(try!(read_zp_x(reader))),
        0x2D => Instruction::AND(try!(read_abs(reader))),
        0x3D => Instruction::AND(try!(read_abs_x(reader))),
        0x39 => Instruction::AND(try!(read_abs_y(reader))),
        0x21 => Instruction::AND(try!(read_ind_x(reader))),
        0x31 => Instruction::AND(try!(read_ind_y(reader))),

        0x0A => Instruction::ASL(Operand::Accumulator),
        0x06 => Instruction::ASL(try!(read_zp(reader))),
        0x16 => Instruction::ASL(try!(read_zp_x(reader))),
        0x0E => Instruction::ASL(try!(read_abs(reader))),
        0x1E => Instruction::ASL(try!(read_abs_x(reader))),

        0x90 => Instruction::BCC(try!(read_byte(reader)) as i8),
        0xB0 => Instruction::BCS(try!(read_byte(reader)) as i8),
        0xF0 => Instruction::BEQ(try!(read_byte(reader)) as i8),

        0x24 => Instruction::BIT(try!(read_zp(reader))),
        0x2C => Instruction::BIT(try!(read_abs(reader))),

        0x30 => Instruction::BMI(try!(read_byte(reader)) as i8),
        0xD0 => Instruction::BNE(try!(read_byte(reader)) as i8),
        0x10 => Instruction::BPL(try!(read_byte(reader)) as i8),

        0x00 => Instruction::BRK,

        0x50 => Instruction::BVC(try!(read_byte(reader)) as i8),
        0x70 => Instruction::BVS(try!(read_byte(reader)) as i8),

        0x18 => Instruction::CLC,
        0xD8 => Instruction::CLD,
        0x58 => Instruction::CLI,
        0xB8 => Instruction::CLV,

        0xC9 => Instruction::CMP(try!(read_imm(reader))),
        0xC5 => Instruction::CMP(try!(read_zp(reader))),
        0xD5 => Instruction::CMP(try!(read_zp_x(reader))),
        0xCD => Instruction::CMP(try!(read_abs(reader))),
        0xDD => Instruction::CMP(try!(read_abs_x(reader))),
        0xD9 => Instruction::CMP(try!(read_abs_y(reader))),
        0xC1 => Instruction::CMP(try!(read_ind_x(reader))),
        0xD1 => Instruction::CMP(try!(read_ind_y(reader))),

        0xE0 => Instruction::CPX(try!(read_imm(reader))),
        0xE4 => Instruction::CPX(try!(read_zp(reader))),
        0xEC => Instruction::CPX(try!(read_abs(reader))),

        0xC0 => Instruction::CPY(try!(read_imm(reader))),
        0xC4 => Instruction::CPY(try!(read_zp(reader))),
        0xCC => Instruction::CPY(try!(read_abs(reader))),

        0xC6 => Instruction::DEC(try!(read_zp(reader))),
        0xD6 => Instruction::DEC(try!(read_zp_x(reader))),
        0xCE => Instruction::DEC(try!(read_abs(reader))),
        0xDE => Instruction::DEC(try!(read_abs_x(reader))),

        0xCA => Instruction::DEX,
        0x88 => Instruction::DEY,

        0x49 => Instruction::EOR(try!(read_imm(reader))),
        0x45 => Instruction::EOR(try!(read_zp(reader))),
        0x55 => Instruction::EOR(try!(read_zp_x(reader))),
        0x4D => Instruction::EOR(try!(read_abs(reader))),
        0x5D => Instruction::EOR(try!(read_abs_x(reader))),
        0x59 => Instruction::EOR(try!(read_abs_y(reader))),
        0x41 => Instruction::EOR(try!(read_ind_x(reader))),
        0x51 => Instruction::EOR(try!(read_ind_y(reader))),

        0xE6 => Instruction::INC(try!(read_zp(reader))),
        0xF6 => Instruction::INC(try!(read_zp_x(reader))),
        0xEE => Instruction::INC(try!(read_abs(reader))),
        0xFE => Instruction::INC(try!(read_abs_x(reader))),

        0xE8 => Instruction::INX,
        0xC8 => Instruction::INY,
        
        _ => return Err(Error::UnknownOpcode)
    };

    Ok(instr)
}

fn read_abs<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    Ok(Operand::Absolute(try!(read_u16(reader))))
}

fn read_abs_x<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    Ok(Operand::Indexed(try!(read_u16(reader)), RegisterName::X))
}

fn read_abs_y<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    Ok(Operand::Indexed(try!(read_u16(reader)), RegisterName::Y))
}

fn read_u16<R>(reader: &mut R) -> Result<u16, io::Error> where R: io::Read {
    let low : u16 = try!(read_byte(reader)) as u16;
    let high : u16 = try!(read_byte(reader)) as u16;
    Ok((high << 8) | low)
}

fn read_imm<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    Ok(Operand::Immediate(try!(read_byte(reader))))
}

fn read_zp<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    let zp = try!(read_byte(reader));
    Ok(Operand::Absolute(zp as u16))
}

fn read_zp_x<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    let zp = try!(read_byte(reader));
    Ok(Operand::Indexed(zp as u16, RegisterName::X))
}

fn read_ind_x<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    Ok(Operand::PreIndexedIndirect(try!(read_byte(reader))))
}

fn read_ind_y<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    Ok(Operand::PostIndexedIndirect(try!(read_byte(reader))))
}

fn read_byte<R>(reader: &mut R) -> Result<u8, io::Error> where R: io::Read {
    let mut buf : [u8; 1] = [0; 1];
    
    match reader.read(&mut buf) {
        Ok(_) => Ok(buf[0]),
        Err(e) => Err(e)
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

    fn decoder_test(bytes: Vec<u8>, expected: Instruction) {
        let result = decode(&mut Cursor::new(bytes.as_slice()));
        match result {
            Ok(actual) => if actual != expected {
                panic!("Decoding of 0x{:X} was [{}] but expected [{}]", bytes[0], actual, expected);
            },
            x => panic!("Decoding of 0x{:X} failed: {:?}", bytes[0], x)
        }
    }
}
