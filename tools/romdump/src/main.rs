extern crate remy;

use remy::systems::nes;

use std::{env, fs};

fn main() {
    let rom_path = match env::args().nth(1) {
        Some(r) => r,
        None => {
            println!("usage: romdump [path to ROM file]");
            return;
        }
    };

    println!("Loading ROM: {}", rom_path);

    let rom = nes::load_rom(&mut fs::File::open(rom_path).expect("failed to open ROM file")).expect("failed to load ROM file");

    println!("{:?} ROM", rom.header.version);
    println!("  Header:");
    println!("    TV System: {:?}", rom.header.tv_system);
    println!("    PRG ROM Banks: {}", rom.header.prg_rom_size);
    println!("    CHR ROM Banks: {}", rom.header.chr_rom_size);
    println!("    PRG RAM Size: {} bytes ({} bytes of which battery backed)", rom.header.prg_ram_size.total, rom.header.prg_ram_size.battery_backed);
    println!("    CHR RAM Size: {} bytes ({} bytes of which battery backed)", rom.header.chr_ram_size.total, rom.header.chr_ram_size.battery_backed);
    println!("    Cartridge Info:");
    println!("      Mapper: {}", rom.header.cartridge.mapper);
    println!("      Submapper: {}", rom.header.cartridge.submapper);
    println!("      Bus Conflicts?: {}", rom.header.cartridge.bus_conflicts);
    println!("    Use Vertical Arrangement?: {}", rom.header.vertical_arrangement);
    println!("    Four-Screen VRAM?: {}", rom.header.four_screen_vram);
    println!("    SRAM: {}", if !rom.header.sram_present { "None" } else if rom.header.sram_battery_backed { "Battery-Backed" } else { "Present" });
    println!("    Trainer Present?: {}", rom.header.trainer_present);
    println!("    Designed for Vs. Unisystem?: {}", rom.header.vs_unisystem);
    println!("    Designed for PlayChoice-10?: {}", rom.header.playchoice_10);
    println!("");
    println!("  Memory:");
    println!("    Total PRG ROM: {}", format_size(rom.prg.len()));
    println!("    Total CHR ROM: {}", format_size(rom.chr.len()));
}

fn format_size(size: usize) -> String {
    if size < 1024 {
        format!("{} bytes", size)
    } else if size < 1024*1024 {
        let val = size as f64 / 1024f64;
        format!("{} KB", val)
    } else {
        let val = size as f64 / (1024f64 * 1024f64);
        format!("{} MB", val)
    }
}
