extern crate remy;

use std::{env,fs,io};
use std::path::Path;
use std::io::Write;

use remy::hw::mos6502;
use remy::systems::nes;

fn main() {
    // Load the rom
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: decompiler <rom file name>");
        return;
    }

    let file = Path::new(&args[1]);
    println!("Loading {:?}...", file);

    let rom = nes::load_rom(&mut fs::File::open(file).unwrap()).unwrap();

    // Determine the output file and delete it if already present
    let output_file = Path::new(file.file_name().unwrap()).with_extension("out");
    println!("Decompiling to {:?}...", output_file);
    let mut output = fs::File::create(output_file).unwrap();

    writeln!(&mut output, "Decompilation of {:?}", file.file_name().unwrap()).unwrap();
    writeln!(&mut output, "{}", "").unwrap();

    decompile_banks(&mut output, "PRG", rom.prg_banks);
    write_banks(&mut output, "CHR", rom.chr_banks);
}

fn decompile_banks<W>(output: &mut W, bank_type: &'static str, banks: Vec<Vec<u8>>) where W: io::Write {
    for (i, bank) in banks.iter().enumerate() {
        writeln!(output, "{} ROM Bank {}", bank_type, i).unwrap();

        let mut cursor = io::Cursor::new(&bank[..]);

        loop {
            let pos = cursor.position();
            match mos6502::instr::decode(&mut cursor) {
                Ok(i) => writeln!(output, "  0x{:04X}  {}", pos, i).unwrap(),
                Err(mos6502::instr::decoder::Error::EndOfFile) => break,
                Err(e) => {
                    println!("Error decoding at 0x{:04X}: {:?}", pos, e);
                    break;
                }
            }
        }
    }
}

fn write_banks<W>(output: &mut W, bank_type: &'static str, banks: Vec<Vec<u8>>) where W: io::Write {
    for (i, bank) in banks.iter().enumerate() {
        writeln!(output, "{} ROM Bank {}", bank_type, i).unwrap();

        for (i, x) in bank.iter().enumerate() {
            if i % 16 == 0 {
                write!(output, "  0x{:04X} : ", i).unwrap();
            }
            write!(output, "0x{:02X} ", x).unwrap();
            if i % 16 == 15 {
                writeln!(output, "{}", "").unwrap();
            }
        }
        writeln!(output, "{}", "").unwrap();
    }
}
