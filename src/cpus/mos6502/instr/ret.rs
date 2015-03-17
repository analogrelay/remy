use mem::Memory;
use cpus::mos6502::{ExecError,Flags,Mos6502};

pub fn from_interrupt<M>(cpu: &mut Mos6502<M>) -> Result<(), ExecError> where M : Memory {
    let p = try!(cpu.pull());
    let l = try!(cpu.pull()) as usize;
    let h = try!(cpu.pull()) as usize;

    cpu.pc.set((h << 8) | l);
    cpu.flags.replace(Flags::new(p));
    Ok(())
}

pub fn from_sub<M>(cpu: &mut Mos6502<M>) -> Result<(), ExecError> where M : Memory {
    let l = try!(cpu.pull()) as usize;
    let h = try!(cpu.pull()) as usize;
    
    cpu.pc.set(((h << 8) | l) + 1);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::{FixedMemory,VirtualMemory};
	use cpus::mos6502::instr::ret;
	use cpus::mos6502::{Mos6502,STACK_START};

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

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let stack_memory = FixedMemory::new(32);
        let mut vm = VirtualMemory::new();

        vm.attach(STACK_START, Box::new(stack_memory)).unwrap();

        let mut cpu = Mos6502::new(vm);

        cpu.registers.sp = 16;
        cpu
    }
}
