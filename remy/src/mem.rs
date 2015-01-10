use std::num;
use std::rt::heap;
use std::ptr;
use std::intrinsics::offset;

#[derive(Show)]
pub enum MemoryError {
    OutOfBounds
}

pub trait Memory<A: num::Int> {
    fn get_u8(&self, addr: A) -> Result<u8, MemoryError>;
    fn set_u8(&mut self, addr: A, val: u8) -> Result<(), MemoryError>;

    fn get_u16(&self, addr: A) -> Result<u16, MemoryError>;
    fn set_u16(&mut self, addr: A, val: u16) -> Result<(), MemoryError>;

    fn get_u32(&self, addr: A) -> Result<u32, MemoryError>;
    fn set_u32(&mut self, addr: A, val: u32) -> Result<(), MemoryError>;
}

pub enum Endianness {
    BigEndian,
    LittleEndian
}

pub struct FixedMemory {
    data: *mut u8,
    size: u16,
    endian: Endianness
}

impl FixedMemory {
    fn with_size_and_endian(size: u16, endian: Endianness) -> FixedMemory {
        unsafe {
            let buf = heap::allocate(size as usize, 0);
            ptr::zero_memory(buf, size as usize);

            FixedMemory {
                data: buf,
                size: size,
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

impl Memory<u16> for FixedMemory {
    fn get_u8(&self, addr: u16) -> Result<u8, MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                let val = ptr::read(offset(self.data, addr as isize));
                Ok(self.from_mem_endian(val))
            }
        }
    }

    fn set_u8(&mut self, addr: u16, val: u8) -> Result<(), MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                ptr::write(offset(self.data, addr as isize) as *mut u8, self.to_mem_endian(val));
                Ok(())
            }
        }
    }

    fn get_u16(&self, addr: u16) -> Result<u16, MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                let val = ptr::read(offset(self.data, addr as isize) as *const u16);
                Ok(self.from_mem_endian(val))
            }
        }
    }

    fn set_u16(&mut self, addr: u16, val: u16) -> Result<(), MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                ptr::write(offset(self.data, addr as isize) as *mut u16, self.to_mem_endian(val));
                Ok(())
            }
        }
    }

    fn get_u32(&self, addr: u16) -> Result<u32, MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                let val = ptr::read(offset(self.data, addr as isize) as *const u32);
                Ok(self.from_mem_endian(val))
            }
        }
    }

    fn set_u32(&mut self, addr: u16, val: u32) -> Result<(), MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                ptr::write(offset(self.data, addr as isize) as *mut u32, self.to_mem_endian(val));
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    mod FixedMemory {
        use mem::{FixedMemory,Memory,Endianness};

        #[test]
        pub fn can_read_and_write_u8_value() {
            let mut mem = FixedMemory::with_size_and_endian(10, Endianness::LittleEndian);
            assert!(mem.set_u8(4, 42).is_ok());
            let val = mem.get_u8(4).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn can_read_and_write_u16_value() {
            let mut mem = FixedMemory::with_size_and_endian(10, Endianness::LittleEndian);
            assert!(mem.set_u16(4, 1024).is_ok());
            let val = mem.get_u16(4).unwrap();
            assert_eq!(val, 1024);
        }

        #[test]
        pub fn can_read_and_write_u32_value() {
            let mut mem = FixedMemory::with_size_and_endian(10, Endianness::LittleEndian);
            assert!(mem.set_u32(4, 75536).is_ok());
            let val = mem.get_u32(4).unwrap();
            assert_eq!(val, 75536);
        }

        #[test]
        pub fn can_write_u32_and_read_as_u8_le() {
            let mut mem = FixedMemory::with_size_and_endian(10, Endianness::LittleEndian);
            assert!(mem.set_u32(4, 75536).is_ok());
            let val1 = mem.get_u8(4).unwrap();
            let val2 = mem.get_u8(5).unwrap();
            let val3 = mem.get_u8(6).unwrap();
            let val4 = mem.get_u8(7).unwrap();
            assert_eq!(val1, 16);
            assert_eq!(val2, 39);
            assert_eq!(val3, 1);
            assert_eq!(val4, 0);
        }

        #[test]
        pub fn can_write_u32_and_read_as_u8_be() {
            let mut mem = FixedMemory::with_size_and_endian(10, Endianness::BigEndian);
            assert!(mem.set_u32(4, 75536).is_ok());
            let val1 = mem.get_u8(4).unwrap();
            let val2 = mem.get_u8(5).unwrap();
            let val3 = mem.get_u8(6).unwrap();
            let val4 = mem.get_u8(7).unwrap();
            assert_eq!(val4, 16);
            assert_eq!(val3, 39);
            assert_eq!(val2, 1);
            assert_eq!(val1, 0);
        }
    }
}