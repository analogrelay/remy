use mem::Memory;
use cpus::mos6502::exec;
use cpus::mos6502::{Mos6502,Operand,Flags};

pub fn left<M>(cpu: &mut Mos6502, mem: &mut M, op: Operand) -> Result<(), exec::Error> where M : Memory {
    exec(cpu, op, true)
}

pub fn right<M>(cpu: &mut Mos6502, mem: &mut M, op: Operand) -> Result<(), exec::Error> where M : Memory {
    exec(cpu, op, false)
}

fn exec<M>(cpu: &mut Mos6502, mem: &mut M, op: Operand, left: bool) -> Result<(), exec::Error> where M : Memory {
    let n = try!(op.get_u8(cpu, mem));

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
    try!(op.set_u8(cpu, mem, b));

    // Set the flags
    cpu.flags.set_sign_and_zero(b);
    cpu.flags.set_if(Flags::CARRY(), t != 0);

    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use cpus::mos6502::exec::rotate;
    use cpus::mos6502::{Mos6502,Operand,Flags};

    #[test]
    pub fn rotate_can_rotate_left() {
        let mut cpu = Mos6502::new();

        cpu.flags.set(Flags::CARRY());
        cpu.registers.a = 0b01101100;

        rotate::left(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();

        assert_eq!(cpu.registers.a, 0b11011001);
        assert!(!cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn rotate_left_puts_leftmost_bit_in_carry_flag() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0b10000000;
        rotate::left(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert!(cpu.flags.intersects(Flags::CARRY()));
        cpu.registers.a = 0b01111111;
        rotate::left(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert!(!cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn rotate_left_sets_sign_flag_if_value_now_negative() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0b01111111;
        rotate::left(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn rotate_left_sets_zero_flag_if_value_now_zero() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0b10000000;
        rotate::left(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    pub fn rotate_can_rotate_right() {
        let mut cpu = Mos6502::new();

        cpu.flags.set(Flags::CARRY());
        cpu.registers.a = 0b01101100;

        rotate::right(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();

        assert_eq!(cpu.registers.a, 0b10110110);
        assert!(!cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn rotate_right_puts_rightmost_bit_in_carry_flag() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0b00000001;
        rotate::right(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert!(cpu.flags.intersects(Flags::CARRY()));
        cpu.registers.a = 0b11111110;
        rotate::right(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert!(!cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn rotate_right_sets_sign_flag_if_value_now_negative() {
        let mut cpu = Mos6502::new();

        cpu.flags.set(Flags::CARRY());
        cpu.registers.a = 0b00000000;
        rotate::right(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn rotate_right_sets_zero_flag_if_value_now_zero() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0b00000001;
        rotate::right(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }
}
