use std::{error,io};

use cpus::mos6502::{Operand,Instruction};

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
        0x00 => Instruction::BRK,
        0x01 => Instruction::ORA(try!(read_ind_x(reader))),
        0x05 => Instruction::ORA(try!(read_zp(reader))),
        0x06 => Instruction::ASL(try!(read_zp(reader))),
        0x08 => Instruction::PHP,
        0x09 => Instruction::ORA(try!(read_imm(reader))),
        0x0A => Instruction::ASL(Operand::Accumulator),
        0x0D => Instruction::ORA(try!(read_abs(reader))),
        0x0E => Instruction::ASL(try!(read_abs(reader))),
        _ => return Err(Error::UnknownOpcode)
    };

    Ok(instr)
}

fn read_abs<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    let low : u16 = try!(read_byte(reader)) as u16;
    let high : u16 = try!(read_byte(reader)) as u16;
    Ok(Operand::Absolute((high << 8) | low))
}

fn read_imm<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    Ok(Operand::Immediate(try!(read_byte(reader))))
}

fn read_zp<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    let zp = try!(read_byte(reader));
    Ok(Operand::Absolute(zp as u16))
}

fn read_ind_x<R>(reader: &mut R) -> Result<Operand, io::Error> where R: io::Read {
    Ok(Operand::PreIndexedIndirect(try!(read_byte(reader))))
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

    use cpus::mos6502::{Operand,Instruction};
    use cpus::mos6502::instr::decode;

    #[test] pub fn can_decode_brk()         { decoder_test(vec![0x00], Instruction::BRK); }
    #[test] pub fn can_decode_ora_ind_x()   { decoder_test(vec![0x01, 0xAB], Instruction::ORA(Operand::PreIndexedIndirect(0xAB))); }
    #[test] pub fn can_decode_ora_zp()      { decoder_test(vec![0x05, 0xAB], Instruction::ORA(Operand::Absolute(0x00AB))); }
    #[test] pub fn can_decode_asl_zp()      { decoder_test(vec![0x06, 0xAB], Instruction::ASL(Operand::Absolute(0x00AB))); }
    #[test] pub fn can_decode_php()         { decoder_test(vec![0x08], Instruction::PHP); }
    #[test] pub fn can_decode_ora_imm()     { decoder_test(vec![0x09, 0xAB], Instruction::ORA(Operand::Immediate(0xAB))); }
    #[test] pub fn can_decode_asl_accum()   { decoder_test(vec![0x0A], Instruction::ASL(Operand::Accumulator)); }
    #[test] pub fn can_decode_ora_abs()     { decoder_test(vec![0x0D, 0xCD, 0xAB], Instruction::ORA(Operand::Absolute(0xABCD))); }
    #[test] pub fn can_decode_asl_abs()     { decoder_test(vec![0x0E, 0xCD, 0xAB], Instruction::ASL(Operand::Absolute(0xABCD))); }

    fn decoder_test(bytes: Vec<u8>, expected: Instruction) {
        let result = decode(&mut Cursor::new(bytes.as_slice()));
        assert_eq!(result, Ok(expected));
    }
}
