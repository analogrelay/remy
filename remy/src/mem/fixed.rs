use mem;
use std::convert;
use std::slice::bytes;

/// Represents a flat fixed-size memory buffer
///
/// Upon initialization, a memory buffer will be allocated to hold all bytes in the memory
pub struct Fixed {
    data: Vec<u8>
}

impl Fixed {
    /// Initializes a new fixed memory of the specified size
    ///
    /// # Arguments
    ///
    /// * `size` - The size, in bytes, of the memory to create
    pub fn new(size: usize) -> Fixed {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(0);
        }
        Fixed {
            data: data
        }
    }

    /// Initializes a new fixed memory with the specified contents
    ///
    /// # Arguments
    ///
    /// * `contents` - The contents to initialize the memory with
    pub fn from_contents<T>(contents: T) -> Fixed where T: convert::Into<Vec<u8>> {
        Fixed {
            data: contents.into()
        }
    }
}

impl mem::Memory for Fixed {
    /// Retrieves the size of the memory.
    fn len(&self) -> u64 {
        self.data.len() as u64
    }

    /// Fills the provided buffer with data from the memory starting at the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address at which to start reading
    /// * `buf` - The buffer to fill
    fn get(&self, addr: u64, buf: &mut [u8]) -> mem::Result<()> {
        let end = (addr as usize + buf.len() - 1) as usize;
        if end >= self.data.len() {
            // The read will take us out of bounds, don't start it.
            Err(mem::Error::with_detail(
                mem::ErrorKind::OutOfBounds,
                "Read would reach end of memory",
                format!("requested: 0x{:X} - 0x{:X}, but size is 0x{:x}", addr, end, self.data.len())))
        }
        else {
            let start = addr as usize;
            bytes::copy_memory(&self.data[start .. end + 1], buf);
            Ok(())
        }
    }

    /// Writes the provided buffer to the memory starting at the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address at which to start writing
    /// * `buf` - The buffer to write
    fn set(&mut self, addr: u64, buf: &[u8]) -> mem::Result<()> {
        let end = (addr as usize + buf.len() - 1) as usize;
        if end >= self.data.len() {
            Err(mem::Error::with_detail(
                mem::ErrorKind::OutOfBounds,
                "Write would reach end of memory",
                format!("requested: 0x{:X} - 0x{:X}, but size is 0x{:x}", addr, end, self.data.len())))
        } else {
            let start = addr as usize;
            bytes::copy_memory(buf, &mut self.data[start .. end + 1]);
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use mem;
    use mem::{Memory,MemoryExt};

    #[test]
    pub fn get_and_set_work() {
        let mut mem = mem::Fixed::new(10);
        mem.set(1, &[42, 24, 44, 22]).unwrap();

        let mut buf = [0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();

        assert_eq!([42, 24, 44, 22], buf);
    }

    #[test]
    pub fn get_returns_err_if_would_go_out_of_bounds() {
        let mem = mem::Fixed::new(10);
        let mut buf = [0, 0, 0, 0];
        assert_eq!(mem.get(8, &mut buf).unwrap_err().kind, mem::ErrorKind::OutOfBounds);
    }

    #[test]
    pub fn get_does_not_fill_buffer_if_read_would_go_out_of_bounds() {
        let mut mem = mem::Fixed::new(10);
        mem.set(8, &[42]).unwrap();
        let mut buf = [0, 0, 0, 0];
        mem.get(8, &mut buf).unwrap_err();
        assert_eq!([0, 0, 0, 0], buf);
    }

    #[test]
    pub fn set_returns_err_if_would_go_out_of_bounds() {
        let mut mem = mem::Fixed::new(10);
        assert_eq!(mem.set(8, &[42, 24, 44, 22]).unwrap_err().kind, mem::ErrorKind::OutOfBounds);
    }

    #[test]
    pub fn set_does_not_write_anything_unless_whole_write_fits() {
        let mut mem = mem::Fixed::new(10);
        mem.set(8, &[42, 24, 44, 22]).unwrap_err();

        assert_eq!(0, mem.get_u8(8).unwrap());
        assert_eq!(0, mem.get_u8(9).unwrap());
    }
}
