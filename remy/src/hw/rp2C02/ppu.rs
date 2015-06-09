use clock;
use mem::{self,Memory};

use hw::rp2C02::{self,Result,registers};

pub const NAMETABLE_SIZE: usize = 0x0400;
pub const NAMETABLE_BASE: usize = 0x2000;
pub const NAMETABLE_END: usize = 0x2C00;

pub const BACKDROP_COLOR_ADDR: u64 = 0x3F00;
pub const BG_PALETTE_BASE: u64 = 0x3F01;

pub const PIXELS_PER_SCANLINE: u16 = 256;
pub const PIXELS_PER_TILE: u16 = 8;
pub const PIXELS_PER_SCREEN: u16 = PIXELS_PER_SCANLINE * SCANLINES_PER_FRAME;
pub const BYTES_PER_PIXEL: u16 = 3;
pub const BYTES_PER_SCREEN: usize = BYTES_PER_PIXEL as usize * PIXELS_PER_SCREEN as usize;
pub const TILES_PER_SCANLINE: u16 = PIXELS_PER_SCANLINE / PIXELS_PER_TILE;
pub const SCANLINES_PER_FRAME: u16 = 240;
pub const CYCLES_PER_SCANLINE: u64 = 114;
pub const VBLANK_SCANLINE: u16 = 241;
pub const END_SCANLINE: u16 = 261;

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

pub struct Rp2C02 {
    clock: clock::Clock,
    registers: registers::Registers,
    current_scanline: u16
}

impl Rp2C02 {
    pub fn new() -> Rp2C02 {
        Rp2C02 {
            clock: clock::Clock::new(),
            registers: registers::Registers::new(),
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
    pub fn step(&mut self, target_cycle: u64, mem: &mut mem::Memory, screen: &mut rp2C02::ScreenBuffer) -> Result<()> {
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

                let start = (PIXELS_PER_SCANLINE * self.current_scanline) as usize;
                let end = start + PIXELS_PER_SCANLINE as usize;
                assert!(start < end && start < (BYTES_PER_SCREEN as usize) && end < (BYTES_PER_SCREEN as usize));

                try!(self.render_scanline(mem, screen));
            } else if self.current_scanline == VBLANK_SCANLINE {
                panic!("vblank starting");
            } else if self.current_scanline == END_SCANLINE {
                panic!("frame completed");
            }

            // Advance to the next scanline
            self.current_scanline += 1;

            self.clock.tick(CYCLES_PER_SCANLINE);
        }
        Ok(())
    }

    //fn get_pixel(&self, index: usize) -> Pixel {
        //assert!(index * 3 + 2 < PALETTE.len());
        //Pixel {
            //blue: PALETTE[index * 3],
            //green: PALETTE[index * 3 + 1],
            //red: PALETTE[index * 3 + 2],
        //}
    //}

    //fn get_pattern_value(&self, mem: &mem::Memory, pattern_table: u16, tile_index: u8, tile_x: u16, tile_y: u16) -> mem::Result<u8> {
        //let tile_base = (pattern_table + ((tile_index as u16) << 4) as u16 + tile_y) as u64;
        //let lo_plane = try!(mem.get_u8(tile_base));
        //let hi_plane = try!(mem.get_u8(tile_base + 0x08));
        //let lo_bit = (lo_plane >> (7 - tile_x)) & 0x01;
        //let hi_bit = (hi_plane >> (7 - tile_x)) & 0x01;
        //Ok((hi_bit << 1) | lo_bit)
    //}

    //fn get_background(&mut self, mem: &mut mem::Memory, x: u16, y: u16) -> Result<Option<Pixel>> {
        //// Determine active nametable
        //let nametable = self.registers.ppuctrl().nametable_base();
        //assert!(nametable == 0x2000 || nametable == 0x2400 || nametable == 0x2800 || nametable == 0x2C00);

        //// Calculate nametable cell and tile offset
        //let (col, tile_x) = (x / 8, x % 8);
        //let (row, tile_y) = (y / 8, y % 8);

        //// Load tile from nametable
        //let tile_index = try!(mem.get_u8((nametable + (row * TILES_PER_SCANLINE) + col) as u64));

        //// Load palette index from the pattern table
        //let pattern_table = self.registers.ppuctrl().bg_pattern_table();
        //let pattern_color = try!(self.get_pattern_value(mem, pattern_table, tile_index, tile_x, tile_y));
        //if pattern_color == 0 {
            //// Pattern has no value here, background is transparent.
            //return Ok(None);
        //}

        //// Load attribute table
        //let attribute_group = row / 4 * 8 + col / 4;
        //let attribute = try!(mem.get_u8((nametable + 0x3C0 + attribute_group) as u64));
        //let palette_index = match (col % 4 < 2, row % 4 < 2) {
            //(true, true) => attribute & 0x0003,
            //(false, true) => (attribute >> 2) & 0x0003,
            //(true, false) => (attribute >> 4) & 0x0003,
            //(false, false) => (attribute >> 6) & 0x0003
        //};

        //// Load the colour out of the palette
        //let color_offset = (palette_index << 2) | pattern_color;
        //let color = try!(mem.get_u8(BG_PALETTE_BASE + color_offset as u64));
        //Ok(Some(self.get_pixel(color as usize)))
    //}

    fn render_scanline(&mut self, _mem: &mut mem::Memory, screen: &mut rp2C02::ScreenBuffer) -> Result<()> {
        // Pick a random color and render it
        let pixel = rp2C02::Pixel {
            red: if self.current_scanline % 3 == 0 { 255 } else { 0 },
            green: if self.current_scanline % 3 == 1 { 255 } else { 0 },
            blue: if self.current_scanline % 3 == 2 { 255 } else { 0 }
        };
        for x in 0 .. PIXELS_PER_SCANLINE {
            screen.put_pixel(x as usize, self.current_scanline as usize, pixel);
        }

        //let backdrop_index = try!(mem.get_u8(BACKDROP_COLOR_ADDR));
        //let backdrop = self.get_pixel(backdrop_index as usize);

        //for x in 0 .. PIXELS_PER_SCANLINE {
            //let background = if self.registers.ppumask().background() {
                //let scanline = self.current_scanline;
                //try!(self.get_background(mem, x, scanline))
            //} else {
                //None
            //};

            //// TODO: Sprites

            //// Determine the visible pixel
            //let pixel = match background {
                //Some(ref pix) => pix,
                //None => &backdrop
            //};

            //// Put it to the screen
            //let base = (x * 3) as usize;
            //screen[base] = pixel.blue;
            //screen[base + 1] = pixel.green;
            //screen[base + 2] = pixel.red;
        //}

        Ok(())
    }
}
