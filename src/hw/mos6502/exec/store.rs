use mem::{Memory,MemoryExt};
use hw::mos6502::{exec, cpu};
use hw::mos6502::{Mos6502,Operand};

pub fn exec<M>(cpu: &mut Mos6502, mem: &mut M, reg: cpu::RegisterName, op: Operand) -> exec::Result where M: Memory {
    let _x = cpu.clock.suspend();

    let val = reg.get(cpu);
    try!(op.set_u8(cpu, mem, val));

    Ok(())
}

pub fn ahx<M>(cpu: &mut Mos6502, mem: &mut M, op: Operand) -> exec::Result where M: Memory {
    let h = ((try!(op.get_addr(cpu, mem)) & 0xFF00) >> 8) as u8;
    let val = cpu.registers.a & cpu.registers.x & h;
    try!(op.set_u8(cpu, mem, val));
    Ok(())
}

pub fn sax<M>(cpu: &mut Mos6502, mem: &mut M, op: Operand) -> exec::Result where M: Memory {
    let val = cpu.registers.a & cpu.registers.x;
    try!(op.set_u8(cpu, mem, val));
    Ok(())
}

pub fn sh<M>(cpu: &mut Mos6502, mem: &mut M, reg: cpu::RegisterName, op: Operand) -> exec::Result where M: Memory {
    let h = ((try!(op.get_addr(cpu, mem)) & 0xFF00) >> 8) as u8;
    let val = reg.get(cpu) & h;
    try!(op.set_u8(cpu, mem, val));
    Ok(())
}

pub fn tas<M>(cpu: &mut Mos6502, mem: &mut M, op: Operand) -> exec::Result where M: Memory {
    let val = cpu.registers.a & cpu.registers.x;
    cpu.registers.sp = val;
    let h = ((try!(op.get_addr(cpu, mem)) & 0xFF00) >> 8) as u8;
    let mem_val = val & h;
    try!(op.set_u8(cpu, mem, mem_val));
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use mem::MemoryExt;
    use hw::mos6502::exec::store;
    use hw::mos6502::{cpu,Mos6502,Operand};

    #[test]
    pub fn store_sets_operand_to_register_value() {
        let mut mem = mem::Fixed::new(10);
        let mut cpu = Mos6502::new(); 

        cpu.registers.a = 42;
        store::exec(&mut cpu, &mut mem, cpu::RegisterName::A, Operand::Absolute(5)).unwrap();

        assert_eq!(Ok(42), mem.get_u8(5));
    }

    #[test]
    pub fn sh_sets_operand_to_register_value_and_high_byte_of_address() {
        let mem = mem::Fixed::new(10);
        let mut vm = mem::Virtual::new();
        vm.attach(0x3C00, Box::new(mem)).unwrap();

        let mut cpu = Mos6502::new();

        cpu.registers.x = 0xF0;
        store::sh(&mut cpu, &mut vm, cpu::RegisterName::X, Operand::Absolute(0x3C01)).unwrap();

        assert_eq!(Ok(0x30), vm.get_u8(0x3C01));
    }

    #[test]
    pub fn tas_does_its_crazy_business() {
        let mem = mem::Fixed::new(10);
        let mut vm = mem::Virtual::new();
        vm.attach(0x1C00, Box::new(mem)).unwrap();

        let mut cpu = Mos6502::new();

        cpu.registers.a = 0x3F;
        cpu.registers.x = 0xF0;
        store::tas(&mut cpu, &mut vm, Operand::Absolute(0x1C01)).unwrap();

        assert_eq!(0x30, cpu.registers.sp);
        assert_eq!(Ok(0x10), vm.get_u8(0x1C01));
    }

    #[test]
    pub fn ahx_sets_operand_to_a_and_x_and_high_byte_of_address() {
        let mem = mem::Fixed::new(10);
        let mut vm = mem::Virtual::new();
        vm.attach(0x3C00, Box::new(mem)).unwrap();

        let mut cpu = Mos6502::new();

        cpu.registers.a = 0x3F;
        cpu.registers.x = 0xF0;
        store::ahx(&mut cpu, &mut vm, Operand::Absolute(0x3C01)).unwrap();

        assert_eq!(Ok(0x30), vm.get_u8(0x3C01));
    }

    #[test]
    pub fn sax_sets_operand_to_a_and_x() {
        let mut mem = mem::Fixed::new(10);
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0x3F;
        cpu.registers.x = 0xF0;
        store::sax(&mut cpu, &mut mem, Operand::Absolute(5)).unwrap();

        assert_eq!(Ok(0x30), mem.get_u8(5));
    }
}
