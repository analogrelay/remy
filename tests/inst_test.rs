//! Tests the NES using the inst_test rom
extern crate remy;

use remy::slog;
use std::{env,fs};

use remy::systems::nes;

#[test] pub fn mos6502_can_run_01_basics_rom() { run_test("01-basics.nes"); }
#[test] pub fn mos6502_can_run_02_implied_rom() { run_test("02-implied.nes"); }
#[test] pub fn mos6502_can_run_03_immediate_rom() { run_test("03-immediate.nes"); }
#[test] pub fn mos6502_can_run_04_zero_page_rom() { run_test("04-zero_page.nes"); }
#[test] pub fn mos6502_can_run_05_zp_xy_rom() { run_test("05-zp_xy.nes"); }
#[test] pub fn mos6502_can_run_06_absolute_rom() { run_test("06-absolute.nes"); }
#[test] pub fn mos6502_can_run_07_abs_xy_rom() { run_test("07-abs_xy.nes"); }
#[test] pub fn mos6502_can_run_08_ind_x_rom() { run_test("08-ind_x.nes"); }
#[test] pub fn mos6502_can_run_09_ind_y_rom() { run_test("09-ind_y.nes"); }
#[test] pub fn mos6502_can_run_10_branches_rom() { run_test("10-branches.nes"); }
#[test] pub fn mos6502_can_run_11_stack_rom() { run_test("11-stack.nes"); }
#[test] pub fn mos6502_can_run_12_jmp_jsr_rom() { run_test("12-jmp_jsr.nes"); }
#[test] pub fn mos6502_can_run_13_rts_rom() { run_test("13-rts.nes"); }
#[test] pub fn mos6502_can_run_14_rti_rom() { run_test("14-rti.nes"); }
#[test] pub fn mos6502_can_run_15_brk_rom() { run_test("15-brk.nes"); }
#[test] pub fn mos6502_can_run_16_special_rom() { run_test("16-special.nes"); }

fn read_test_status(nes: &nes::Nes) -> String {
    let mut s = String::new();
    let mut addr = 0x6004;
    loop {
        let x = nes.mem().get_u8(addr).expect("failed to read test status");
        if x == 0 {
            break;
        }
        let c = ::std::char::from_u32(x as u32).expect("invalid character in test status");
        s.push(c);
        addr += 1;
    }
    s
}

fn run_test(rom_name: &str) {
    // Locate the test rom
    let mut romfile = env::current_dir().unwrap();
    romfile.push("tests");
    romfile.push("roms");
    romfile.push("inst_test");
    romfile.push("rom_singles");
    romfile.push(rom_name);

    // Create a NES
    let mut nes = nes::Nes::new(None);

    // Load the test rom
    let rom = nes::load_rom(&mut fs::File::open(romfile).expect("failed to open ROM file")).expect("failed to load ROM");
    let cart = nes::Cartridge::load(rom, None).expect("failed to load ROM into cartridge");

    // Load the cartridge into the nes
    nes.load(cart);

    // Reset the system
    nes.reset();

    let mut status = 0;
    let mut started = false;
    let mut final_status = 0;
    loop {
        // Step one cycle forward
        nes.step().expect("error stepping NES");

        // Read the test status
        let new_status = nes.mem().get_u8(0x6000).expect("failed to read test status");

        if new_status != status {
            match (started, new_status) {
                (false, 0x80) => {
                    started = true;
                }
                (true, 0x81) => {
                    nes.reset();
                },
                (true, x) => {
                    final_status = x;
                    break;
                }
                _ => {}
            }
            status = new_status;
        }
    }

    let result = read_test_status(&nes);

    if(final_status != 0x00) {
        panic!("test failed! code: ${:02X}, message: {}", final_status, result);
    }
}
