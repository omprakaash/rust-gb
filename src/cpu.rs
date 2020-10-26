use crate::registers::Registers;
use crate::registers::Flags;
use crate::mmu::MMU;

pub struct CPU<'a>{
    mmu: &'a mut  MMU,
    reg: Registers,
    flags: Flags
}

impl<'a> CPU<'a>{

    pub fn new(mmu: &'a mut MMU) -> CPU<'a>{
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

    fn read_next_byte(&mut self) -> u8{
        let val = self.mmu.read_byte(self.reg.pc);
        self.reg.pc += 1;
        val
    }

    fn read_next_word(&mut self) -> u16{
        let val = self.mmu.read_word(self.reg.pc);
        self.reg.pc += 2; // Move pc after reading next word(u16)
        val
    }

    // Execute instruction and return number of cycles spent 
    fn execute_instruction(&mut self) -> u8{
        let instr: u8 = self.mmu.mem[self.reg.pc as usize ];
        self.reg.pc += 1;
        match instr {

            0x21 => {println!("d16 to HL"); let d16 = self.read_next_word(); self.reg.set_hl(d16); 3}
            0x31 => {println!("Load to stack pointer"); self.reg.sp = self.read_next_word(); 3 }
            0x32 => {println!("LDD (HL), A"); let loc = self.reg.get_hld(); self.mmu.write_byte(loc, self.reg.a)  ;println!("PC IS AT {}", self.reg.pc); 2}

            0xAF => {println!("{:#x} XOR with A", instr); self.alu_xor(self.reg.a); 1}
            0xFE => {println!("Compare"); let n = self.read_next_byte(); self.alu_cmp(n); 2 }
            _ => {println!("{:#x?}: NYT", instr );  1}
        }
        
    }

 
    // Might want to change to account for sbc or just create a sbc function
    fn alu_sub(&mut self, n: u8){
        self.flags.H = (self.reg.a & 0xf) < (n & 0xf) ;
        self.flags.C = self.reg.a < n;
        self.reg.a = self.reg.a.wrapping_sub(n);
        // Setting Flgs
        self.flags.Z = self.reg.a == 0;
        self.flags.N = true;
    }

    fn alu_cmp(&mut self, val: u8 ) -> u8{
        1
    } 

    fn alu_xor(&mut self, val: u8) {
        self.reg.a = self.reg.a ^ val;
        self.flags.Z = self.reg.a == 0;
        self.flags.C = false;
        self.flags.H = false;
        self.flags.N = false;
    }
 
}