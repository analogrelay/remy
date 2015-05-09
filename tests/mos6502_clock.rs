/*
//! Tests the MOS6502 CPU cycle counts

extern crate remy;

use remy::systems::nes;
use remy::mem;
use remy::cpus::mos6502::{self,Mos6502,Instruction,Operand};

#[test]
pub fn adc() {
    let (cpu, mem) = init();
    test_instr(Instruction::ADC(Operand::Immediate(0x42)));
}

pub fn test_instr(instr: Instruction, cycle_diff: u64) {
    let before = cpu.clock.get();
    if let Err(e) = mos6502::dispatch(instr.clone(), &mut cpu, &mut memory) {
        panic!("Error dispatching {}: {}", instr, e)
    }
    assert_eq!(before + cycle_diff, cpu.clock.get());
}

pub fn init() -> (cpu, mem) {
    // 2KB internal ram mirrored through 0x1FFF
    let ram = Box::new(mem::Mirrored::new(mem::Fixed::new(0x0800), 0x2000));

    // Load the ROM into memory
    let prg_rom = Box::new(mem::read_only(mem::Mirrored::new(mem::Fixed::from_contents(vec![0x00]), 0x8000)));

    // Create a black hole for APU/IO registers
    let apu_io = Box::new(mem::Mirrored::new(mem::Fixed::from_contents(vec![0x00]), 0x20));

    // Set up the virtual memory
    let mut memory = mem::Virtual::new();
    memory.attach(0x0000, ram).unwrap();
    memory.attach(0x4000, apu_io).unwrap();
    memory.attach(0x8000, prg_rom).unwrap();

    // Set up the CPU
    let mut cpu = mos6502::Mos6502::without_bcd();
    cpu.flags.replace(mos6502::Flags::new(0x24));
    cpu.pc.set(0xC000);

    (cpu, memory)
}
*/
