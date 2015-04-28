use cpus::mos6502::{exec,cpu};
use cpus::mos6502::Mos6502;

pub fn exec(cpu: &mut Mos6502, src: cpu::RegisterName, dst: cpu::RegisterName) -> Result<(), exec::Error> {
    let val = src.get(cpu);
    dst.set(cpu, val);
    Ok(())
}

#[cfg(test)]
mod test {
    use cpus::mos6502::exec::transfer;
    use cpus::mos6502::{cpu,Mos6502};

    #[test]
    pub fn transfer_sets_destination_register_to_source_register_value() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 42;
        transfer::exec(&mut cpu, cpu::RegisterName::A, cpu::RegisterName::X).unwrap();

        assert_eq!(42, cpu.registers.x);
    }
}
