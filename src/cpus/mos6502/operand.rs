use std::{error,fmt};

use mem;
use mem::{Memory,MemoryExt};

use cpus::mos6502::cpu;
use cpus::mos6502::Mos6502;

/// Represents an operand that can be provided to an instruction
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum Operand {
    /// Indicates an operand provided inline with the instruction
    Immediate(u8),
    /// Indicates an operand stored in the Accumulator (A register) 
    Accumulator,
    /// Indicates an operand stored at the provided memory address
    ///
    /// If the provided address is `m`, this operand is defined as `*m`
    Absolute(u16),
    /// Indicates an operand stored at the provided index from the current value of the provided
    /// register
    ///
    /// If the provided address is `m`, this operand is defined as `*(m+x)` or `*(m+y)` depending
    /// on the register specified
    Indexed(u16, cpu::RegisterName),
    /// Indicates an operand stored at an address stored in the provided address
    ///
    /// If the provided address is `m`, this operand is defined as `**m`
    Indirect(u16),
    /// Indicates an operand stored at an address stored in the provided address (indexed by the
    /// `X` register)
    ///
    /// If the provided address is `m`, this operand is defined as `**(m+x)`
    PreIndexedIndirect(u8),
    /// Indicates an operand stored at an address (indexed by the `Y` register) stored in the provided address
    ///
    /// If the provided address is `x`, this operand is defined as `*(*m+y)`
    PostIndexedIndirect(u8),

    /// Indicates an operand stored as an offset to the program counter
    ///
    /// This variant is never used as a general-purpose operand, it is
    /// always tested for explicitly and unwrapped
    Offset(i8),
    /// Indicates an operand stored as a 16-bit immediate value
    ///
    /// This variant is never used as a general-purpose operand, it is
    /// always tested for explicitly and unwrapped
    TwoByteImmediate(u16)
}

/// Represents an error that occurred which accessing an `Operand`
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Error {
    /// Indicates an error occurred reading or writing memory
    ErrorAccessingMemory(mem::Error),
    /// Indicates that a request was made to write to a read-only operand such as
    /// `Operand::Immediate`
    ReadOnlyOperand,
    /// Indicates that a request was made to take the address of a non-addressable operand
    /// such as `Operand::Immediate`
    NonAddressOperand
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::ErrorAccessingMemory(_) => "error accessing memory",
            &Error::ReadOnlyOperand         => "attempted to write to a read-only operand",
            &Error::NonAddressOperand       => "attempted to take the address of an operand with no address"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::ErrorAccessingMemory(ref err) => Some(err),
            _                                     => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let &Error::ErrorAccessingMemory(ref err) = self {
            write!(fmt, "error accessing memory: {}", err)
        } else {
            error::Error::description(self).fmt(fmt)
        }
    }
}

impl From<mem::Error> for Error {
    fn from(err: mem::Error) -> Error {
        Error::ErrorAccessingMemory(err)
    }
}

impl Operand {
    /// Retrieves the operand value
    ///
    /// # Arguments
    ///
    /// * `cpu` - The cpu from which to get the operand value
    pub fn get_u8<M>(&self, cpu: &mut Mos6502, mem: &M) -> Result<u8> where M: mem::Memory {
        Ok(match self {
            &Operand::Immediate(n)      => n,
            &Operand::Accumulator       => cpu.registers.a,
            _                           => try!(mem.get_u8(try!(self.get_addr(cpu, mem)) as u64))
        })
    }

    /// Returns true if the operand has an address (aka, it is an 'lvalue')
    pub fn has_addr(&self) -> bool {
        match self {
            &Operand::Immediate(_) |
                &Operand::Accumulator |
                &Operand::TwoByteImmediate(_) => false,
            _ => true
        }
    }

    /// Sets the value of the operand on the specified cpu
    ///
    /// # Arguments
    ///
    /// * `cpu` - The cpu on which to set the operand value
    /// * `val` - The value to set the operand to
    pub fn set_u8<M>(&self, cpu: &mut Mos6502, mem: &mut M, val: u8) -> Result<()> where M: mem::Memory {
        match self {
            &Operand::Accumulator        => { cpu.registers.a = val; Ok(()) },
            _                            => { let addr = try!(self.get_addr(cpu, mem)) as u64; Ok(try!(mem.set_u8(addr, val))) }
        }
    }

