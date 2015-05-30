extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate remy;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use remy::systems::nes;

use std::{fs,io,env,path};
use std::io::Write;

mod app;

fn main() {
    // Read the ROM
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: rudy <path to rom>");
        return;
    }

    let rompath = path::Path::new(&args[1]);
    let rom = {
        let mut romfile = fs::File::open(rompath)
            .ok()
            .expect("failed to open ROM file");
        nes::load_rom(&mut romfile).ok().expect("error parsing ROM file")
    };
    let cart = nes::Cartridge::load(rom).ok().expect("unsupported ROM");
    let mut nes = nes::Nes::new();
    nes.load(cart);

    start_gfx(nes);
}

fn start_gfx(nes: nes::Nes) {
    use piston::event::{RenderEvent,UpdateEvent,Events};

    let opengl = OpenGL::_3_2;

    // Create an Glutin window.
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Rudy NES Emulator",
            (512, 480)
        )
        .exit_on_esc(true)
    );

    // Create a new app
    let mut app = app::App::new(
        GlGraphics::new(opengl),
        nes);

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
