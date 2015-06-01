use std::convert;

use clock;
use mem::{self,Memory};

pub const NAMETABLE_SIZE: usize = 0x0400;
pub const NAMETABLE_BASE: usize = 0x2000;
pub const NAMETABLE_END: usize = 0x2C00;

pub const BACKDROP_COLOR_ADDR: usize = 0x3F00;
pub const BG_PALETTE_BASE: usize = 0x3F01;

pub const PIXELS_PER_SCANLINE: usize = 256;
pub const PIXELS_PER_TILE: usize = 8;
pub const PIXELS_PER_SCREEN: usize = PIXELS_PER_SCANLINE * SCANLINES_PER_FRAME;
pub const BYTES_PER_PIXEL: usize = 3;
pub const BYTES_PER_SCREEN: usize = BYTES_PER_PIXEL * PIXELS_PER_SCREEN;
pub const TILES_PER_SCANLINE: usize = PIXELS_PER_SCANLINE / PIXELS_PER_TILE;
pub const SCANLINES_PER_FRAME: usize = 240;
pub const CYCLES_PER_SCANLINE: usize = 114;
pub const VBLANK_SCANLINE: usize = 241;
pub const END_SCANLINE: usize = 261;

pub type ScreenBuffer = [u8; BYTES_PER_SCREEN];

// Unapologetically yanked from https://github.com/pcwalton/sprocketnes/blob/master/ppu.rs
const PALETTE: [u8; 192] = [
    124,124,124,    0,0,252,        0,0,188,        68,40,188,
    148,0,132,      168,0,32,       168,16,0,       136,20,0,
    80,48,0,        0,120,0,        0,104,0,        0,88,0,
    0,64,88,        0,0,0,          0,0,0,          0,0,0,
    188,188,188,    0,120,248,      0,88,248,       104,68,252,
    216,0,204,      228,0,88,       248,56,0,       228,92,16,
    172,124,0,      0,184,0,        0,168,0,        0,168,68,
    0,136,136,      0,0,0,          0,0,0,          0,0,0,
    248,248,248,    60,188,252,     104,136,252,    152,120,248,
    248,120,248,    248,88,152,     248,120,88,     252,160,68,
    248,184,0,      184,248,24,     88,216,84,      88,248,152,
    0,232,216,      120,120,120,    0,0,0,          0,0,0,
    252,252,252,    164,228,252,    184,184,248,    216,184,248,
    248,184,248,    248,164,192,    240,208,176,    252,224,168,
    248,216,120,    216,248,120,    184,248,184,    184,248,216,
    0,252,252,      248,216,248,    0,0,0,          0,0,0
];

pub type Result<T> = ::std::result::Result<T, Error>;

pub enum Error {
    MemoryAccessError(mem::Error)
}

impl convert::From<mem::Error> for Error {
    fn from(err: mem::Error) -> Error {
        Error::MemoryAccessError(err)
    }
}

pub struct Rp2C02 {
    clock: clock::Clock,
    registers: Registers,
    current_scanline: usize
}

pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8
}

pub struct Registers {
    ppuctrl: PpuCtrl,
    ppumask: PpuMask,
    ppustatus: PpuStatus,
    ppuscroll: PpuScroll
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            ppuctrl: PpuCtrl::new(),
            ppumask: PpuMask::new(),
            ppustatus: PpuStatus::new(),
            ppuscroll: PpuScroll::new()
        }
    }
}

pub enum NextScrollField { X, Y }

pub struct PpuScroll {
    x: u8,
    y: u8,
    next: NextScrollField
}

impl PpuScroll {
    pub fn new() {
        PpuScroll {
            x: 0,
            y: 0,
            next: NextScrollField::X
        }
    }
}

pub enum VramDirection {
    GoingAcross,
    GoingDown
}

pub struct PpuCtrl {
    nametable_base: u16,
    vram_direction: VramDirection,
    sprite_pattern_table: u8,
    bg_pattern_table: u8,
    large_sprites: bool,
    secondary: bool,
    generate_nmi: bool
}

impl PpuCtrl {
    pub fn new() -> PpuCtrl {
        PpuCtrl {
            nametable: 0,
            vram_direction: VramDirection::GoingAcross,
            sprite_pattern_table: 0,
            bg_pattern_table: 0,
            large_sprites: false,
            secondary: false,
            generate_nmi: false
        }
    }
}

pub struct PpuMask {
    greyscale: bool,
    leftmost_background: bool,
    leftmost_sprites: bool,
    background: bool,
    sprites: bool,
    emphasize_red: bool,
    emphasize_green: bool,
    emphasize_blue: bool
}

impl PpuMask {
    pub fn new() -> PpuMask {
        PpuMask {
            greyscale: false,
            leftmost_background: false,
            leftmost_sprites: false,
            background: false,
            sprites: false,
            emphasize_red: false,
            emphasize_green: false,
            emphasize_blue: false
        }
    }
}

