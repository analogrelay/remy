use std::num;
use std::rt::heap;
use std::ptr;
use std::intrinsics::offset;

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

pub struct SixteenBitMemory {
    data: *mut u8,
    size: u16
}

impl SixteenBitMemory {
    fn with_size(size: u16) -> SixteenBitMemory {
        unsafe {
            let buf = heap::allocate(size as uint, 0);
            ptr::zero_memory(buf, size as uint);

            SixteenBitMemory {
                data: buf,
                size: size
            }
        }
    }
}

impl Memory<u16> for SixteenBitMemory {
    fn get_u8(&self, addr: u16) -> Result<u8, MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                Ok(ptr::read(offset(self.data, addr as int)))
            }
        }
    }

    fn set_u8(&mut self, addr: u16, val: u8) -> Result<(), MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                ptr::write(offset(self.data, addr as int) as *mut u8, val);
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
                Ok(ptr::read(offset(self.data, addr as int) as *const u16))
            }
        }
    }

    fn set_u16(&mut self, addr: u16, val: u16) -> Result<(), MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                ptr::write(offset(self.data, addr as int) as *mut u16, val);
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
                Ok(ptr::read(offset(self.data, addr as int) as *const u32))
            }
        }
    }

    fn set_u32(&mut self, addr: u16, val: u32) -> Result<(), MemoryError> {
        if addr >= self.size {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                ptr::write(offset(self.data, addr as int) as *mut u32, val);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    
}