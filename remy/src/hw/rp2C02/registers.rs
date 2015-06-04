#[allow(dead_code)]
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

    pub fn ppuctrl(&self) -> &PpuCtrl {
        &self.ppuctrl
    }

    pub fn ppumask(&self) -> &PpuMask {
        &self.ppumask
    }

    pub fn ppustatus(&self) -> &PpuStatus {
        &self.ppustatus
    }

    pub fn ppuscroll(&self) -> &PpuScroll {
        &self.ppuscroll
    }
}

pub enum NextScrollField { X, Y }

#[allow(dead_code)]
pub struct PpuScroll {
    x: u8,
    y: u8,
    next: NextScrollField
}

impl PpuScroll {
    pub fn new() -> PpuScroll {
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

#[allow(dead_code)]
pub struct PpuCtrl {
    nametable_base: u16,
    vram_direction: VramDirection,
    sprite_pattern_table: u16,
    bg_pattern_table: u16,
    large_sprites: bool,
    secondary: bool,
    generate_nmi: bool
}

impl PpuCtrl {
    pub fn new() -> PpuCtrl {
        PpuCtrl {
            nametable_base: 0,
            vram_direction: VramDirection::GoingAcross,
            sprite_pattern_table: 0,
            bg_pattern_table: 0,
            large_sprites: false,
            secondary: false,
            generate_nmi: false
        }
    }

    pub fn nametable_base(&self) -> u16 {
        self.nametable_base
    }

    pub fn bg_pattern_table(&self) -> u16 {
        self.bg_pattern_table
    }
}

#[allow(dead_code)]
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

    pub fn background(&self) -> bool {
        self.background
    }
}

#[allow(dead_code)]
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

