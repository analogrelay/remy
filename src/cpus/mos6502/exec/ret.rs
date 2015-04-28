use mem::Memory;
use cpus::mos6502::exec;
use cpus::mos6502::{Flags,Mos6502};

pub fn from_interrupt<M>(cpu: &mut Mos6502, mem: &M) -> Result<(), exec::Error> where M : Memory {
    let p = try!(cpu.pull(mem));
    let l = try!(cpu.pull(mem)) as u64;
    let h = try!(cpu.pull(mem)) as u64;

    cpu.pc.set((h << 8) | l);
    cpu.flags.replace(Flags::new(p));
    Ok(())
}

pub fn from_sub<M>(cpu: &mut Mos6502, mem: &M) -> Result<(), exec::Error> where M : Memory {
    let l = try!(cpu.pull(mem)) as u64;
    let h = try!(cpu.pull(mem)) as u64;

    cpu.pc.set(((h << 8) | l) + 1);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use cpus::mos6502::exec::ret;
    use cpus::mos6502::{cpu,Mos6502,STACK_START};

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
