use std::rt::heap;
use std::ptr;
use std::intrinsics::offset;

use mem;

/// Represents a flat fixed-size memory buffer
///
/// Upon initialization, a memory buffer will be allocated to hold all bytes in the memory
#[derive(Copy)]
#[allow(raw_pointer_derive)]
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

    fn get(&self, addr: usize, buf: &mut [u8]) -> mem::MemoryResult<()> {
        if addr + (buf.len() - 1) >= self.size {
            // The read will take us out of bounds, don't start it.
            Err(mem::MemoryError::OutOfBounds)
        }
        else {
            unsafe {
                for idx in 0..buf.len() {
                    let value_ptr = offset(self.data, (addr + idx) as isize) as *const u8;
                    buf[idx] = ptr::read(value_ptr);
                }
            }
            Ok(())
        }
    }

    fn set(&mut self, addr: usize, buf: &[u8]) -> mem::MemoryResult<()> {
        if addr + (buf.len() - 1) >= self.size {
            Err(mem::MemoryError::OutOfBounds)
        } else {
            unsafe {
                for idx in 0..buf.len() {
                   let value_ptr = offset(self.data, (addr + idx) as isize) as *mut u8;
                   ptr::write(value_ptr, buf[idx])
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use mem::{FixedMemory,Memory,MemoryError};

    #[test]
    pub fn get_and_set_work() {
        let mut mem = FixedMemory::with_size(10);
        mem.set(1, &[42, 24, 44, 22]).unwrap();

        let mut buf = [0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();

        assert_eq!([42, 24, 44, 22], buf);
    }

    #[test]
    pub fn get_returns_err_if_would_go_out_of_bounds() {
        let mem = FixedMemory::with_size(10);
        let mut buf = [0, 0, 0, 0];
        assert_eq!(mem.get(8, &mut buf), Err(MemoryError::OutOfBounds));
    }

    #[test]
    pub fn get_does_not_fill_buffer_if_read_would_go_out_of_bounds() {
        let mut mem = FixedMemory::with_size(10);
        mem.set(8, &[42]).unwrap();
        let mut buf = [0, 0, 0, 0];
        mem.get(8, &mut buf).unwrap_err();
        assert_eq!([0, 0, 0, 0], buf);
    }

    #[test]
    pub fn set_returns_err_if_would_go_out_of_bounds() {
        let mut mem = FixedMemory::with_size(10);
        assert_eq!(mem.set(8, &[42, 24, 44, 22]), Err(MemoryError::OutOfBounds));
    }

    #[test]
    pub fn set_does_not_write_anything_unless_whole_write_fits() {
        let mut mem = FixedMemory::with_size(10);
        mem.set(8, &[42, 24, 44, 22]).unwrap_err();

        assert_eq!(0, mem.get_u8(8).unwrap());
        assert_eq!(0, mem.get_u8(9).unwrap());
    }
}