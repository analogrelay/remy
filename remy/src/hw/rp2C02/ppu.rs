use clock;

pub struct Ppu {
    clock: clock::Clock
}

impl Ppu {
    /// Emulates the execution of PPU cycles until `target_cycle` is reached
    ///
    /// The PPU emulation only runs entire scanlines at once. So, the target_cycle
    /// is divided by `CYCLES_PER_SCANLINE` to determine how many scanlines to render,
    /// then the clock is updated to `target_cycle` to prepare for the next invocation.
    pub fn run(&mut self, target_cycle: usize) {
        loop {
            // Check when the next scan line is
            let next_scan_line = self.clock.get() + CYCLES_PER_SCANLINE;
            if next_scan_line > target_cycle {
                // Next scan line is beyond our target, we're done
                break;
            }

            // Run the scanline and advance the clock
            self.run_scanline();

            self.clock.tick(CYCLES_PER_SCANLINE);
        }
    }

    fn run_scanline(&mut self) {
    }
}
