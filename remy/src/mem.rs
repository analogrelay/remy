use std::num;
use std::rt::heap;
use std::ptr;
use std::intrinsics::offset;
use std::mem::size_of;

use Endianness;

#[derive(Show,PartialEq)]
/// Represents an error that occurs when accessing a `Memory`
pub enum MemoryError {
    /// The provided address was outside the bounds of the memory
    OutOfBounds,

    /// The provided address referred to memory that is not readable
    MemoryNotReadable,

    /// The provided address referred to memory that is not writable
    MemoryNotWritable
}

/// Represents any memory accessible to a CPU
///
/// Implementations of this may use various sparse storage techniques to avoid
/// allocating the entire memory buffer, or may use ROM content from files to
/// back the memory. The memory may not have an address length longer than the
/// native word size on the host platform.
pub trait Memory {
    fn get<I: num::Int>(&self, addr: usize) -> Result<I, MemoryError>;
    fn set<I: num::Int>(&mut self, addr: usize, val: I) -> Result<(), MemoryError>;
}

/// Represents a flat fixed-size memory buffer
///
/// The memory buffer contained within can have any base address. Upon
/// initialization, a memory buffer will be allocated to hold all bytes in the memory
pub struct FixedMemory {
    data: *mut u8,
    size: usize,
    base_address: usize
}

impl FixedMemory {
    /// Initializes a new fixed memory of the specified size
    ///
    /// The initialized memory has a `base_address` of 0
    ///
    /// # Arguments
    ///
    /// * `size` - The size, in bytes, of the memory to create
    pub fn with_size(size: usize) -> FixedMemory {
        FixedMemory::with_size_and_base_address(size, 0)
    }

    /// Initializes a new fixed memory of the specified size
    ///
    /// The initialized memory has a `base_address` of 0
    ///
    /// # Arguments
    ///
    /// * `size` - The size, in bytes, of the memory to create
    pub fn with_size_and_endianness(size: usize, endian: Endianness) -> FixedMemory {
        FixedMemory::with_size_and_base_address(size, 0)
    }

    /// Initializes a new fixed memory of the specified size and base address
    ///
    /// The value of `base_address` is subtracted from the address provided in all
    /// operations on this memory. This way, the memory appears such that `base_address`
    /// is the lowest possible address in the memory.
    ///
    /// For example, if the base address of a memory was 0x0100 and `get(0x0101)` was called,
    /// the second byte (at offset `1`) would be read out of the buffer used to store this memory
    ///
    /// # Arguments
    ///
    /// * `size` - The size, in bytes, of the memory to create
    /// * `base_address` - The base address to subtract from all provided addresses
    pub fn with_size_and_base_address(size: usize, base_address: usize) -> FixedMemory {
        unsafe {
            let buf = heap::allocate(size, 0);
            ptr::zero_memory(buf, size);

            FixedMemory {
                data: buf,
                size: size,
                base_address: base_address
            }
        }
    }

    /// Initializes a new fixed memory of the specified size and base address
    ///
    /// This is just a short-hand for computing the necessary `size` and `base_address` values
    /// in the other initializers. The `size` of the memory will be `end - start`, and the base
    /// address will be `start`.
    ///
    /// # Arguments
    ///
    /// * `start` - The first address to be present in the memory
    /// * `end` - The final address to be present in the memory
    pub fn with_start_and_end(start: usize, end: usize) -> FixedMemory {
        FixedMemory::with_size_and_base_address(end - start, start)
    }

    fn translate_address(&self, addr: usize) -> Result<usize, MemoryError> {
        if addr < self.base_address {
            Err(MemoryError::OutOfBounds)
        }
        else {
            Ok(addr - self.base_address)
        }
    }
}

impl Memory for FixedMemory {
    /// Retrieves a value from the memory, using the endianness of the host architecture
    ///
    /// When reading multi-byte values, ensure you convert them to the target architecture's
    /// endianness after reading.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get<I: num::Int>(&self, addr: usize) -> Result<I, MemoryError> {
        // Translate the address
        let real_address = try!(self.translate_address(addr));
        if (real_address >= self.size) || (real_address + size_of::<I>() >= self.size) {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                let value_ptr = offset(self.data, real_address as isize) as *const I;
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
    fn set<I: num::Int>(&mut self, addr: usize, val: I) -> Result<(), MemoryError> {
        let real_address = try!(self.translate_address(addr));
        if (real_address >= self.size) || (real_address + size_of::<I>() >= self.size) {
            Err(MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                let value_ptr = offset(self.data, real_address as isize) as *mut I;
                Ok(ptr::write(value_ptr, val))
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
            let mut mem = FixedMemory::with_size(10);
            assert!(mem.set(4, 42u8).is_ok());
            let val: u8 = mem.get(4).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn can_read_and_write_u16_value() {
            let mut mem = FixedMemory::with_size(10);
            assert!(mem.set(4, 1024u16).is_ok());
            let val: u16 = mem.get(4).unwrap();
            assert_eq!(val, 1024);
        }

        #[test]
        pub fn can_read_and_write_u32_value() {
            let mut mem = FixedMemory::with_size(10);
            assert!(mem.set(4, 75536u32).is_ok());
            let val : u32 = mem.get(4).unwrap();
            assert_eq!(val, 75536);
        }

        #[test]
        pub fn returns_error_if_writing_would_run_out_of_bounds() {
            let mut mem = FixedMemory::with_size(10);
            assert_eq!(mem.set(9, 75535u32).unwrap_err(), MemoryError::OutOfBounds);
        }
    }
}