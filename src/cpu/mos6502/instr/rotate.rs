use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Operand,Flags};

pub fn left<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M : Memory {
    exec(cpu, op, true)
}

pub fn right<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M : Memory {
    exec(cpu, op, false)
}

fn exec<M>(cpu: &mut Mos6502<M>, op: Operand, left: bool) -> Result<(), ExecError> where M : Memory {
    let n = try!(op.get_u8(cpu));    

    // Grab the bit that's about to fall off
    let t = if left { n & 0x80 } else { n & 0x01 };

    // Shift the current value, then add the carry bit in to the other end
    let carry_byte =
        if left && cpu.flags.carry() {
            0x01
        } else if cpu.flags.carry() {
            0x80
        } else {
            0x00
        };
    let b = (if left { n << 1 } else { n >> 1 }) | carry_byte;
    try!(op.set_u8(cpu, b));

    // Set the flags
    cpu.flags.set_sign_and_zero(b);
    cpu.flags.set_if(Flags::CARRY(), t != 0);

    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
    use cpu::mos6502::instr::rotate;
    use cpu::mos6502::{Mos6502,Operand,Flags,RegisterName};

    #[test]
    pub fn rotate_can_rotate_left() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);

        cpu.flags.set(Flags::CARRY());
        cpu.registers.a = 0b01101100;

        rotate::left(&mut cpu, Operand::Register(RegisterName::A)).unwrap();

        assert_eq!(cpu.registers.a, 0b11011001);
        assert!(!cpu.flags.intersects(Flags::CARRY()));
    }
}
