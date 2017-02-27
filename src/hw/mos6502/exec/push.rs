use slog;
use mem::Memory;
use hw::mos6502::{exec, cpu};
use hw::mos6502::Mos6502;

pub fn exec<M>(cpu: &mut Mos6502, mem: &mut M, r: cpu::RegisterName, log: &slog::Logger) -> Result<(), exec::Error> where M : Memory {
    let val = if r == cpu::RegisterName::P {
        // http://visual6502.org/wiki/index.php?title=6502_BRK_and_B_bit
        // Set B bit on the value before pushing it
        trace!(log, cpu_state!(cpu), "setting BREAK on flags before pushing");
        (cpu::Flags::BREAK() | cpu::Flags::new(r.get(cpu))).bits
    } else {
        r.get(cpu)
    };
    let dest = cpu.registers.sp;
    try!(cpu.push(mem, val));
    trace!(log, cpu_state!(cpu),
        "from" => dest,
        "value" => val,
        "register" => r;
        "pushed to ${:04X}", dest);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use hw::mos6502::exec::push;
    use hw::mos6502::{cpu,Mos6502,STACK_START};

    #[test]
    pub fn push_puts_register_value_on_top_of_stack() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.registers.a = 42;
        push::exec(&mut cpu, &mut mem, cpu::RegisterName::A).unwrap();
        assert_eq!(Ok(42), cpu.pull(&mem));
    }

    #[test]
    pub fn push_sets_brk_flag_when_pushing_flags() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.flags.replace(cpu::Flags::SIGN() | cpu::Flags::ZERO());
        push::exec(&mut cpu, &mut mem, cpu::RegisterName::P).unwrap();
        assert_eq!(Ok(0b10110010), cpu.pull(&mem));
    }

    fn init_cpu() -> (Mos6502,mem::Virtual<'static>) {
        let stack_memory = mem::Fixed::new(32);
        let mut vm = mem::Virtual::new();

        vm.attach(STACK_START, Box::new(stack_memory)).unwrap();

        let mut cpu = Mos6502::new();

        cpu.registers.sp = 16;
        (cpu, vm)
    }
}
