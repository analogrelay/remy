//! Tests the MOS6502 CPU using the nestest rom
#![feature(convert)]

extern crate remy;

use std::{env,fs,io};
use std::io::BufRead;

use remy::systems::nes;
use remy::hw::mos6502;
use remy::mem::{self,Memory};
use remy::instr::Instruction;

#[test]
pub fn mos6502_can_run_nestest_rom() {
    // Locate the test rom
    let mut romfile = env::current_dir().unwrap();
    romfile.push("tests");
    romfile.push("files");

    let mut logfile = romfile.clone();
    logfile.push("nestest.log");
    romfile.push("nestest.nes");

    // Load the test rom
    let mut rom = nes::rom::read(&mut fs::File::open(romfile).unwrap()).unwrap();

    // 2KB internal ram mirrored through 0x1FFF
    let ram = Box::new(mem::Mirrored::new(mem::Fixed::new(0x0800), 0x2000));

    // Load the ROM into memory
    let prg_rom = Box::new(mem::read_only(mem::Mirrored::new(mem::Fixed::from_contents(rom.prg_banks.remove(0)), 0x8000)));

    // Create a black hole for APU/IO registers
    let apu_io = Box::new(mem::Fixed::from_contents(vec![0xFF; 0x20]));

    // Set up the virtual memory
    let mut memory = mem::Virtual::new();
    memory.attach(0x0000, ram).unwrap();
    memory.attach(0x4000, apu_io).unwrap();
    memory.attach(0x8000, prg_rom).unwrap();

    // If there is PRG RAM, set it up
    if rom.header.prg_ram_size.total > 0 {
        let prg_ram = Box::new(mem::Mirrored::new(mem::Fixed::new(rom.header.prg_ram_size.total as usize), 0x2000));
        memory.attach(0x6000, prg_ram).unwrap();
    }

    // Set up the CPU
    let mut cpu = mos6502::Mos6502::without_bcd();
    cpu.flags.replace(mos6502::Flags::new(0x24));
    cpu.pc.set(0xC000);

    // Load the test log
    let log = io::BufReader::new(fs::File::open(logfile).unwrap());

    for log_line in log.lines() {
        // Fetch next instruction
        let addr = cpu.pc.get();
        if addr == 0x0000 {
            return;
        }
        let instr: mos6502::Instruction = match cpu.pc.decode(&memory) {
            Ok(i) => i,
            Err(e) => panic!("Error decoding instruction at ${:04X}: {}", addr, e)
        };
        let end_addr = cpu.pc.get();

        // Load the actual value of the memory of the address
        let mut buf = [0u8; 3];
        let instr_size: usize = (end_addr - addr) as usize;
        if let Err(e) = memory.get(addr, &mut buf[0..instr_size]) {
            panic!("Error retrieving opcodes for instruction at ${:04X}: {}", addr, e)
        }

        // Capture register values for logging
        let a = cpu.registers.a;
        let x = cpu.registers.x;
        let y = cpu.registers.y;
        let p = cpu.flags.bits;
        let sp = cpu.registers.sp;

        // Format the instruction log entry
        let instr_str = match instr.get_log_string(&mut cpu, &memory) {
            Ok(s) => s,
            Err(e) => panic!("Error getting log string for ${:04X} {}: {}", addr, instr, e)
        };

        // Dispatch the instruction, but use a clone so we can still dump the instruction to the
        // log
        if let Err(e) = mos6502::dispatch(instr.clone(), &mut cpu, &mut memory) {
            panic!("Error at ${:04X} {}: {}", addr, instr, e)
        }

        // Generate the log line in the style of the nestest log
        // C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD CYC:  0 SL:241
        // (We're going to strip "SL" and "CYC" out of the input line)
        let actual_log = format!(
            "{:04X}  {:<9}{:<32} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            addr,
            buf.iter()
                .take(instr_size)
                .map(|&x| format!("{:02X} ", x))
                .fold(String::with_capacity(instr_size), |s,v| s + v.as_str()),
            instr_str,
            a,
            x,
            y,
            p,
            sp);

        // Compare to the next line in the
        let mut expected_log = log_line.unwrap();
        let end = expected_log.find("CYC:").unwrap() - 1;
        expected_log.truncate(end);
        if expected_log != actual_log {
            println!("Execution Error");
            println!("Expected: {}", expected_log);
            println!("  Actual: {}", actual_log);
            panic!("CPU execution did not match expected results");
        }
    }
}
