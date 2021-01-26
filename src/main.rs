mod cpu;
mod mmu;
mod registers;
mod instructions;
mod timer;
mod ppu;

use cpu::CPU;
use mmu::MMU;
use ppu::PPU;
use timer::Timer;
use registers::Registers;
use std::env;

extern crate minifb;
use minifb::{Key, Window, WindowOptions};


const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut mmu = MMU::new(&args[1]);
    let mut cpu = CPU::new(&mut mmu);

    loop
    {
        cpu.cpu_step();
    }

}
