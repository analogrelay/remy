use sdl2::{video,pixels,render};
use sdl2::rect::Rect;

use remy::systems::nes;
use remy::hw::rp2C02;

pub struct App<'a> {
    nes: nes::Nes, // NES system
    renderer: render::Renderer<'a>,
    texture: render::Texture,
    size: (i32, i32)
}

impl<'a> App<'a> {
    pub fn new(nes: nes::Nes, window: video::Window, size: (i32, i32)) -> App<'a> {
        // Set up drawing surfaces
        let renderer = window.renderer()
            .accelerated()
            .build()
            .unwrap();
        let texture = renderer.create_texture(
            pixels::PixelFormatEnum::BGR24,
            render::TextureAccess::Streaming,
            (nes::PIXELS_PER_SCANLINE as i32, nes::SCANLINES_PER_FRAME as i32)).unwrap();

        App {
            nes: nes,
            renderer: renderer,
            texture: texture,
            size: size
        }
    }

    pub fn update(&mut self) {
        trace!("stepping NES system");

        let tex = &mut self.texture;
        let nes = &mut self.nes;
        tex.with_lock(None, |buf, pitch| {
            let mut screen = rp2C02::ScreenBuffer::new(buf, pitch);
            nes.step(&mut screen);
        }).unwrap();
    }

    pub fn render(&mut self) {
        trace!("rendering NES display");
        let tex = &self.texture;

        let (w, h) = self.size;
        let mut drawer = self.renderer.drawer();
        drawer.clear();
        drawer.copy(
            tex,
            None,
            Some(Rect::new(0, 0, w, h)));
        drawer.present();
    }
}