pub struct PpuStatus {
    sprite_overflow: bool,
    sprite_0_hit: bool,
    vertical_blank: bool
}

impl PpuStatus {
    pub fn new() -> PpuStatus {
        PpuStatus {
            sprite_overflow: false,
            sprite_0_hit: false,
            vertical_blank: false
        }
    }
}

impl Rp2C02 {
    pub fn new() -> Rp2C02 {
        Rp2C02 {
            clock: clock::Clock::new(),
            registers: Registers::new(),
            current_scanline: 0
        }
    }

    /// Emulates the execution of PPU cycles until `target_cycle` is reached
    ///
    /// The PPU emulation only runs entire scanlines at once. So, this method
    /// only executes a scanline if the current clock, plus the number of cycles
    /// required to render a scanline (`CYCLES_PER_SCANLINE`) is less than
    /// the desired `target_cycle`. If it is, a single scan line is rendered,
    /// the PPU clock is updated by `CYCLES_PER_SCANLINE` and the process is
    /// repeated. If it is not, this method returns to allow the CPU to continue
    /// processing.
    pub fn step(&mut self, target_cycle: usize, mem: &mut mem::Memory, screen: &mut ScreenBuffer) -> Result<()> {
        loop {
            // Check when the next scan line is
            let next_scan_line = self.clock.get() + CYCLES_PER_SCANLINE;
            if next_scan_line > target_cycle {
                // Next scan line is beyond our target, we've done enough.
                break;
            }

            if self.current_scanline < SCANLINES_PER_FRAME {
                // Render the scanline
                debug!("rendering scanline {}", self.current_scanline);

                let start = PIXELS_PER_SCANLINE * self.current_scanline;
                let end = start + PIXELS_PER_SCANLINE;
                assert!(start < end && start < BYTES_PER_SCREEN && end < BYTES_PER_SCREEN);

                let scanline_screen = screen[start .. end];

                try!(self.render_scanline(mem, scanline_screen));
            } else if self.current_scanline == VBLANK_SCANLINE {
                debug!("vblank starting");
            } else if self.current_scanline == END_SCANLINE {
                debug!("frame completed");
            }

            // Advance to the next scanline
            self.current_scanline += 1;

            self.clock.tick(CYCLES_PER_SCANLINE);
        }
    }

    fn get_pixel(&self, index: usize) -> Pixel {
        assert!(index * 3 + 2 < PALETTE.len());
        Pixel {
            blue: PALETTE[index * 3],
            green: PALETTE[index * 3 + 1],
            red: PALETTE[index * 3 + 2],
        }
    }

    fn get_background(&mut self, mem: &mut mem::Memory, x: usize, y: usize) -> Option<Pixel> {
        // Determine active nametable
        let nametable = self.registers.ppuctrl.nametable;
        assert!(nametable == 0x2000 || nametable == 0x2400 || nametable == 0x2800 || nametable == 0x2C00);

        // Calculate nametable cell and tile offset
        let (col, tile_x) = (x / 8, x % 8);
        let (row, tile_y) = (y / 8, y % 8);

        // Load tile from nametable
        let tile_index = try!(mem.get_u8(nametable + (row * TILES_PER_SCANLINE) + col));

        // Load palette index from the pattern table
        let pattern_table = self.registers.ppuctrl.bg_pattern_table;
        let pattern_color = self.get_pattern_value(pattern_table, tile_index, tile_x, tile_y);
        if pattern_color == 0 {
            // Pattern has no value here, background is transparent.
            return None;
        }

        // Load attribute table
        let attribute_group = row / 4 * 8 + col / 4;
        let attribute = try!(mem.get_u8(nametable + 0x3C0 + attribute_group));
        let palette_index = match (col % 4 < 2, row % 4 < 2) {
            (true, true) => attribute & 0x0003,
            (false, true) => (attribute >> 2) & 0x0003,
            (true, false) => (attribute >> 4) & 0x0003,
            (false, false) => (attribute >> 6) & 0x0003
        };

        // Load the colour out of the palette
        let color_offset = (palette_index << 2) | pattern_color;
        let color = try!(mem.get_u8(BG_PALETTE_BASE + color_offset));
        Some(self.get_color(color))
    }

    fn render_scanline(&mut self, mem: &mut mem::Memory, screen: &mut [u8]) -> Result<()> {
        let mut cursor = 0;

        let backdrop_index = try!(mem.get_u8(BACKDROP_COLOR_ADDR));
        let backdrop = self.get_pixel(backdrop_index);

        for x in 0 .. PIXELS_PER_SCANLINE {
            let background = if self.ppumask.background {
                self.get_background(mem, x, self.current_scanline)
            } else {
                None
            };

            // TODO: Sprites

            // Determine the visible pixel
            let pixel = match background {
                Some(pix) => pix,
                None => backdrop
            };

            // Put it to the screen
            screen[x * 3] = pixel.blue;
            screen[x * 3 + 1] = pixel.green;
            screen[x * 3 + 2] = pixel.red;
        }
    }
}
