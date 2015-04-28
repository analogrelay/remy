//! Tests the MOS6502 CPU using the nestest rom

extern crate remy;

use std::env;
use std::fs::{self};

use remy::systems::nes;
use remy::cpus::mos6502;
use remy::mem::{self,Memory};

fn main() {
    // Locate the test rom
    let mut romfile = env::current_dir().unwrap();
    romfile.push("tests");
    romfile.push("files");
    romfile.push("nestest.nes");

    // Load the test rom
    let mut rom = nes::rom::read(&mut fs::File::open(romfile).unwrap()).unwrap();

    // 2KB internal ram mirrored through 0x1FFF
    let ram = Box::new(mem::Mirrored::new(mem::Fixed::new(0x0800), 0x2000));

    // Load the ROM into memory
    let prg_rom = Box::new(mem::read_only(mem::Mirrored::new(mem::Fixed::from_contents(rom.prg_banks.remove(0)), 0x8000)));
    println!("Attaching {} bytes of prg rom", prg_rom.len());

    // Set up the virtual memory
    let mut memory = mem::Virtual::new();
    memory.attach(0x0000, ram).unwrap();
    memory.attach(0x8000, prg_rom).unwrap();

    // If there is PRG RAM, set it up
    if rom.header.prg_ram_size.total > 0 {
        let prg_ram = Box::new(mem::Mirrored::new(mem::Fixed::new(rom.header.prg_ram_size.total as usize), 0x2000));
        memory.attach(0x6000, prg_ram).unwrap();
    }

    // Set up the CPU
    let mut cpu = mos6502::Mos6502::without_bcd(memory);
    cpu.flags.replace(mos6502::Flags::new(0x24));
    cpu.pc.set(0xC000);

    loop {
        // Run an instruction
        cpu.step().unwrap();

        // Log stuff
        println!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:  0 SL:  0",
            cpu.registers.a,
            cpu.registers.x,
            cpu.registers.y,
            cpu.flags.bits,
            cpu.registers.sp);
    }
}
