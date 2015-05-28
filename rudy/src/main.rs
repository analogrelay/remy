#[macro_use]
extern crate log;

extern crate sdl2;
extern crate remy;
extern crate env_logger;

use std::{env,fs,path};
use remy::systems::nes;

fn main() {
    // Initialize logging
    env_logger::init().unwrap();

    // Load the rom
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rudy <rom file name>");
        return;
    }

    let file = path::Path::new(&args[1]);
    println!("Loading {:?}...", file);

    // Load the ROM
    let rom = nes::load_rom(&mut fs::File::open(file).unwrap()).unwrap();

    let screen = gfx::BufferScreen::new(256, 240);

    // Create a NES
    let nes = nes::Nes::new(rom, screen);
}
