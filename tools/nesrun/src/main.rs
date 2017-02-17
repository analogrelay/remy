//! Tests the NES using the inst_test rom
extern crate remy;

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;

use std::{env,fs};

use remy::systems::nes;

pub fn main() {
    // Set up console logging
    let drain = slog_term::streamer().compact().build().fuse();
    let log = slog::Logger::root(drain, o!());

    let rom_path = match env::args().nth(1) {
        Some(r) => r,
        None => {
            println!("usage: nesrun [path to ROM file]");
            return;
        }
    };

    // Create a NES
    let mut nes = nes::Nes::new(Some(log.clone()));

    // Load the test rom
    let rom = nes::load_rom(&mut fs::File::open(rom_path).expect("failed to open ROM file")).expect("failed to load ROM");
    let cart = nes::Cartridge::load(rom, Some(log.clone())).expect("failed to load ROM into cartridge");

    // Load the cartridge into the nes
    nes.load(cart);

    loop {
        // Step one cycle forward
        nes.step().expect("error stepping NES");

        // Read the test status
        let status = nes.mem().get_u8(0x6000).expect("failed to read test status");
        info!(log,
            "cycle" => nes.cpu.clock.get(),
            "test_status" => status;
            "status: {}", status);
        println!("Press ENTER to step to the next instruction");
        let mut _str = String::new();
        ::std::io::stdin().read_line(&mut _str).unwrap();
    }
}

