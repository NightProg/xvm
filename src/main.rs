use crate::ast::Bits;
use crate::bytecode::*;
use crate::decoder::Decoder;
use crate::vm::cpu::Cpu;
use crate::vm::Mode;

mod decoder;
mod bytecode;
mod ast;
mod vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let bootloader = std::fs::read(args[1].clone()).unwrap();
    let context = sdl2::ttf::init().unwrap();
    let mut cpu = Cpu::new(&context);
    cpu.disk.write_sector(0, bootloader);
    cpu.init_bios();
    cpu.run();



}
