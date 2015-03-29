use std::rt::heap;
use std::ptr;
use std::intrinsics::offset;

use mem;

/// Represents a flat fixed-size memory buffer
///
/// Upon initialization, a memory buffer will be allocated to hold all bytes in the memory
#[allow(missing_copy_implementations)]
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
    pub fn new(size: usize) -> FixedMemory {
        unsafe {
            let buf = heap::allocate(size, 0);
            ptr::write_bytes(buf, 0, size);

            FixedMemory::from_raw_parts(buf, size)
        }
    }

    /// Initializes a fixed memory from the provided buffer
    ///
    /// # Arguments
    ///
    /// * `buf` - A pointer to the first element in the memory buffer
    /// * `size` - The size of the buffer in bytes
    pub unsafe fn from_raw_parts(buf: *mut u8, size: usize) -> FixedMemory {
        FixedMemory {
            data: buf,
            size: size
        }
    }
}

impl Clone for FixedMemory {
    fn clone(&self) -> FixedMemory {
        unsafe {
            let buf = heap::allocate(self.size, 0);
            ptr::copy_nonoverlapping(buf, self.data, self.size);
            FixedMemory::from_raw_parts(buf, self.size)
        }
    }
}

impl mem::Memory for FixedMemory {
    /// Retrieves the size of the memory.
    fn size(&self) -> usize {
        self.size
    }

    /// Fills the provided buffer with data from the memory starting at the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address at which to start reading
    /// * `buf` - The buffer to fill
    fn get(&self, addr: usize, buf: &mut [u8]) -> mem::Result<()> {
        let end = addr + buf.len() - 1;
        if end >= self.size {
            // The read will take us out of bounds, don't start it.
            Err(mem::Error::with_detail(
                mem::ErrorKind::OutOfBounds,
                "Read would reach end of memory",
                format!("requested: 0x{:X} - 0x{:X}, but size is 0x{:x}", addr, end, self.size)))
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

    /// Writes the provided buffer to the memory starting at the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address at which to start writing
    /// * `buf` - The buffer to write
    fn set(&mut self, addr: usize, buf: &[u8]) -> mem::Result<()> {
        let end = addr + buf.len() - 1;
        if end >= self.size {
            Err(mem::Error::with_detail(
                mem::ErrorKind::OutOfBounds,
                "Write would reach end of memory",
                format!("requested: 0x{:X} - 0x{:X}, but size is 0x{:x}", addr, end, self.size)))
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
    use mem::{FixedMemory,Memory,MemoryErrorKind};

    #[test]
    pub fn get_and_set_work() {
        let mut mem = FixedMemory::new(10);
        mem.set(1, &[42, 24, 44, 22]).unwrap();

        let mut buf = [0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();

        assert_eq!([42, 24, 44, 22], buf);
    }

    #[test]
    pub fn get_returns_err_if_would_go_out_of_bounds() {
        let mem = FixedMemory::new(10);
        let mut buf = [0, 0, 0, 0];
        assert_eq!(mem.get(8, &mut buf).unwrap_err().kind, MemoryErrorKind::OutOfBounds);
    }

    #[test]
    pub fn get_does_not_fill_buffer_if_read_would_go_out_of_bounds() {
        let mut mem = FixedMemory::new(10);
        mem.set(8, &[42]).unwrap();
        let mut buf = [0, 0, 0, 0];
        mem.get(8, &mut buf).unwrap_err();
        assert_eq!([0, 0, 0, 0], buf);
    }

    #[test]
    pub fn set_returns_err_if_would_go_out_of_bounds() {
        let mut mem = FixedMemory::new(10);
        assert_eq!(mem.set(8, &[42, 24, 44, 22]).unwrap_err().kind, MemoryErrorKind::OutOfBounds);
    }

    #[test]
    pub fn set_does_not_write_anything_unless_whole_write_fits() {
        let mut mem = FixedMemory::new(10);
        mem.set(8, &[42, 24, 44, 22]).unwrap_err();

        assert_eq!(0, mem.get_u8(8).unwrap());
        assert_eq!(0, mem.get_u8(9).unwrap());
    }
}