    /// Retrieves the address of the operand on the specified cpu
    ///
    /// # Arguments
    ///
    /// * `cpu` - The cpu on which to get the operand value
    pub fn get_addr<M>(&self, cpu: &mut Mos6502, mem: &M) -> Result<u16> where M: mem::Memory {
        match self.get_addr_impl(cpu, mem) {
            Ok((addr, true)) => { cpu.clock.tick(1); Ok(addr) },
            Ok((addr, false)) => Ok(addr),
            Err(e) => Err(e)
        }
    }

    /// Get a string in the form of the nestest "golden log" output
    pub fn get_log_string<M>(&self, cpu: &Mos6502, mem: &M) -> Result<String> where M: mem::Memory {
        Ok(match self {
            &Operand::Offset(offset) => format!("${:04X}", ((cpu.pc.get() as i32) + (offset as i32)) as u16),
            &Operand::PreIndexedIndirect(addr) => {
                let (preindex_addr, _) = try!(self.get_addr_impl(cpu, mem));
                let eaddr = (addr as u64 + cpu.registers.x as u64) & 0xFF;
                format!("{} @ {:02X} = {:04X} = {:02X}", self, eaddr, preindex_addr, try!(self.get_u8_noclock(cpu, mem)))
            },
            &Operand::PostIndexedIndirect(addr) => {
                let (preindex_addr, _) = try!(self.get_addr_impl(cpu, mem));
                let low = try!(mem.get_u8(addr as u64)) as u64;
                let high = try!(mem.get_u8((addr as u64 + 1) & 0xFF)) as u64;
                let eaddr = low | (high << 8);
                format!("{} = {:04X} @ {:04X} = {:02X}", self, eaddr, preindex_addr, try!(self.get_u8_noclock(cpu, mem)))
            },
            &Operand::Indexed(addr, _) => {
                let (preindex_addr, _) = try!(self.get_addr_impl(cpu, mem));
                if addr < 0x0100 {
                    format!("{} @ {:02X} = {:02X}", self, preindex_addr as u8, try!(self.get_u8_noclock(cpu, mem)))
                } else {
                    format!("{} @ {:04X} = {:02X}", self, preindex_addr, try!(self.get_u8_noclock(cpu, mem)))
                }
            },
            &op if op.has_addr() => {
                let (addr, _) = try!(op.get_addr_impl(cpu, mem));
                let value = try!(mem.get_u8(addr as u64));
                format!("{} = {:02X}", op, value)
            },
            &op => format!("{}", op),
        })
    }

    fn get_u8_noclock<M>(&self, cpu: &Mos6502, mem: &M) -> Result<u8> where M: mem::Memory {
        Ok(match self {
            &Operand::Immediate(n)      => n,
            &Operand::Accumulator       => cpu.registers.a,
            _                           => {
                let (addr, _) = try!(self.get_addr_impl(cpu, mem));
                try!(mem.get_u8(addr as u64))
            }
        })
    }

