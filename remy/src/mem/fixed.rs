use std::rt::heap;
use std::ptr;
use std::intrinsics::offset;
use std::u8;

use mem;

/// Represents a flat fixed-size memory buffer
///
/// Upon initialization, a memory buffer will be allocated to hold all bytes in the memory
pub struct FixedMemory {
    data: *mut u8,
    size: usize
}

impl FixedMemory {
    /// Initializes a new fixed memory of the specified size
    ///
    /// # Arguments
    ///
    /// * `size` - The size, in bytes, of the memory to create
    pub fn with_size(size: usize) -> FixedMemory {
        unsafe {
            let buf = heap::allocate(size, 0);
            ptr::zero_memory(buf, size);

            FixedMemory {
                data: buf,
                size: size,
            }
        }
    }
}

impl mem::Memory for FixedMemory {
    /// Retrieves the size of the memory.
    fn size(&self) -> usize {
        self.size
    }

    /// Retrieves a value from the memory, using the endianness of the host architecture
    ///
    /// When reading multi-byte values, ensure you convert them to the target architecture's
    /// endianness after reading.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_u8(&self, addr: usize) -> Result<u8, mem::MemoryError> {
        // Translate the address
        if (addr >= self.size) {
            Err(mem::MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                let value_ptr = offset(self.data, addr as isize) as *const u8;
                Ok(ptr::read(value_ptr))
            }
        }
    }

    /// Stores a value in the memory, using the endianness of the host architecture
    ///
    /// When storing multi-byte values, ensure you convert them from the target architecture's
    /// endianness to the host architecture's endianness before storing.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to write to
    /// * `val` - The value to write to the memory
    fn set_u8(&mut self, addr: usize, val: u8) -> Result<(), mem::MemoryError> {
        if (addr >= self.size) {
            Err(mem::MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                let value_ptr = offset(self.data, addr as isize) as *mut u8;
                Ok(ptr::write(value_ptr, val))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use mem;
    use mem::{FixedMemory,Memory,MemoryError};

    #[test]
    pub fn can_read_and_write_u8_value() {
        let mut fm = FixedMemory::with_size(10);
        fm.set_u8(4, 42u8).unwrap();
        let val: u8 = fm.get_u8(4).unwrap();
        assert_eq!(val, 42);
    }
}