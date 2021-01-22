mod cpu;
mod mmu;
mod registers;
mod instructions;
mod timer;

use cpu::CPU;
use mmu::MMU;
use timer::Timer;
use registers::Registers;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut mmu = MMU::new(&args[1]);
    let mut cpu = CPU::new(&mut mmu);

    cpu.start();

}
