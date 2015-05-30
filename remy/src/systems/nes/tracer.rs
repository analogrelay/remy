use hw::mos6502;

/// Implemented by objects that wish to be notified of events that occur during NES execution.
pub trait Tracer {
    fn decoded(&mut self, instr: mos6502::Instruction, addr: u64, bytes: [u8]
}