    fn get_addr_impl<M>(&self, cpu: &Mos6502, mem: &M) -> Result<(u16,bool)> where M: mem::Memory {
        Ok(match self {
            &Operand::Absolute(addr)             => (addr, false),
            &Operand::Indirect(addr)             => {
                // Indirect accesses can't leave the page, they wrap around
                let low = try!(mem.get_u8(addr as u64)) as u64;
                let high = try!(mem.get_u8((addr as u64 & 0xFF00) | ((addr as u64 + 1) & 0x00FF))) as u64;
                ((low | (high << 8)) as u16, false)
            },
            &Operand::Indexed(addr, r)           => {
                let mut eaddr = addr as u64 + r.get(cpu) as u64;
                if addr < 0x0100 {
                    // Zero-page accesses can't leave the zero page, they wrap around
                    eaddr = eaddr & 0xFF;
                } else {
                    eaddr = eaddr & 0xFFFF;
                }
                (eaddr as u16, oops_cycle(addr as u64, eaddr))
            },
            &Operand::PreIndexedIndirect(addr)   => {
                // Indirect accesses can't leave the zero page, they wrap around
                let mut eaddr = (addr as u64 + cpu.registers.x as u64) & 0xFF;
                let low = try!(mem.get_u8(eaddr)) as u16;
                eaddr = (eaddr + 1) & 0xFF;
                let high = try!(mem.get_u8(eaddr)) as u16;
                ((high << 8) | low, false)
            },
            &Operand::PostIndexedIndirect(addr)  => {
                // Indirect accesses can't leave the page, they wrap around
                let low = try!(mem.get_u8(addr as u64)) as u64;
                let high = try!(mem.get_u8((addr as u64 + 1) & 0xFF)) as u64;

                let original_addr = low | (high << 8);
                let final_addr = original_addr + cpu.registers.y as u64;
                ((final_addr & 0xFFFF) as u16, oops_cycle(original_addr, final_addr))
            },
            _                                   => return Err(Error::NonAddressOperand)
        })
}

}

fn oops_cycle(original_addr: u64, actual_addr: u64) -> bool {
    (original_addr & 0xFF00) != (actual_addr & 0xFF00)
}

impl fmt::Display for Operand {
    /// Returns a string representing the instruction
    fn fmt(&self, formatter: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match self {
            &Operand::Immediate(val)             => write!(formatter, "#${:02X}", val),
            &Operand::Accumulator                => formatter.write_str("A"),
            &Operand::Absolute(val)              =>
                if val <= 0x00FF {
                    write!(formatter, "${:02X}", val)
                } else {
                    write!(formatter, "${:04X}", val)
                },
            &Operand::Indexed(val, reg)          =>
                if val <= 0x00FF {
                    write!(formatter, "${:02X},{}", val, reg)
                } else {
                    write!(formatter, "${:04X},{}", val, reg)
                },
            &Operand::Indirect(val)              => write!(formatter, "(${:04X})", val),
            &Operand::PreIndexedIndirect(val)    => write!(formatter, "(${:02X},X)", val),
            &Operand::PostIndexedIndirect(val)   => write!(formatter, "(${:02X}),Y", val),
            &Operand::Offset(val)                => write!(formatter, "{}${:02X}", if val < 0 { "-" } else { "" }, val),
            &Operand::TwoByteImmediate(val)      => write!(formatter, "${:04X}", val),
        }
    }
}

#[cfg(test)]
mod test {
    mod operand {
        use mem;
        use mem::MemoryExt;
        use cpus::mos6502::{cpu,Mos6502,Operand};
        use byteorder::LittleEndian;

        #[test]
        pub fn to_string_test() {
            assert_eq!("#$AB", Operand::Immediate(0xAB).to_string());
            assert_eq!("A", Operand::Accumulator.to_string());
            assert_eq!("$ABCD", Operand::Absolute(0xABCD).to_string());
            assert_eq!("$0BCD", Operand::Absolute(0x0BCD).to_string());
            assert_eq!("$AB", Operand::Absolute(0x00AB).to_string());
            assert_eq!("$ABCD,X", Operand::Indexed(0xABCD, cpu::RegisterName::X).to_string());
            assert_eq!("$ABCD,Y", Operand::Indexed(0xABCD, cpu::RegisterName::Y).to_string());
            assert_eq!("$0BCD,X", Operand::Indexed(0xBCD, cpu::RegisterName::X).to_string());
            assert_eq!("$0BCD,Y", Operand::Indexed(0xBCD, cpu::RegisterName::Y).to_string());
            assert_eq!("$AB,X", Operand::Indexed(0x00AB, cpu::RegisterName::X).to_string());
            assert_eq!("$AB,Y", Operand::Indexed(0x00AB, cpu::RegisterName::Y).to_string());
            assert_eq!("($ABCD)", Operand::Indirect(0xABCD).to_string());
            assert_eq!("($AB,X)", Operand::PreIndexedIndirect(0xAB).to_string());
            assert_eq!("($AB),Y", Operand::PostIndexedIndirect(0xAB).to_string());
        }

