mod cpu;
mod mmu;
mod registers;

use cpu::CPU;
use mmu::MMU;
use registers::Registers;
use std::env;

fn main() {
    print!("Hello !");
    let args: Vec<String> = env::args().collect();


    let mmu = MMU::new(&args[0]);
    let cpu = CPU::new(&mmu);

}
