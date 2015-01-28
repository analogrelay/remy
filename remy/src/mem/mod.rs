use std::num;
use std::mem::size_of;

pub use mem::fixed::FixedMemory;
pub use mem::virt::VirtualMemory;

mod fixed;
mod virt;

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
/// back the memory. In the current implementation, the memory may not have an
/// address length longer than the native word size on the host platform.
pub trait Memory {
    fn size(&self) -> usize;
    fn get_u8(&self, addr: usize) -> Result<u8, MemoryError>;
    fn set_u8(&mut self, addr: usize, val: u8) -> Result<(), MemoryError>;
}

pub fn read<I: num::Int>(mem: &Memory, addr: usize) -> Result<I, MemoryError> {
    let size = size_of::<I>();

    // Load the number as a big-endian number (we can change it later)
    let val = range(0, size)
        .fold(Ok(0 as usize), |cur, idx| {
            match cur {
                Ok(x) => Ok((x << 8) | try!(mem.get_u8(addr + idx)) as usize),
                x => x
            }
        });

    // Convert to the expected value and return
    // We can unwrap here because we only read `size` bytes above
    val.map(|v| num::NumCast::from(v).unwrap())
}

pub fn write<I: num::Int+::std::fmt::String>(mem: &mut Memory, addr: usize, val: I) -> Result<(), MemoryError> {
    let size = size_of::<I>();

    // Write the number out as a big-endian number (the caller can ensure it is formatted as necessary for that)
    let result = range(0, size)
    	.fold(Ok(val), |cur, idx| {
    		match cur {
    			Ok(x) => {
    				// Take the right-most byte
    				let byte : u8 = num::NumCast::from(x & num::NumCast::from(0xFF).unwrap()).unwrap();

    				// Calculate the address for it
    				let byte_addr = (addr + (size - 1)) - idx;

    				println!("write {} [{}]: byte={},addr={}", val, idx, byte, byte_addr);

    				// Store the byte
    				try!(mem.set_u8(byte_addr, byte));

    				// Shift the input and continue
    				Ok(x >> 8)
    			},
    			x => x
    		}
    	});

    match result {
    	Ok(_) => Ok(()),
    	Err(e) => Err(e)
    }
}