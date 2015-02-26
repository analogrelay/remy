use mem::Memory;
use cpu::mos6502::{ExecError,Operand,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M: Memory {
    let b = try!(op.get_u8(cpu));
    if b & 0x80 != 0 {
        cpu.flags.set(Flags::CARRY());
    }
    let r = (b << 1) & 0xFE;
    try!(op.set_u8(cpu, r));
    if r & 0x80 != 0 {
        cpu.flags.set(Flags::SIGN());
    }
    if r == 0 {
        cpu.flags.set(Flags::ZERO());
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpu::mos6502::instr::asl;
	use cpu::mos6502::{Mos6502,Operand,RegisterName,Flags};
    
    #[test]
    pub fn asl_shifts_value_left() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0x0F;
        asl::exec(&mut cpu, Operand::Register(RegisterName::A)).unwrap();
        assert_eq!(cpu.registers.a, 0x1E);
    }

    #[test]
    pub fn asl_sets_carry_if_bit_7_is_set_before_shifting() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0x81;
        asl::exec(&mut cpu, Operand::Register(RegisterName::A)).unwrap();
        assert_eq!(cpu.registers.a, 0x02);
        assert_eq!(cpu.flags, Flags::CARRY() | Flags::RESERVED());
    }

    #[test]
    pub fn asl_sets_sign_if_bit_7_is_set_after_shifting() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0x40;
        asl::exec(&mut cpu, Operand::Register(RegisterName::A)).unwrap();
        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED());
    }

    #[test]
    pub fn asl_sets_zero_if_value_is_zero_after_shifting() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0x00;
        asl::exec(&mut cpu, Operand::Register(RegisterName::A)).unwrap();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let cpu = Mos6502::new(vm);

        cpu
    }
}
