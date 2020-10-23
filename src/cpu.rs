use crate::registers::Registers;
use crate::mmu::MMU;


pub struct CPU<'a>{
    mmu: &'a MMU,
    registers: Registers
}

impl<'a> CPU<'a>{

    pub fn new(mmu: &'a MMU) -> CPU<'a>{

        let cpu = CPU{
            mmu: mmu,
            registers: Registers::new()
        };

        return cpu;

    }

    // Execute instruction and return number of cycles spent 
    fn execute_instruction(instr : u8) -> u8{
        match instr {
            _ => {print!("Instruction"); 4}
        }
    }


}