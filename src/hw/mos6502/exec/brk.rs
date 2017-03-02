use slog;
use byteorder::LittleEndian;

use mem::{Memory,MemoryExt};
use hw::mos6502::exec;
use hw::mos6502::{Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502, mem: &mut M, log: &slog::Logger) -> Result<(), exec::Error> where M: Memory {
    cpu.pc.advance(1);
    let pc = cpu.pc.get();
    try_log!(cpu.push(mem, ((pc & 0xFF00) >> 8) as u8), log);
    try_log!(cpu.push(mem, (pc & 0x00FF) as u8), log);
    trace!(log, "cpu" => cpu, "next_pc" => pc; "pushed next PC value on stack");

    let new_flags = cpu.flags | Flags::BREAK();
    try_log!(cpu.push(mem, new_flags.bits), log);
    trace!(log, "cpu" => cpu, "pushed_flags" => new_flags; "pushed flags on stack");

    trace!(log, "cpu" => cpu; "jumping to $FFFE");
    cpu.pc.set(try_log!(mem.get_u16::<LittleEndian>(0xFFFE), log) as u64);
    Ok(())
}

#[cfg(test)]
mod test {
    use slog;
    use byteorder::LittleEndian;

    use mem::{self,Memory,MemoryExt};
    use hw::mos6502::exec::brk;
    use hw::mos6502::{Mos6502,Flags};
    use hw::mos6502::STACK_START;

    #[test]
    pub fn brk_increments_and_pushes_pc_on_to_stack() {
        let (mut cpu, mut mem) = init_cpu();
        brk::exec(&mut cpu, &mut mem, &slog::Logger::root(slog::Discard, o!())).unwrap();

        assert_eq!(Ok(0xAB), mem.get_u8(STACK_START + 16));
        assert_eq!(Ok(0xCE), mem.get_u8(STACK_START + 15));
    }

    #[test]
    pub fn brk_sets_break_flag_and_pushes_flags_on_to_stack() {
        let (mut cpu, mut mem) = init_cpu();
        let flags = Flags::SIGN() | Flags::OVERFLOW() | Flags::RESERVED();
        cpu.flags.set(flags);
        brk::exec(&mut cpu, &mut mem, &slog::Logger::root(slog::Discard, o!())).unwrap();

        assert_eq!(Ok((flags | Flags::BREAK()).bits), mem.get_u8(STACK_START + 14));
    }

    #[test]
    pub fn brk_does_not_set_break_flag_on_current_flags() {
        let (mut cpu, mut mem) = init_cpu();
        let flags = Flags::SIGN() | Flags::OVERFLOW() | Flags::RESERVED();
        cpu.flags.set(flags);
        brk::exec(&mut cpu, &mut mem, &slog::Logger::root(slog::Discard, o!())).unwrap();

        assert_eq!(flags, cpu.flags);
    }

    #[test]
    pub fn brk_sets_pc_to_address_at_vector() {
        let (mut cpu, mut mem) = init_cpu();
        brk::exec(&mut cpu, &mut mem, &slog::Logger::root(slog::Discard, o!())).unwrap();

        assert_eq!(0xBEEF, cpu.pc.get());
    }

    fn init_cpu() -> (Mos6502, mem::Virtual<'static>) {
        let base_memory = mem::Fixed::new(32);
        let stack_memory = mem::Fixed::new(32);
        let vector_memory = mem::Fixed::new(6);
        let mut vm = mem::Virtual::new();

        vm.attach(0, Box::new(base_memory)).unwrap();
        vm.attach(STACK_START, Box::new(stack_memory)).unwrap();
        vm.attach(0xFFFA, Box::new(vector_memory)).unwrap();

        let mut cpu = Mos6502::new();

        cpu.registers.a = 42;
        cpu.registers.sp = 16;
        cpu.pc.set(0xABCD);
        vm.set_u16::<LittleEndian>(0xFFFE, 0xBEEF).unwrap();

        (cpu, vm)
    }
}
