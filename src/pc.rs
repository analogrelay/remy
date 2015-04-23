use instr;
use mem;

/// Represents a program counter value
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct ProgramCounter {
    pc: u64
}

impl ProgramCounter {
    /// Allocates a new program counter value (initialized to 0)
    pub fn new() -> ProgramCounter {
        ProgramCounter { pc: 0 }
    }

    /// Retrieves the current value of the program counter
    pub fn get(&self) -> u64 {
        self.pc
    }

    /// Sets the program counter to the provided value
    ///
    /// # Arguments
    ///
    /// * `val` - The value to set the program counter to
    pub fn set(&mut self, val: u64) {
        self.pc = val;
    }

    /// Advances or retreats the program counter by the provided (signed) amount
    ///
    /// # Argument
    ///
    /// * `amount` - The amount to advance (or retreat) the program counter by
    pub fn advance(&mut self, amount: i64) {
        self.pc = if amount < 0 {
            self.pc.wrapping_sub(amount.abs() as u64)
        } else {
            self.pc.wrapping_add(amount.abs() as u64)
        }
    }

    /// Decodes an instruction from the provided memory and updates the program counter as
    /// necessary
    pub fn decode<M, I>(&mut self, mem: M) -> Result<I, I::DecodeError> where I: instr::Instruction, M: mem::Memory {
        // Construct the reader
        let mut r = mem::cursor(mem, self.pc);

        // Decode the instruction
        let inst : Result<I, I::DecodeError> = instr::Instruction::decode(&mut r);

        // Adjust the PC
        self.pc = r.position();

        // Return the value
        inst
    }
}

#[cfg(test)]
mod test {
    use mem;
    use cpus::mos6502;
    use pc::ProgramCounter;

    #[test]
    pub fn advance_by_positive_value_increases_pc() {
        let mut pc = ProgramCounter::new();
        pc.advance(42);
        assert_eq!(pc.get(), 42);
    }

    #[test]
    pub fn advance_by_negative_value_decreases_pc() {
        let mut pc = ProgramCounter::new();
        pc.advance(42);
        pc.advance(-24);
        assert_eq!(pc.get(), 18);
    }

    #[test]
    pub fn decode_returns_decoded_instruction_and_advances_pc_on_successful_decode() {
        let mut pc = ProgramCounter::new();
        let mem = mem::Fixed::from_contents(vec![0x00, 0x00, 0x0C, 0xCD, 0xAB, 0x00, 0x00]);
        pc.advance(2);

        let inst = pc.decode(mem).unwrap();

        assert_eq!(mos6502::Instruction::IGN(mos6502::Operand::Absolute(0xABCD)), inst);
        assert_eq!(pc.get(), 5);
    }

    #[test]
    pub fn decode_returns_error_and_advances_pc_on_failed_decode() {
        let mut pc = ProgramCounter::new();
        let mem = mem::Fixed::from_contents(vec![0x00, 0x00, 0x0C]);
        pc.advance(2);

        let inst: mos6502::instr::decoder::Result<mos6502::Instruction> = pc.decode(mem);

        assert!(inst.is_err());
        assert_eq!(pc.get(), 3);
    }
}
