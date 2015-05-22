/*
/// Represents a single Pattern Table, which is a collection of `Tile` objects
pub struct PatternTable {
    data: Vec<Tile>
}

impl PatternTable {
}
*/

/// Represents a single Tile in the Pattern Table
pub struct Tile {
    data: [u8; 16]
}

impl Tile {
    pub fn new() -> Tile {
        Tile { data: [0; 16] }
    }

    pub fn get_raw(&self) -> [u8; 16] {
        self.data
    }

    pub fn set_raw(&mut self, buf: [u8; 16]) {
        self.data = buf;
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        // Read the two bit planes and return the value
        // http://wiki.nesdev.com/w/index.php/PPU_pattern_tables
        if x > 8 {
            panic!("{} is out of range for 'x' coordinate! Tiles are 8x8 pixels in size.", x);
        }
        if y > 8 {
            panic!("{} is out of range for 'y' coordinate! Tiles are 8x8 pixels in size.", y);
        }

        // Get the two bit planes
        let low_plane = self.data[y as usize];
        let hi_plane = self.data[(y + 8) as usize];

        // Twiddle some bits
        let low = (low_plane >> (7 - x)) & 0x01;
        let hi = (hi_plane >> (7 - x)) & 0x01;
        low | (hi << 1)
    }

    pub fn set(&mut self, x: u8, y: u8, val: u8) {
        if x > 8 {
            panic!("{} is out of range for 'x' coordinate! Tiles are 8x8 pixels in size.", x);
        }
        if y > 8 {
            panic!("{} is out of range for 'y' coordinate! Tiles are 8x8 pixels in size.", y);
        }
        if val > 3 {
            panic!("{} is out of range for pixel value! Tile pixel data must be a value between 0 and 3 (inclusive).", val);
        }

        // Extract the new values
        let low = val & 0x01;
        let hi = (val >> 1) & 0x01;

        // Get the current plane values
        let low_plane = self.data[y as usize];
        let hi_plane = self.data[(y + 8) as usize];

        // Twiddle some bits (could totally improve this, I'm just bad at bit twiddling :))
        self.data[y as usize] = if low == 0 {
            low_plane & !(1 << (7 - x))
        } else {
            low_plane | (1 << (7 - x))
        };

        self.data[(y + 8) as usize] = if hi == 0 {
            hi_plane & !(1 << (7 - x))
        } else {
            hi_plane | (1 << (7 - x))
        };
    }
}

#[cfg(test)]
mod tests {
    mod tile {
        use hw::rp2C02::pattern_table::Tile;

        const TEST_TILE_DATA: [u8; 16] = [0x41, 0xC2, 0x44, 0x48, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x16, 0x21, 0x42, 0x87];

        #[test]
        pub fn set_writes_correct_data_to_tile_buffer() {
            // Uses the test pattern from http://wiki.nesdev.com/w/index.php/PPU_pattern_tables
            //      01234567
            //      --------
            //  0 | .1.....3
            //  1 | 11....3.
            //  2 | .1...3..
            //  3 | .1..3...
            //  4 | ...3.22.
            //  5 | ..3....2
            //  6 | .3....2.
            //  7 | 3....222
            let mut tile = Tile::new();

            tile.set(1, 0, 1);
            tile.set(7, 0, 3);

            tile.set(0, 1, 1);
            tile.set(1, 1, 1);
            tile.set(6, 1, 3);

            tile.set(1, 2, 1);
            tile.set(5, 2, 3);

            tile.set(1, 3, 1);
            tile.set(4, 3, 3);

            tile.set(3, 4, 3);
            tile.set(5, 4, 2);
            tile.set(6, 4, 2);

            tile.set(2, 5, 3);
            tile.set(7, 5, 2);

            tile.set(1, 6, 3);
            tile.set(6, 6, 2);

            tile.set(0, 7, 3);
            tile.set(5, 7, 2);
            tile.set(6, 7, 2);
            tile.set(7, 7, 2);

            // Read the raw data
            let raw = tile.get_raw();

            // Check against the expected value
            assert_eq!(TEST_TILE_DATA, raw);
        }

        #[test]
        pub fn get_reads_correct_data_from_tile_buffer() {
            let mut tile = Tile::new();

            // Uses the test pattern from http://wiki.nesdev.com/w/index.php/PPU_pattern_tables
            //      01234567
            //      --------
            //  0 | .1.....3
            //  1 | 11....3.
            //  2 | .1...3..
            //  3 | .1..3...
            //  4 | ...3.22.
            //  5 | ..3....2
            //  6 | .3....2.
            //  7 | 3....222

            // Write the raw data
            tile.set_raw(TEST_TILE_DATA);

            // Test the pixels
            assert_eq!(1, tile.get(1, 0));
            assert_eq!(3, tile.get(7, 0));

            assert_eq!(1, tile.get(0, 1));
            assert_eq!(1, tile.get(1, 1));
            assert_eq!(3, tile.get(6, 1));

            assert_eq!(1, tile.get(1, 2));
            assert_eq!(3, tile.get(5, 2));

            assert_eq!(1, tile.get(1, 3));
            assert_eq!(3, tile.get(4, 3));

            assert_eq!(3, tile.get(3, 4));
            assert_eq!(2, tile.get(5, 4));
            assert_eq!(2, tile.get(6, 4));

            assert_eq!(3, tile.get(2, 5));
            assert_eq!(2, tile.get(7, 5));

            assert_eq!(3, tile.get(1, 6));
            assert_eq!(2, tile.get(6, 6));

            assert_eq!(3, tile.get(0, 7));
            assert_eq!(2, tile.get(5, 7));
            assert_eq!(2, tile.get(6, 7));
            assert_eq!(2, tile.get(7, 7));
        }
    }
}
