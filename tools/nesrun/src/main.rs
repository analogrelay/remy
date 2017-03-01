//! Tests the NES using the inst_test rom
extern crate remy;

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;

use std::{env,fs};

use remy::systems::nes;

fn read_test_status(nes: &nes::Nes) -> String {
    let mut s = String::new();
    let mut addr = 0x6004;
    loop {
        let x = nes.mem().get_u8(addr).expect("failed to read test status");
        if x == 0 {
            break;
        }
        let c = ::std::char::from_u32(x as u32).expect("invalid character in test status");
        s.push(c);
        addr += 1;
    }
    s
}

pub fn main() {
    // Set up console logging
    let drain = slog_term::streamer().build().fuse();
    let log = slog::Logger::root(slog::level_filter(slog::Level::Debug, drain), o!(
        "location" => move |info: &slog::Record| format!("{}:{}", info.module(), info.line())
    ));

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

    // Reset the system
    nes.reset().expect("error resetting NES");

    let mut status = 0;
    let mut started = false;
    loop {
        // Step one cycle forward
        nes.step().expect("error stepping NES");

        // Read the test status
        let new_status = nes.mem().get_u8(0x6000).expect("failed to read test status");

        if new_status != status {
            match (started, new_status) {
                (false, 0x80) => {
                    started = true;
                    info!(log, "test started");
                }
                (true, 0x00) => {
                    info!(log, "test complete");
                    break;
                },
                _ => {}
            }
            status = new_status;
        }
    }

    let result = read_test_status(&nes);

    println!("Result:{}", result);
}

