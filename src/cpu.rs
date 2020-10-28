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
            0x00 =>{println!("NOP "); 1}
            0x01 => {println!("LD BC u16"); let nn = self.read_next_word(); self.reg.set_bc(nn); 3}
            0x03 => {println!("INC BC"); let val = self.reg.get_bc() + 1; self.reg.set_bc(val); 2}
            0x04 => {println!("INC B"); self.reg.b = self.alu_inc(self.reg.b);  1 }
            0x05 => {println!("DEC B"); self.reg.b = self.alu_dec(self.reg.b) ; 1}
            0x06 => {println!("LD B, u8"); let val = self.read_next_byte(); self.reg.b = val; 2}
            0x07 => {println!("RLCA");  1} // TODO
            0x08 => {println!()} // TODO
            0x09 => {println!("ADD HL, BC"); ; 2}


            0x21 => {println!("d16 to HL"); let nn = self.read_next_word(); self.reg.set_hl(nn); 3}
            0x31 => {println!("Load to stack pointer"); self.reg.sp = self.read_next_word(); 3 }
            0x32 => {println!("LDD (HL), A"); let loc = self.reg.get_hld(); self.mmu.write_byte(loc, self.reg.a)  ;println!("PC IS AT {}", self.reg.pc); 2}

            0xAF => {println!("{:#x} XOR with A", instr); self.alu_xor(self.reg.a); 1}
            0xFE => {println!("Compare"); let n = self.read_next_byte(); self.alu_cmp(n); 2 }
            _ => {panic!("Unrecognized opcode") ; }
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

    fn alu_add(&mut self, n:u8) -> u8{
        let a = self.reg.a;
        let new_a = a + n;

        self.flags.H = ((a & 0x0f) + (n &0x0f)) & 0x10 > 0;
        self.flags.C =   (a as u16) + (n as u16) > 0xFF;
        self.flags.N = false;

        new_a
    }
 
    fn alu_dec(&mut self, cur_val: u8) -> u8{
        let dec_val = cur_val - 1;
        self.flags.Z = dec_val == 0;
        self.flags.N = true;
        self.flags.H = (self.reg.a & 0x0f) < (1 & 0x0f);
        dec_val
    }

    // Used only for 8-bit registers
    fn alu_inc(&mut self, cur_val: u8) -> u8{
        let inc_val = cur_val + 1;
        self.flags.H =  ((cur_val & 0x0f) + (1 & 0x0f)) & 0x10 > 0; // Check
        self.flags.Z = inc_val == 0;
        self.flags.N = false;
        inc_val
    }

    fn alu_addnn(&mut self, val: u16) -> u16{
        let cur_val = self.reg.get_hl();
        let new_val = cur_val + val;
        self.flags.Z = false;
        self.flags.H = ((cur_val & 0x0f00) + (val & 0x0f00)) & 0x1000 > 0;
        self.flags.C = (cur_val as u32) + (new_val as u32) > 0xFFFF;
        new_val
    }
}