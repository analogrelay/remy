use slog;
use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{Flags,Mos6502};

pub fn from_interrupt<M>(cpu: &mut Mos6502, mem: &M, log: &slog::Logger) -> Result<(), exec::Error> where M : Memory {
    let p = try_log!(cpu.pull(mem), log);
    let flags = Flags::new(p);
    trace!(log, "cpu" => cpu,
        "from" => cpu.registers.sp,
        "flags" => flags;
        "pulled flags from stack");

    let l = try_log!(cpu.pull(mem), log) as u64;
    let h = try_log!(cpu.pull(mem), log) as u64;
    let pc = (h << 8) | l;
    trace!(log, "cpu" => cpu,
        "from" => cpu.registers.sp - 1,
        "pc" => pc;
        "pulled new PC from stack");

    cpu.pc.set(pc);
    cpu.flags.replace(flags);
    trace!(log, "cpu" => cpu; "updated flags and PC");
    Ok(())
}

pub fn from_sub<M>(cpu: &mut Mos6502, mem: &M, log: &slog::Logger) -> Result<(), exec::Error> where M : Memory {
    let l = try_log!(cpu.pull(mem), log) as u64;
    let h = try_log!(cpu.pull(mem), log) as u64;
    let pc = ((h << 8) | l) + 1;
    trace!(log, "cpu" => cpu,
        "from" => cpu.registers.sp - 1,
        "pc" => pc;
        "pulled new PC from stack");

    cpu.pc.set(pc);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use hw::mos6502::exec::ret;
    use hw::mos6502::{Mos6502,STACK_START};

    #[test]
    pub fn rti_loads_flags_from_stack() {
        let (mut cpu, mut mem) = init_cpu();

        // Set up for the return
        cpu.push(&mut mem, 0xAB).unwrap(); // PC High
        cpu.push(&mut mem, 0xCD).unwrap(); // PC Low
        cpu.push(&mut mem, 0xEF).unwrap(); // Flags

        ret::from_interrupt(&mut cpu, &mem).unwrap();

        assert_eq!(cpu.flags.bits, 0xEF);
    }

    #[test]
    pub fn rti_loads_pc_from_stack() {
        let (mut cpu, mut mem) = init_cpu();

        // Set up for the return
        cpu.push(&mut mem, 0xAB).unwrap(); // PC High
        cpu.push(&mut mem, 0xCD).unwrap(); // PC Low
        cpu.push(&mut mem, 0xEF).unwrap(); // Flags

        ret::from_interrupt(&mut cpu, &mem).unwrap();

        assert_eq!(cpu.pc.get(), 0xABCD);
    }

    #[test]
    pub fn rts_loads_pc_from_stack_and_increments_it() {
        let (mut cpu, mut mem) = init_cpu();

        // Set up for the return
        cpu.push(&mut mem, 0xAB).unwrap(); // PC High
        cpu.push(&mut mem, 0xCD).unwrap(); // PC Low

        ret::from_sub(&mut cpu, &mem).unwrap();

        assert_eq!(cpu.pc.get(), 0xABCE);
    }

    fn init_cpu() -> (Mos6502,mem::Virtual<'static>) {
        let stack_memory = mem::Fixed::new(32);
        let mut vm = mem::Virtual::new();

        vm.attach(STACK_START, Box::new(stack_memory)).unwrap();

        let mut cpu = Mos6502::new();

        cpu.registers.sp = 16;
        (cpu,vm)
    }
}
