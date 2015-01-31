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

#[derive(Copy)]
/// Describes the [endianness](http://en.wikipedia.org/wiki/Endianness) of a machine
pub enum Endianness {
	/// The machine is big-endian. Higher-order bytes of multi-byte numbers are stored first in memory
    BigEndian,

    /// The machine is little-endian. Lower-order bytes of multi-byte numbers are stored first in memory
    LittleEndian
}

impl Endianness {
	/// Gets an `Endianness` representing the host machine endianness.
	pub fn host() -> Endianness {
		if cfg!(target_endian = "big") {
			Endianness::BigEndian
		} else {
			Endianness::LittleEndian
		}
	}
}

/// Represents any memory accessible to a CPU
///
/// Implementations of this may use various sparse storage techniques to avoid
/// allocating the entire memory buffer, or may use ROM content from files to
/// back the memory. In the current implementation, the memory may not have an
/// address length longer than the native word size on the host platform.
pub trait Memory {
	/// Gets the size of the memory
    fn size(&self) -> usize;

    /// Reads a single byte from the specified address
    fn get_u8(&self, addr: usize) -> Result<u8, MemoryError>;

    /// Writes a single byte to the specified address
    fn set_u8(&mut self, addr: usize, val: u8) -> Result<(), MemoryError>;
}

/// Represents a view over a memory object that allows values larger than a single byte to
/// be easily read and written.
///
/// The view is configured with a specific `Endianness` in order to allow it to convert multi-byte
/// values from the host machine endianness to the expected target machine endianness
pub struct MemoryView<M: Memory> {
	endianness: Endianness,
	memory: M
}

impl<M: Memory> MemoryView<M> {
	/// Creates a new `MemoryView` given the specified `Endianness` and `Memory`
	pub fn new(endianness: Endianness, memory: M) -> MemoryView<M> {
		MemoryView {
			endianness: endianness,
			memory: memory
		}
	}

	/// Creates a new `MemoryView` backed by the specified `Memory` which reads multi-byte
	/// integers in little-endian format
	pub fn little_endian(memory: M) -> MemoryView<M> {
		MemoryView::new(Endianness::LittleEndian, memory)
	}

	/// Creates a new `MemoryView` backed by the specified `Memory` which reads multi-byte
	/// integers in big-endian format
	pub fn big_endian(memory: M) -> MemoryView<M> {
		MemoryView::new(Endianness::BigEndian, memory)
	}

	/// Reads a (possibly multi-byte) integer (of type `I`) from the underlying memory.
	pub fn read<I: num::Int>(&self, addr: usize) -> Result<I, MemoryError> {
	    let size = size_of::<I>();

	    // Load the number as a big-endian number (we can change it later)
	    let val = range(0, size)
	        .fold(Ok(0 as usize), |cur, idx| {
	            match cur {
	                Ok(x) => Ok((x << 8) | try!(self.memory.get_u8(addr + idx)) as usize),
	                x => x
	            }
	        });

	    // Convert to the expected value and return
	    // We can unwrap here because we only read `size` bytes above
	    match val {
	    	Ok(v) => Ok(to_endian(self.endianness, num::NumCast::from(v).unwrap())),
	    	Err(e) => Err(e)
	    }
	}

	/// Writes a (possibly multi-byte) integer (of type `I`) to the underlying memory.
	pub fn write<I: num::Int>(&mut self, addr: usize, val: I) -> Result<(), MemoryError> {
		// Convert to the specified endianness
		let out_val = to_endian(self.endianness, val);

	    let size = size_of::<I>();

	    // Write the number out as a big-endian number (the caller can ensure it is formatted as necessary for that)
	    let result = range(0, size)
	    	.fold(Ok(out_val), |cur, idx| {
	    		match cur {
	    			Ok(x) => {
	    				// Take the right-most byte
	    				let byte : u8 = num::NumCast::from(x & num::NumCast::from(0xFF).unwrap()).unwrap();

	    				// Calculate the address for it
	    				let byte_addr = (addr + (size - 1)) - idx;

	    				// Store the byte
	    				try!(self.memory.set_u8(byte_addr, byte));

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
}

fn to_endian<I: num::Int>(endianness: Endianness, val: I) -> I {
	match endianness {
		Endianness::BigEndian => val.to_be(),
		Endianness::LittleEndian => val.to_le()
	}
}

#[cfg(test)]
mod test {
	use mem::{Memory,FixedMemory,MemoryView};

	#[test]
	pub fn read_can_read_u8_value() {
		let mut mem = FixedMemory::with_size(10);
		mem.set_u8(1, 42);
		let view = MemoryView::big_endian(mem);
		assert_eq!(view.read::<u8>(1).unwrap(), 42u8);
	}

	#[test]
	pub fn read_can_read_u16_big_endian_value() {
		let mut mem = FixedMemory::with_size(10);
		mem.set_u8(1, 0xFF);
		mem.set_u8(2, 0xEE);
		let view = MemoryView::big_endian(mem);
		assert_eq!(view.read::<u16>(1).unwrap(), 0xFFEEu16);
	}
}