use crate::registers::Registers;
use crate::registers::Flags;
use crate::mmu::MMU;
use crate::instructions;

pub struct CPU<'a>{
    mmu: &'a mut  MMU,
    reg: Registers,
    flags: Flags,
    instMap: instructions::InstructionMap<'a>
}

impl<'a> CPU<'a>{

    pub fn new(mmu: &'a mut MMU) -> CPU<'a>{
        let cpu = CPU{
            mmu: mmu,
            reg: Registers::new(),
            flags: Flags::new(),
            instMap : instructions::InstructionMap::new()
        };
        return cpu;
    }


    pub fn start(&mut self){
        while(self.reg.pc < 256){
            self.parse_instruction();
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
    fn parse_instruction(&mut self) -> u8{
        let instr: u8 = self.mmu.mem[self.reg.pc as usize ];

        // Print to debug
        self.instMap.printInstruction(instr);


        self.reg.pc += 1;
        match instr {
            0x00 => {println!("NOP "); 1}
            0x01 => {println!("LD BC u16"); let nn = self.read_next_word(); self.reg.set_bc(nn); 3}
            0x03 => {println!("INC BC"); let val = self.reg.get_bc() + 1; self.reg.set_bc(val); 2}
            0x04 => {println!("INC B"); self.reg.b = self.alu_inc(self.reg.b);  1 }
            0x05 => {println!("DEC B"); self.reg.b = self.alu_dec(self.reg.b) ; 1}
            0x06 => {println!("LD B, u8"); let val = self.read_next_byte(); self.reg.b = val; 2}
            0x07 => {println!("RLCA");  1} // TODO - set CARRY Bit ?
            0x08 => {3} // TODO
            0x09 => {println!("ADD HL, BC"); let val = self.alu_addnn(self.reg.get_bc()); self.reg.set_hl(val) ; 2}
            0x0A => {println!("LD A, (BC)"); let val = self.mmu.read_byte(self.reg.get_bc()); self.reg.a = val; 2}
            0x0B => {println!("DEC BC"); let val = self.reg.get_bc() - 1; self.reg.set_bc(val)  ;2 }
            0x0C => {println!("INC C"); self.reg.c = self.alu_inc(self.reg.c); 1}
            0x0D => {println!("DEC C"); self.reg.c = self.alu_dec(self.reg.c); 1}
            0x0E => {println!("LD C, u8"); let n = self.read_next_byte(); self.reg.c = n ;2}
            0x0F => {println!("RRCA"); 1} // TODO - SET CARRY BIT ?

            0x10 => {println!("STOP"); 2 } // TODO
            0x11 => {println!("LD DE, u16"); let nn = self.read_next_word(); self.reg.set_de(nn); 3}
            0x12 => {println!("LD (DE), A"); self.mmu.write_byte(self.reg.get_de(), self.reg.a); 2}
            0x13 => {println!("INC DE");let val = self.reg.get_de() + 1; self.reg.set_de(val) ; 2}
            0x14 => {println!("INC D"); self.reg.d = self.alu_inc(self.reg.d); 1 }
            0x15 => {println!("DEC D"); self.reg.d = self.alu_dec(self.reg.d) ; 1}
            0x16 => {println!("LD D, u8"); self.reg.d = self.read_next_byte() ; 2}
            0x17 => {println!("RLA") ; 1} // TODO
            0x18 => {println!("JR i8"); ; 3} // TODO and check timing
            0x19 => {println!("ADD HL, DE"); let val = self.alu_addnn(self.reg.get_de()); self.reg.set_hl(val); 2}
            0x1A => {println!("LD A, (DE)"); self.reg.a = self.mmu.read_byte(self.reg.get_de()); 2}
            0x1B => {println!("DEC DE"); let val = self.reg.get_de() - 1; self.reg.set_de(val) ; 2}
            0x1C => {println!("INC E"); self.reg.e = self.alu_inc(self.reg.e); 1}
            0x1D => {println!("DEC E"); self.reg.e = self.alu_dec(self.reg.e) ; 1}
            0x1E => {println!("LD E, u8"); self.reg.e = self.read_next_byte(); 2}
            0x1F => {println!("RRA");  1} // TODO

            0x20 => {println!("JR NZ, i8"); 3 } // TODO - check for variable cycle count
            0x21 => {println!("d16 to HL"); let nn = self.read_next_word(); self.reg.set_hl(nn); 3}
            0x22 => {println!("LD (HL+), A"); self.mmu.write_byte(self.reg.get_hl(), self.reg.a); self.inc_hl(); 2} // TODO
            0x23 => { let val = self.reg.get_hl() + 1; self.reg.set_hl(val); 2}
            0x24 => { self.reg.h = self.alu_inc((self.reg.h)); 1}
            0x25 => { self.reg.h = self.alu_dec(self.reg.h); 1}
            0x26 => { self.reg.h = self.read_next_byte(); 2}
            0x27 => { 1 } // TODO - OP: DAA
            0x28 => { 3} // TODO
            0x29 => { let val = self.alu_addnn(self.reg.get_hl()); self.reg.set_hl(val) ;  2}
            0x2A => {let val = self.reg.get_hl(); self.reg.a = self.mmu.read_byte(val); self.reg.set_hl(val - 1); 2 } // TODO
            0x2B => { let val = self.reg.get_hl() - 1; self.reg.set_hl(val); 2  }
            0x2C => { self.reg.l = self.alu_inc(self.reg.l); 1}
            0x2D => { self.reg.l = self.alu_inc(self.reg.l); 1}
            0x2E => { self.reg.l = self.read_next_byte(); 2}
            0x2F => { 1 } // TODO


            // TODO : Check on get_hld()
            0x30 => {3} // TODO
            0x31 => {println!("Load to stack pointer"); self.reg.sp = self.read_next_word(); 3 }
            0x32 => {println!("LDD (HL), A"); let loc = self.reg.get_hld(); self.mmu.write_byte(loc, self.reg.a)  ;println!("PC IS AT {}", self.reg.pc); 2}
            0x33 => {let val = self.reg.get_hl() + 1; self.reg.set_hl(val); 2}
            0x34 => { self.mmu.write_byte(self.reg.get_hl(), self.alu_inc(self.mmu.read_byte(self.reg.get_hl()))); 3}    
            0x35 => { self.mmu.write_byte(self.reg.get_hl(), self.alu_dec(self.mmu.read_byte(self.reg.get_hl()))); 3}
            0x36 => { self.mmu.write_byte(self.reg.get_hl(), self.read_next_byte()); 3}
            0x37 => {1} // TODO: SCF
            0x38 => {3} // TODO JR
            0x39 => { let val = self.alu_addnn(self.reg.sp); self.reg.set_hl(val); 2 }
            0x3A => { let val = self.reg.get_hl(); self.reg.a = self.mmu.read_byte(val); self.reg.set_hl(val - 1) ;2}
            0x3B => { self.reg.sp -= 1; 2 }
            0x3C => { self.reg.a = self.alu_inc(self.reg.a); 1}
            0x3D => { self.reg.a = self.alu_dec(self.reg.a); 1}
            0x3E => { self.reg.a = self.read_next_byte(); 2}
            0x3F => { 1 } // TODO: CCF

            0xAF => {println!("{:#x} XOR with A", instr); self.alu_xor(self.reg.a); 1}
            0xFE => {println!("Compare"); let n = self.read_next_byte(); self.alu_cmp(n); 2 }
            _ => {panic!("Unrecognized opcode") ; }
        }
        
    }


    fn ldi(&mut self){ // Necessary ?

    }

    fn inc_bc(&mut self){
        let val = self.reg.get_bc();
        self.reg.set_hl(val);
    }

    fn inc_hl(&mut self){
        let val = self.reg.get_hl() + 1;
        self.reg.set_hl(val);
    }

    fn dec_hl(&mut self){
        let val = self.reg.get_hl() - 1;
        self.reg.set_hl(val);
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