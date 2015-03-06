use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,RegisterName};

pub fn exec<M>(cpu: &mut Mos6502<M>, r: RegisterName) -> Result<(), ExecError> where M : Memory {
    let val = try!(cpu.pull());
    cpu.flags.set_sign_and_zero(val);
    r.set(cpu, val);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::{FixedMemory,VirtualMemory};
	use cpu::mos6502::instr::pull;
	use cpu::mos6502::{Mos6502,RegisterName,Flags,STACK_START};

    #[test]
    pub fn pull_puts_register_value_on_top_of_stack() {
        let mut cpu = init_cpu();
        cpu.push(42).unwrap();
        pull::exec(&mut cpu, RegisterName::A).unwrap();
        assert_eq!(42, cpu.registers.a);
    }

    #[test]
    pub fn pull_clears_sign_flag_if_incoming_value_non_negative() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::SIGN());
        cpu.push(42).unwrap();
        pull::exec(&mut cpu, RegisterName::A).unwrap();
        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn pull_sets_sign_flag_if_incoming_value_negative() {
        let mut cpu = init_cpu();
        cpu.push(0xFF).unwrap();
        pull::exec(&mut cpu, RegisterName::A).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn pull_clears_zero_flag_if_incoming_value_non_zero() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::ZERO());
        cpu.push(42).unwrap();
        pull::exec(&mut cpu, RegisterName::A).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    pub fn pull_sets_zero_flag_if_incoming_value_zero() {
        let mut cpu = init_cpu();
        cpu.push(0).unwrap();
        pull::exec(&mut cpu, RegisterName::A).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let stack_memory = FixedMemory::new(32);
        let mut vm = VirtualMemory::new();

        vm.attach(STACK_START, Box::new(stack_memory)).unwrap();

        let mut cpu = Mos6502::new(vm);

        cpu.registers.sp = 16;
        cpu
    }
}
