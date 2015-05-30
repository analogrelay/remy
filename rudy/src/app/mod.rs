use remy::systems::nes;
use opengl_graphics;
use piston::event;

pub struct App {
    gl: opengl_graphics::GlGraphics, // OpenGL drawing backend.
    backbufer: opengl_graphics::Texture, // The back buffer to draw to
    nes: nes::Nes, // NES system
}

impl App {
    pub fn new(gl: GlGraphics, nes: nes::Nes) -> App {
        App {
            gl: gl,
            nes: nes
        }
    }

    pub fn render(&mut self, args: &event::RenderArgs) {
    }

    pub fn update(&mut self, args: &event::UpdateArgs) {
        // Tick the system
    }
}

