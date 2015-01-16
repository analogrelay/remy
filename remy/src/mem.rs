use std::num;
use std::rt::heap;
use std::ptr;
use std::intrinsics::offset;
use std::mem::size_of;

use Endianness;

#[derive(Show,PartialEq)]
pub enum MemoryError {
    AddressTooLarge,
    OutOfBounds
}

pub trait Memory<A: num::UnsignedInt> {
    fn get<I: num::Int>(&self, addr: A) -> Result<I, MemoryError>;
    fn set<I: num::Int>(&mut self, addr: A, val: I) -> Result<(), MemoryError>;
}

pub struct FixedMemory<A: num::UnsignedInt+num::ToPrimitive> {
    data: *mut u8,
    size: usize,
    endian: Endianness
}

impl<A: num::UnsignedInt+num::ToPrimitive> FixedMemory<A> {
    pub fn with_size(size: A) -> FixedMemory<A> {
        FixedMemory::with_size_and_endian(size, Endianness::native())
    }

    pub fn with_size_and_endian(size: A, endian: Endianness) -> FixedMemory<A> {
        unsafe {
            let buf = heap::allocate(num::NumCast::from(size).unwrap(), 0);
            ptr::zero_memory(buf, num::NumCast::from(size).unwrap());

            FixedMemory {
                data: buf,
                size: num::NumCast::from(size).unwrap(), // TODO: Figure out what to do with this error?
                endian: endian
            }
        }
    }

    fn to_mem_endian<I: num::Int>(&self, val: I) -> I {
        match self.endian {
            Endianness::BigEndian => val.to_be(),
            Endianness::LittleEndian => val.to_le()
        }
    }

    fn from_mem_endian<I: num::Int>(&self, val: I) -> I {
        match self.endian {
            Endianness::BigEndian => num::Int::from_be(val),
            Endianness::LittleEndian => num::Int::from_le(val)
        }
    }
}

impl<A: num::UnsignedInt+num::ToPrimitive> Memory<A> for FixedMemory<A> {
    fn get<I: num::Int>(&self, addr: A) -> Result<I, MemoryError> {
        // Get values into native ints
        let native_addr : usize = try!(num::NumCast::from(addr).ok_or(MemoryError::AddressTooLarge));

        if (native_addr >= self.size) || (native_addr + size_of::<I>() >= self.size) {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                let value_ptr = offset(self.data, native_addr as isize) as *const I;
                Ok(self.from_mem_endian(ptr::read(value_ptr)))
            }
        }
    }

    fn set<I: num::Int>(&mut self, addr: A, val: I) -> Result<(), MemoryError> {
        // Get values into native ints
        let native_addr : usize = try!(num::NumCast::from(addr).ok_or(MemoryError::AddressTooLarge));

        if (native_addr >= self.size) || (native_addr + size_of::<I>() >= self.size) {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                let value_ptr = offset(self.data, native_addr as isize) as *mut I;
                Ok(ptr::write(value_ptr, self.to_mem_endian(val)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    mod fixed_memory {
        use Endianness;
        use mem::{FixedMemory,Memory,MemoryError};

        #[test]
        pub fn can_read_and_write_u8_value() {
            let mut mem = FixedMemory::with_size_and_endian(10u16, Endianness::LittleEndian);
            assert!(mem.set(4, 42u8).is_ok());
            let val: u8 = mem.get(4).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn can_read_and_write_u16_value() {
            let mut mem = FixedMemory::with_size_and_endian(10u16, Endianness::LittleEndian);
            assert!(mem.set(4, 1024u16).is_ok());
            let val: u16 = mem.get(4).unwrap();
            assert_eq!(val, 1024);
        }

        #[test]
        pub fn can_read_and_write_u32_value() {
            let mut mem = FixedMemory::with_size_and_endian(10u16, Endianness::LittleEndian);
            assert!(mem.set(4, 75536u32).is_ok());
            let val : u32 = mem.get(4).unwrap();
            assert_eq!(val, 75536);
        }

        #[test]
        pub fn returns_error_if_writing_would_run_out_of_bounds() {
            let mut mem = FixedMemory::with_size_and_endian(10u16, Endianness::LittleEndian);
            assert_eq!(mem.set(9, 75535u32).unwrap_err(), MemoryError::OutOfBounds);
        }

        #[test]
        pub fn can_write_u32_and_read_as_u8_le() {
            let mut mem = FixedMemory::with_size_and_endian(10u16, Endianness::LittleEndian);
            assert!(mem.set(4, 75536u32).is_ok());
            let val1 : u8 = mem.get(4).unwrap();
            let val2 : u8 = mem.get(5).unwrap();
            let val3 : u8 = mem.get(6).unwrap();
            let val4 : u8 = mem.get(7).unwrap();
            assert_eq!(val1, 16);
            assert_eq!(val2, 39);
            assert_eq!(val3, 1);
            assert_eq!(val4, 0);
        }

        #[test]
        pub fn can_write_u32_and_read_as_u8_be() {
            let mut mem = FixedMemory::with_size_and_endian(10u16, Endianness::BigEndian);
            assert!(mem.set(4, 75536u32).is_ok());
            let val1 : u8 = mem.get(4).unwrap();
            let val2 : u8 = mem.get(5).unwrap();
            let val3 : u8 = mem.get(6).unwrap();
            let val4 : u8 = mem.get(7).unwrap();
            assert_eq!(val4, 16);
            assert_eq!(val3, 39);
            assert_eq!(val2, 1);
            assert_eq!(val1, 0);
        }
    }
}