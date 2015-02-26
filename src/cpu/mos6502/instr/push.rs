use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,RegisterName};

pub fn exec<M>(cpu: &mut Mos6502<M>, r: RegisterName) -> Result<(), ExecError> where M : Memory {
    let val = r.get(cpu);
    try!(cpu.push(val));
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::{FixedMemory,VirtualMemory};
	use cpu::mos6502::instr::push;
	use cpu::mos6502::{Mos6502,RegisterName,STACK_START};

    #[test]
    pub fn push_puts_register_value_on_top_of_stack() {
        let mut cpu = init_cpu();
        cpu.registers.a = 42;
        push::exec(&mut cpu, RegisterName::A).unwrap();
        assert_eq!(Ok(42), cpu.pop());
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
