use mem;
use std::cmp;

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
    pub fn new(mem: M, size: u64) -> Mirrored<M> {
        Mirrored {
            mem: mem,
            size: size
        }
    }
}

impl<M> mem::Memory for Mirrored<M> where M: mem::Memory {
    fn size(&self) -> u64 { self.size }

    fn get(&self, addr: u64, buf: &mut [u8]) -> mem::Result<()> {
        if addr >= self.size || (addr + buf.len() as u64) >= self.size {
            return Err(mem::Error::new(mem::ErrorKind::OutOfBounds, "attempted to read beyond the end of the memory"))
        }
        // Read chunks until we've read everything we're expected to read
        let mut ptr = 0;
        while ptr < buf.len() {
            // Determine the current effective address
            let eaddr = (addr + ptr as u64) % self.mem.size();

            // Determine how much we can read in a single burst
            let to_read = cmp::min((self.mem.size() - eaddr) as usize, buf.len() - ptr);

            // Read that much
            let inp = &mut buf[ptr .. (ptr + to_read)];
            if let Err(e) = self.mem.get(eaddr, inp) {
                return Err(e)
            }

            // Advance the pointer
            ptr = ptr + to_read;
        }

        Ok(())
    }

    fn set(&mut self, addr: u64, buf: &[u8]) -> mem::Result<()> {
        if addr >= self.size || (addr + buf.len() as u64) >= self.size {
            return Err(mem::Error::new(mem::ErrorKind::OutOfBounds, "attempted to write beyond the end of the memory"))
        }

        // Write chunks until we've written everything we're expected to write
        let mut ptr = 0;
        while ptr < buf.len() {
            // Determine the current effective address
            let eaddr = (addr + ptr as u64) % self.mem.size();

            // Determine how much we can write in a single burst
            let to_write = cmp::min((self.mem.size() - eaddr) as usize, buf.len() - ptr);

            // Write that much
            let outp = &buf[ptr .. (ptr + to_write)];
            if let Err(e) = self.mem.set(eaddr, outp) {
                return Err(e)
            }

            // Advance the pointer
            ptr = ptr + to_write;
        }

        Ok(())
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
}
