use slog;
use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{Mos6502,Operand};

pub fn exec<M>(cpu: &mut Mos6502, mem: &M, op: Operand, log: &slog::Logger) -> Result<(), exec::Error> where M : Memory {
    let m = try_log!(op.get_u8(cpu, mem), log);
    let v = cpu.registers.a | m;
    trace!(log, "cpu" => cpu,
        "a" => cpu.registers.a,
        "m" => m,
        "r" => v,
        "op" => op;
        "evaluated a | m = r");

    cpu.flags.set_sign_and_zero(v);
    cpu.registers.a = v;
    trace!(log, "cpu" => cpu; "stored result in A");

    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use mem::Memory;
    use hw::mos6502::exec::ora;
    use hw::mos6502::{Mos6502,Flags,Operand};

    #[test]
    fn ora_sets_sign_bit_if_result_is_negative() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b00001111;
        ora::exec(&mut cpu, &mut mem, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn ora_sets_zero_bit_if_result_is_zero() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 0b00000000).unwrap();
        cpu.registers.a = 0b00000000;
        ora::exec(&mut cpu, &mut mem, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn ora_sets_a_to_result_of_or() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b00001111;
        ora::exec(&mut cpu, &mut mem, Operand::Absolute(0)).unwrap();
        assert_eq!(0b11111111, cpu.registers.a);
    }

    fn init_cpu() -> (Mos6502,mem::Virtual<'static>) {
        let base_memory = mem::Fixed::new(10);
        let mut vm = mem::Virtual::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new();

        (cpu,vm)
    }
}
