use crate::registers::Registers;
use crate::registers::Flags;
use crate::mmu::MMU;

pub struct CPU<'a>{
    mmu: &'a MMU,
    reg: Registers,
    flags: Flags
}

impl<'a> CPU<'a>{

    pub fn new(mmu: &'a MMU) -> CPU<'a>{
        let cpu = CPU{
            mmu: mmu,
            reg: Registers::new(),
            flags: Flags::new()
        };
        return cpu;
    }


    pub fn start(&mut self){
        while(self.reg.pc < 256){
            self.execute_instruction();
        }
    }

    fn read_next_word(&mut self) -> u16{
        let val = self.mmu.read_word(self.reg.pc);
        self.reg.pc += 2;
        val
    }

    // Execute instruction and return number of cycles spent 
    fn execute_instruction(&mut self) -> u8{
        let instr: u8 = self.mmu.mem[self.reg.pc as usize ];
        match instr {
            0x31 => {print!("Load to stack pointer"); self.reg.pc += 1; self.reg.sp = self.read_next_word(); 3 }
            _ => {println!("{:#x?}: NYT", instr ); self.reg.pc+= 1;  1}
        }
    }

    
}