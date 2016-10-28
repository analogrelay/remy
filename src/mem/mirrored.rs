use mem;

/// Provides a `mem::Memory` that mirrors a provided `Memory` through a certain size
///
/// This is useful in systems like the NES where the 2KB of on-board RAM occupy addresses
/// 0x0000 through 0x2000 and simply repeat every 0x800 bytes. So a read or write to
/// 0x0042 is exactly the same as a read or write to 0x0842 or 0x1042 or 0x1842
pub struct Mirrored<M> where M: mem::Memory {
    mem: M,
    size: u64
}

impl<M> Mirrored<M> where M: mem::Memory {
    /// Creates a new `Mirrored` memory wrapping the provided memory and mirroring it through
    /// the `size` bytes.
    ///
    /// Addresses `0` through `size - 1` are valid and are translated to the effective address
    /// `eaddr` using the following expression: `eaddr = (addr % mem.len())`
    pub fn new(mem: M, size: u64) -> Mirrored<M> {
        Mirrored {
            mem: mem,
            size: size
        }
    }
}

impl<M> mem::Memory for Mirrored<M> where M: mem::Memory {
    fn len(&self) -> u64 { self.size }

    fn get_u8(&self, addr: u64) -> mem::Result<u8> {
        if addr >= self.size {
            Err(mem::Error::with_detail(
                mem::ErrorKind::OutOfBounds,
                "Read would reach end of memory",
                format!("attempted to read from 0x{:X}, but size is 0x{:x}", addr, self.size)))
        }
        else {
            let eaddr = addr % self.mem.len();
            self.mem.get_u8(eaddr)
        }
    }

    fn set_u8(&mut self, addr: u64, val: u8) -> mem::Result<()> {
        if addr >= self.size {
            Err(mem::Error::with_detail(
                mem::ErrorKind::OutOfBounds,
                "Write would reach end of memory",
                format!("attempted to write to 0x{:X}, but size is 0x{:x}", addr, self.size)))
        }
        else {
            let eaddr = addr % self.mem.len();
            self.mem.set_u8(eaddr, val)
        }
    }
}

#[cfg(test)]
mod test {
    use mem;
    use mem::Memory;

    #[test]
    pub fn can_read_and_write_inside_the_inner_memory_bounds() {
        let mut mem = mem::Mirrored::new(mem::Fixed::new(10), 10);
        let exp: [u8; 2] = [42, 24];
        let mut buf = [0; 2];

        mem.set(1, &exp).unwrap();
        mem.get(1, &mut buf).unwrap();
        assert_eq!(exp, buf);
    }

    #[test]
    pub fn reads_can_wrap_around() {
        let mut mem = mem::Mirrored::new(mem::Fixed::new(6), 18);
        let mut buf = [0; 6];

        mem.set(0, &[1, 2, 3, 4, 5, 6]).unwrap();
        mem.get(3, &mut buf).unwrap();
        assert_eq!([4, 5, 6, 1, 2, 3], buf);
    }

    #[test]
    pub fn writes_can_wrap_around() {
        let mut mem = mem::Mirrored::new(mem::Fixed::new(6), 18);
        let mut buf = [0; 6];

        mem.set(3, &[1, 2, 3, 4, 5, 6]).unwrap();
        mem.get(0, &mut buf).unwrap();
        assert_eq!([4, 5, 6, 1, 2, 3], buf);
    }

    #[test]
    pub fn reads_can_wrap_around_multiple_times() {
        let mut mem = mem::Mirrored::new(mem::Fixed::new(2), 6);
        let mut buf = [0; 6];

        mem.set(0, &[1, 2]).unwrap();
        mem.get(0, &mut buf).unwrap();
        assert_eq!([1, 2, 1, 2, 1, 2], buf);
    }

    #[test]
    pub fn writes_can_wrap_around_multiple_times() {
        let mut mem = mem::Mirrored::new(mem::Fixed::new(2), 6);
        let mut buf = [0; 6];

        mem.set(0, &[1, 2, 3, 4, 5, 6]).unwrap();
        mem.get(0, &mut buf).unwrap();
        assert_eq!([5, 6, 5, 6, 5, 6], buf);
    }

    #[test]
    pub fn reads_and_writes_that_start_out_of_bounds_produce_errors() {
        let mut mem = mem::Mirrored::new(mem::Fixed::new(10), 10);
        let mut buf = [0; 2];

        assert_eq!(mem::ErrorKind::OutOfBounds, mem.set(10, &buf).unwrap_err().kind);
        assert_eq!(mem::ErrorKind::OutOfBounds, mem.get(10, &mut buf).unwrap_err().kind);
    }

    #[test]
    pub fn reads_and_writes_that_end_out_of_bounds_produce_errors() {
        let mut mem = mem::Mirrored::new(mem::Fixed::new(10), 10);
        let mut buf = [0; 2];

        assert_eq!(mem::ErrorKind::OutOfBounds, mem.set(9, &buf).unwrap_err().kind);
        assert_eq!(mem::ErrorKind::OutOfBounds, mem.get(9, &mut buf).unwrap_err().kind);
    }
}
