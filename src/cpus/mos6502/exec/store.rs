use mem::Memory;
use cpus::mos6502::{exec, cpu};
use cpus::mos6502::{Mos6502,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, reg: cpu::RegisterName, op: Operand) -> exec::Result where M: Memory {
    let val = reg.get(cpu);
    try!(op.set_u8(cpu, val));
    Ok(())
}

pub fn ahx<M>(cpu: &mut Mos6502<M>, op: Operand) -> exec::Result where M: Memory {
    let h = ((try!(op.get_addr(cpu)) & 0xFF00) >> 8) as u8;
    let val = cpu.registers.a & cpu.registers.x & h;
    try!(op.set_u8(cpu, val));
    Ok(())
}

pub fn sax<M>(cpu: &mut Mos6502<M>, op: Operand) -> exec::Result where M: Memory {
    let val = cpu.registers.a & cpu.registers.x;
    try!(op.set_u8(cpu, val));
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use mem::Memory;
    use cpus::mos6502::exec::store;
    use cpus::mos6502::{cpu,Mos6502,Operand};

    #[test]
    pub fn store_sets_operand_to_register_value() {
        let mut cpu = Mos6502::with_fixed_memory(10); 

        cpu.registers.a = 42;
        store::exec(&mut cpu, cpu::RegisterName::A, Operand::Absolute(5)).unwrap();

        assert_eq!(Ok(42), cpu.mem.get_u8(5));
    }

    #[test]
    pub fn ahx_sets_operand_to_a_and_x_and_high_byte_of_address() {
        let mem = mem::FixedMemory::new(10);
        let mut vm = mem::VirtualMemory::new();
        vm.attach(0x3C00, Box::new(mem)).unwrap();

        let mut cpu = Mos6502::new(vm);

        cpu.registers.a = 0x3F;
        cpu.registers.x = 0xF0;
        store::ahx(&mut cpu, Operand::Absolute(0x3C01)).unwrap();

        assert_eq!(Ok(0x30), cpu.mem.get_u8(0x3C01));
    }

    #[test]
    pub fn sax_sets_operand_to_a_and_x() {
        let mut cpu = Mos6502::with_fixed_memory(10);

        cpu.registers.a = 0x3F;
        cpu.registers.x = 0xF0;
        store::sax(&mut cpu, Operand::Absolute(5)).unwrap();

        assert_eq!(Ok(0x30), cpu.mem.get_u8(5));
    }
}