        #[test]
        pub fn set_absolute_puts_value_in_memory_location() {
            let mut mem = mem::Fixed::new(10);
            let mut cpu = Mos6502::new();
            assert!(Operand::Absolute(2).set_u8(&mut cpu, &mut mem, 24).is_ok());
            let val = mem.get_u8(2).unwrap();
            assert_eq!(val, 24);
        }

        #[test]
        pub fn set_indexed_x_puts_value_in_memory_location() {
            let mut mem = mem::Fixed::new(10);
            let mut cpu = Mos6502::new();
            cpu.registers.x = 1;
            assert!(Operand::Indexed(2, cpu::RegisterName::X).set_u8(&mut cpu, &mut mem, 24).is_ok());
            let val = mem.get_u8(3).unwrap();
            assert_eq!(val, 24);
        }

        #[test]
        pub fn set_indexed_y_puts_value_in_memory_location() {
            let mut mem = mem::Fixed::new(10);
            let mut cpu = Mos6502::new();
            cpu.registers.y = 1;
            assert!(Operand::Indexed(2, cpu::RegisterName::Y).set_u8(&mut cpu, &mut mem, 24).is_ok());
            let val = mem.get_u8(3).unwrap();
            assert_eq!(val, 24);
        }

        #[test]
        pub fn set_accumulator_puts_value_in_accumulator() {
            let mut mem = mem::Fixed::new(10);
            let mut cpu = Mos6502::new();
            cpu.registers.a = 24;
            Operand::Accumulator.set_u8(&mut cpu, &mut mem, 42).unwrap();
            assert_eq!(cpu.registers.a, 42);
        }

        #[test]
        pub fn get_accumulator_returns_value_from_accumulator() {
            let mut cpu = Mos6502::new();
            cpu.registers.a = 42;
            assert_eq!(Ok(42), Operand::Accumulator.get_u8(&mut cpu, &mem::Empty));
        }

        #[test]
        pub fn get_immediate_returns_immediate_value() {
            let mut cpu = Mos6502::new();
            let val = Operand::Immediate(42).get_u8(&mut cpu, &mem::Empty).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_absolute_returns_value_from_memory_address() {
            let mut mem = mem::Fixed::new(10);
            let mut cpu = Mos6502::new();
            assert!(mem.set_u8(4, 42).is_ok());
            let val = Operand::Absolute(4).get_u8(&mut cpu, &mem).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_indexed_x_adds_x_to_address() {
            let mut mem = mem::Fixed::new(10);
            let mut cpu = Mos6502::new();
            assert!(mem.set_u8(4, 42).is_ok());
            cpu.registers.x = 2;
            let val = Operand::Indexed(2, cpu::RegisterName::X).get_u8(&mut cpu, &mem).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_indexed_y_adds_y_to_address() {
            let mut mem = mem::Fixed::new(10);
            let mut cpu = Mos6502::new();
            assert!(mem.set_u8(4, 42).is_ok());
            cpu.registers.y = 2;
            let val = Operand::Indexed(2, cpu::RegisterName::Y).get_u8(&mut cpu, &mem).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_preindexed_indirect_works() {
            let mut mem = mem::Fixed::new(10);
            let mut cpu = Mos6502::new();
            assert!(mem.set_u8(8, 42).is_ok()); // Value
            assert!(mem.set_u16::<LittleEndian>(6, 8).is_ok()); // Indirect Memory Address
            cpu.registers.x = 2;
            let val = Operand::PreIndexedIndirect(4).get_u8(&mut cpu, &mem).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_postindexed_indirect_works() {
            let mut mem = mem::Fixed::new(10);
            let mut cpu = Mos6502::new();
            assert!(mem.set_u8(8, 42).is_ok()); // Value
            assert!(mem.set_u16::<LittleEndian>(2, 6).is_ok()); // Indirect Memory Address
            cpu.registers.y = 2;
            let val = Operand::PostIndexedIndirect(2).get_u8(&mut cpu, &mem).unwrap();
            assert_eq!(val, 42);
        }
    }
}
