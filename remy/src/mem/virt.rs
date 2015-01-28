use mem;

struct Segment<'a> {
    base : usize,
    memory : Box<mem::Memory+'a>
}

impl<'a> Segment<'a> {
    fn new(base: usize, memory: Box<mem::Memory+'a>) -> Segment<'a> {
        Segment {
            base: base,
            memory: memory
        }
    }
}

pub struct VirtualMemory<'a> {
    segments : Vec<Segment<'a>>
}

impl<'a> VirtualMemory<'a> {
    /// Constructs a new Virtual Memory with no member segments
    pub fn new() -> VirtualMemory<'a> {
        VirtualMemory {
            segments: Vec::new()
        }
    }

    /// Attaches another memory as a segment of this virtual memory with the specified base.
    ///
    /// # Arguments
    /// * `base` - The address to use as the base for the specified memory
    /// * `mem` - The memory to attach.
    pub fn attach(&mut self, base: usize, mem: Box<mem::Memory+'a>) {
        self.segments.push(Segment::new(base, mem));
    }
}

#[cfg(test)]
mod test {
    use mem::{FixedMemory,VirtualMemory};

    #[test]
    pub fn attach_adds_new_segment_with_specified_base() {
        let mut vm = VirtualMemory::new();
        let mem = Box::new(FixedMemory::with_size(10));
        assert_eq!(vm.segments.len(), 0);
        vm.attach(0x1400, mem);
        assert_eq!(vm.segments.len(), 1);
        assert_eq!(vm.segments[0].base, 0x1400);
    }
}