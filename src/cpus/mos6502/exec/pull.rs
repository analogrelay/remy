use mem::Memory;
use cpus::mos6502::{exec,cpu};
use cpus::mos6502::Mos6502;

pub fn exec<M>(cpu: &mut Mos6502, mem: &M, r: cpu::RegisterName) -> Result<(), exec::Error> where M : Memory {
    let val = try!(cpu.pull(mem));
    cpu.flags.set_sign_and_zero(val);
    r.set(cpu, val);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use cpus::mos6502::exec::pull;
    use cpus::mos6502::{cpu,Mos6502,Flags,STACK_START};

    #[test]
    pub fn pull_puts_register_value_on_top_of_stack() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.push(&mut mem, 42).unwrap();
        pull::exec(&mut cpu, &mem, cpu::RegisterName::A).unwrap();
        assert_eq!(42, cpu.registers.a);
    }

    #[test]
    pub fn pull_clears_sign_flag_if_incoming_value_non_negative() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.flags.set(Flags::SIGN());
        cpu.push(&mut mem, 42).unwrap();
        pull::exec(&mut cpu, &mem, cpu::RegisterName::A).unwrap();
        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn pull_sets_sign_flag_if_incoming_value_negative() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.push(&mut mem, 0xFF).unwrap();
        pull::exec(&mut cpu, &mem, cpu::RegisterName::A).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn pull_clears_zero_flag_if_incoming_value_non_zero() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.flags.set(Flags::ZERO());
        cpu.push(&mut mem, 42).unwrap();
        pull::exec(&mut cpu, &mem, cpu::RegisterName::A).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    pub fn pull_sets_zero_flag_if_incoming_value_zero() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.push(&mut mem, 0).unwrap();
        pull::exec(&mut cpu, &mem, cpu::RegisterName::A).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
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
