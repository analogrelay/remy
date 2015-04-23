use mem::Memory;
use cpus::mos6502::exec;
use cpus::mos6502::{Flags,Mos6502};

pub fn from_interrupt<M>(cpu: &mut Mos6502<M>) -> Result<(), exec::Error> where M : Memory {
    let p = try!(cpu.pull());
    let l = try!(cpu.pull()) as u64;
    let h = try!(cpu.pull()) as u64;

    cpu.pc.set((h << 8) | l);
    cpu.flags.replace(Flags::new(p));
    Ok(())
}

pub fn from_sub<M>(cpu: &mut Mos6502<M>) -> Result<(), exec::Error> where M : Memory {
    let l = try!(cpu.pull()) as u64;
    let h = try!(cpu.pull()) as u64;

    cpu.pc.set(((h << 8) | l) + 1);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use cpus::mos6502::exec::ret;
    use cpus::mos6502::{cpu,Mos6502};

    #[test]
    pub fn rti_loads_flags_from_stack() {
        let mut cpu = init_cpu();

        // Set up for the return
        cpu.push(0xAB).unwrap(); // PC High
        cpu.push(0xCD).unwrap(); // PC Low
        cpu.push(0xEF).unwrap(); // Flags

        ret::from_interrupt(&mut cpu).unwrap();

        assert_eq!(cpu.flags.bits, 0xEF);
    }

    #[test]
    pub fn rti_loads_pc_from_stack() {
        let mut cpu = init_cpu();

        // Set up for the return
        cpu.push(0xAB).unwrap(); // PC High
        cpu.push(0xCD).unwrap(); // PC Low
        cpu.push(0xEF).unwrap(); // Flags

        ret::from_interrupt(&mut cpu).unwrap();

        assert_eq!(cpu.pc.get(), 0xABCD);
    }

    #[test]
    pub fn rts_loads_pc_from_stack_and_increments_it() {
        let mut cpu = init_cpu();

        // Set up for the return
        cpu.push(0xAB).unwrap(); // PC High
        cpu.push(0xCD).unwrap(); // PC Low

        ret::from_sub(&mut cpu).unwrap();

        assert_eq!(cpu.pc.get(), 0xABCE);
    }

    fn init_cpu() -> Mos6502<mem::Virtual<'static>> {
        let stack_memory = mem::Fixed::new(32);
        let mut vm = mem::Virtual::new();

        vm.attach(cpu::STACK_START, Box::new(stack_memory)).unwrap();

        let mut cpu = Mos6502::new(vm);

        cpu.registers.sp = 16;
        cpu
    }
}
