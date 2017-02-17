//! Tests the NES using the inst_test rom
extern crate remy;

use std::{env,fs};

use remy::systems::nes;

#[test]
pub fn mos6502_can_run_01_basics_rom() {
    run_test("01-basics.nes");
}

fn run_test(rom_name: &str) {
    // Locate the test rom
    let mut romfile = env::current_dir().unwrap();
    romfile.push("tests");
    romfile.push("roms");
    romfile.push("inst_test");
    romfile.push("rom_singles");
    romfile.push(rom_name);

    // Create a NES
    let mut nes = nes::Nes::new();

    // Load the test rom
    let rom = nes::load_rom(&mut fs::File::open(romfile).expect("failed to open ROM file")).expect("failed to load ROM");
    let cart = nes::Cartridge::load(rom).expect("failed to load ROM into cartridge");

    // Load the cartridge into the nes
    nes.load(cart);

    loop {
        // Step one cycle forward
        nes.step().expect("error stepping NES");

        // Read the test status
        let status = nes.mem().get_u8(0x6000).expect("failed to read test status");
        println!("[after cycle {}] current test status: 0x{:X}", nes.cpu.clock.get(), status);
    }
}
