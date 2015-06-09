extern crate sdl2;
extern crate remy;
extern crate env_logger;

#[macro_use]
extern crate log;

use remy::systems::nes;

use std::{fs,io,env,path};
use std::io::Write;

use sdl2::event;

mod app;

fn main() {
    // Initialize Logging
    env_logger::init().unwrap();

    // Read the ROM
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: rudy <path to rom>").unwrap();
        return;
    }

    let rompath = path::Path::new(&args[1]);
    let rom = {
        let mut romfile = fs::File::open(rompath)
            .ok()
            .expect("failed to open ROM file");
        nes::load_rom(&mut romfile).ok().expect("error parsing ROM file")
    };
    let mut nes = nes::Nes::new();
    nes.load(rom).ok().expect("failed to load ROM into emulator");

    start_gfx(nes);
}

fn start_gfx(nes: nes::Nes) {
    let mut sdl = sdl2::init().everything().unwrap();
    let window = sdl.window("Rudy NES Emulator", 1024, 960).build().unwrap();
    let mut event_pump = sdl.event_pump();

    // Create the app
    let mut app = app::App::new(nes, window, (1024, 960));

    // Pump events
    loop {
        for event in event_pump.poll_iter() {
            if let event::Event::Quit { .. } = event {
                return
            }
        }

        app.update();
        app.render();
    }
}
