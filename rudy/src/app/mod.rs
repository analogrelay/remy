use sdl2::{video,pixels,render};
use sdl2::rect::Rect;

use remy::systems::nes;
use remy::hw::rp2C02;

pub struct App<'a> {
    nes: nes::Nes, // NES system
    renderer: render::Renderer<'a>,
    texture: render::Texture,
    size: (i32, i32),
    screen: [u8; rp2C02::ppu::BYTES_PER_SCREEN]
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
            (nes::SCREEN_WIDTH as i32, nes::SCREEN_HEIGHT as i32)).unwrap();

        App {
            nes: nes,
            renderer: renderer,
            texture: texture,
            size: size,
            screen: [0; rp2C02::ppu::BYTES_PER_SCREEN]
        }
    }

    pub fn update(&mut self) {
    }

    pub fn render(&mut self) {
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
