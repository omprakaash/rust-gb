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

    println!("{}", &args[1]);
    let mmu = MMU::new(&args[1]);
    let mut cpu = CPU::new(&mmu);

    cpu.start();

}
