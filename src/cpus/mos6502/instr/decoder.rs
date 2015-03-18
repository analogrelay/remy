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
