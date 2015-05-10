//! Tests the MOS6502 CPU cycle counts

extern crate remy;
extern crate byteorder;

use remy::mem::{self,Memory,MemoryExt};
use remy::cpus::mos6502::{self,Instruction,Operand,RegisterName};

use byteorder::LittleEndian;

#[test]
pub fn adc() {
    let mut ctx = TestContext::new();
    ctx.test(Instruction::ADC(Operand::Immediate(0xA5)), 2);
    ctx.test(Instruction::ADC(Operand::Absolute(0x0010)), 3);
    ctx.test(Instruction::ADC(Operand::Indexed(0x0010, RegisterName::X)), 4);
    ctx.test(Instruction::ADC(Operand::Absolute(0x0110)), 4);
    ctx.test(Instruction::ADC(Operand::Indexed(0x01E0, RegisterName::X)), 4);
    ctx.test(Instruction::ADC(Operand::Indexed(0x01FF, RegisterName::X)), 5);
    ctx.test(Instruction::ADC(Operand::PreIndexedIndirect(0x0000)), 6);
    ctx.test(Instruction::ADC(Operand::PostIndexedIndirect(0x0000)), 5);
    ctx.test(Instruction::ADC(Operand::PostIndexedIndirect(0x0010)), 6);
}

struct TestContext<'a> {
    cpu: mos6502::Mos6502,
    mem: mem::Virtual<'a>,
    errors: Vec<String>
}

impl<'a> TestContext<'a> {
    pub fn test(&mut self, instr: Instruction, cycle_diff: u64) {
        let before = self.cpu.clock.get();
        if let Err(e) = mos6502::dispatch(instr.clone(), &mut self.cpu, &mut self.mem) {
            panic!("Error dispatching {}: {}", instr, e)
        }
        let actual_diff = self.cpu.clock.get() - before;
        if cycle_diff != actual_diff {
            self.errors.push(format!("{:?} cycles were {}; expected: {}", instr, actual_diff, cycle_diff));
        }
    }

    pub fn new() -> TestContext<'a> {
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
        cpu.registers.x = 10;
        cpu.registers.y = 10;

        memory.set_u16::<LittleEndian>(0x0000, 0x0100).unwrap();
        memory.set_u16::<LittleEndian>(0x0010, 0x01FF).unwrap();

        TestContext {
            cpu: cpu,
            mem: memory,
            errors: Vec::new()
        }
    }
}

impl<'a> Drop for TestContext<'a> {
    fn drop(&mut self) {
        if self.errors.len() > 0 {
            println!("");
            println!("Errors:");
            for err in self.errors.iter() {
                println!(" * {}", err);
            }
            panic!("Errors occurred");
        }
    }
}
