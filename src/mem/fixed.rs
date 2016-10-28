use mem;
use std::convert;

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

    fn get_u8(&self, addr: u64) -> mem::Result<u8> {
        if addr >= self.data.len() as u64 {
            Err(mem::Error::with_detail(
                mem::ErrorKind::OutOfBounds,
                "Read would reach end of memory",
                format!("attempted to read from 0x{:X}, but size is 0x{:x}", addr, self.data.len())))
        }
        else {
            Ok(self.data[addr as usize])
        }
    }

    fn set_u8(&mut self, addr: u64, val: u8) -> mem::Result<()> {
        if addr >= self.data.len() as u64 {
            Err(mem::Error::with_detail(
                mem::ErrorKind::OutOfBounds,
                "Write would reach end of memory",
                format!("attempted to write to 0x{:X}, but size is 0x{:x}", addr, self.data.len())))
        } else {
            self.data[addr as usize] = val;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use mem;
    use mem::Memory;

    #[test]
    pub fn get_and_set_work() {
        let mut mem = mem::Fixed::new(10);
        mem.set_u8(1, 42).ok().expect("set failed");
        assert_eq!(Ok(42), mem.get_u8(1));
    }

    #[test]
    pub fn get_returns_err_if_out_of_bounds() {
        let mem = mem::Fixed::new(10);
        assert_eq!(mem::ErrorKind::OutOfBounds, mem.get_u8(12).unwrap_err().kind);
    }

    #[test]
    pub fn set_returns_err_if_out_of_bounds() {
        let mut mem = mem::Fixed::new(10);
        assert_eq!(mem::ErrorKind::OutOfBounds, mem.set_u8(12, 42).unwrap_err().kind);
    }
}
