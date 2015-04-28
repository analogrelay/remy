use mem::Memory;
use cpus::mos6502::{exec, cpu};
use cpus::mos6502::Mos6502;

pub fn exec<M>(cpu: &mut Mos6502, mem: &mut M, r: cpu::RegisterName) -> Result<(), exec::Error> where M : Memory {
    let val = r.get(cpu);
    try!(cpu.push(mem, val));
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use cpus::mos6502::exec::push;
    use cpus::mos6502::{cpu,Mos6502,STACK_START};

    #[test]
    pub fn push_puts_register_value_on_top_of_stack() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.registers.a = 42;
        push::exec(&mut cpu, &mut mem, cpu::RegisterName::A).unwrap();
        assert_eq!(Ok(42), cpu.pull(&mem));
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
