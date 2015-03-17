use mem::Memory;
use cpus::mos6502::{ExecError,Mos6502,RegisterName,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, reg: RegisterName, op: Operand) -> Result<(), ExecError> where M: Memory {
    let val = reg.get(cpu);
    try!(op.set_u8(cpu, val));
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::Memory;
    use cpus::mos6502::instr::store;
    use cpus::mos6502::{Mos6502,RegisterName,Operand};

    #[test]
    pub fn store_sets_operand_to_register_value() {
        let mut cpu = Mos6502::with_fixed_memory(10); 

        cpu.registers.a = 42;
        store::exec(&mut cpu, RegisterName::A, Operand::Absolute(5)).unwrap();

        assert_eq!(Ok(42), cpu.mem.get_u8(5));
    }
}
