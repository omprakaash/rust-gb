mod cpu;
mod mmu;
mod registers;
mod instructions;
mod timer;
mod ppu;
mod cartridge;
mod util;

use cpu::CPU;
use mmu::MMU;
use std::env;
use std::time::{Instant};

extern crate minifb;
use minifb::{Key, Window, WindowOptions};


const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut mmu = MMU::new(&args[1]);
    let mut cpu = CPU::new(&mut mmu);

    let mut cycles:u16 = 0;
    let mut now = Instant::now();
    loop
    {
            cycles = cycles.wrapping_add(cpu.cpu_step() as u16);
            if cycles >= 17496 {
                cycles = 0;
                while now.elapsed().as_millis() < (16.67 as u128){
                    
                }
            }
            
    }

}
