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

            0x40 => { 1  } // just loads b to b ( does nothing ) - ld b, b
            0x41 => { self.reg.b = self.reg.c; 1}
            0x42 => { self.reg.b = self.reg.d; 1 }
            0x43 => { self.reg.b = self.reg.e; 1}
            0x44 => { self.reg.b = self.reg.h; 1}
            0x45 => { self.reg.b = self.reg.l; 1}
            0x46 => { self.reg.b = self.mmu.read_byte(self.reg.get_hl()); 2 }
            0x47 => { self.reg.b = self.reg.a; 1}
            0x48 => { self.reg.c = self.reg.b; 1}
            0x49 => { 1 } // Redundant - ld c, c
            0x4A => { self.reg.c = self.reg.d; 1}
            0x4B => { self.reg.c = self.reg.e ; 1 }
            0x4C => { self.reg.c = self.reg.h; 1}
            0x4D => { self.reg.c = self.reg.l; 1}
            0x4E => { self.reg.c = self.mmu.read_byte(self.reg.get_hl()); 2}
            0x4F => { self.reg.c = self.reg.a; 1}

            0x50 => { self.reg.d = self.reg.b; 1}
            0x51 => { self.reg.d = self.reg.c; 1}
            0x52 => { 1 } // ld d, d
            0x53 => { self.reg.d = self.reg.e; 1}
            0x54 => { self.reg.d = self.reg.h; 1}
            0x55 => { self.reg.d = self.reg.l; 1}
            0x56 => { self.reg.d = self.mmu.read_byte(self.reg.get_hl()); 2}
            0x57 => { self.reg.d = self.reg.a; 1}
            0x58 => { self.reg.e = self.reg.b; 1}
            0x59 => { self.reg.e = self.reg.c; 1}
            0x5A => { self.reg.e = self.reg.d; 1}
            0x5B => { 1 } // ld e, e
            0x5C => { self.reg.e = self.reg.h; 1}
            0x5D => { self.reg.e = self.reg.l; 1}
            0x5E => { self.reg.e = self.mmu.read_byte(self.reg.get_hl()); 2}
            0x5F => { self.reg.e = self.reg.a; 1}

            0x60 => { self.reg.h = self.reg.b; 1}
            0x61 => { self.reg.h = self.reg.c; 1}
            0x62 => { self.reg.h = self.reg.d; 1 } // ld d, d
            0x63 => { self.reg.h = self.reg.e; 1}
            0x64 => { 1 } // LD h, h
            0x65 => { self.reg.h = self.reg.l; 1}
            0x66 => { self.reg.h = self.mmu.read_byte(self.reg.get_hl()); 2}
            0x67 => { self.reg.h = self.reg.a; 1}
            0x68 => { self.reg.l = self.reg.b; 1}
            0x69 => { self.reg.l = self.reg.c; 1}
            0x6A => { self.reg.l = self.reg.d; 1}
            0x6B => { self.reg.l = self.reg.e; 1 } // ld e, e
            0x6C => { self.reg.l = self.reg.h; 1}
            0x6D => { 1 } // ld l, l
            0x6E => { self.reg.l = self.mmu.read_byte(self.reg.get_hl()); 2}
            0x6F => { self.reg.l = self.reg.a; 1}

            0x70 => { self.mmu.write_byte(self.reg.get_hl(), self.reg.b); 2 }
            0x71 => { self.mmu.write_byte(self.reg.get_hl(), self.reg.c); 2 }
            0x72 => { self.mmu.write_byte(self.reg.get_hl(), self.reg.d); 2 }
            0x73 => { self.mmu.write_byte(self.reg.get_hl(), self.reg.e); 2 }
            0x74 => { self.mmu.write_byte(self.reg.get_hl(), self.reg.h); 2 }
            0x75 => { self.mmu.write_byte(self.reg.get_hl(), self.reg.l); 2 }
            0x76 => { 1 } // TODO: HALT
            0x77 => { self.mmu.write_byte(self.reg.get_hl(), self.reg.a); 2 }
            0x78 => { self.reg.a = self.reg.b; 1 }
            0x79 => { self.reg.a = self.reg.c; 1 }
            0x7A => { self.reg.a = self.reg.d; 1 }
            0x7B => { self.reg.a = self.reg.e; 1 } 
            0x7C => { self.reg.a = self.reg.h; 1 }
            0x7D => { self.reg.a = self.reg.l; 1 } 
            0x7E => { self.reg.a = self.mmu.read_byte(self.reg.get_hl()); 2 }
            0x7F => { 1 } // ld a, a

            0x80 => { self.reg.a = self.alu_add(self.reg.b); 1 }
            0x81 => { self.reg.a = self.alu_add(self.reg.c); 1 }
            0x82 => { self.reg.a = self.alu_add(self.reg.d); 1 }
            0x83 => { self.reg.a = self.alu_add(self.reg.e); 1 }
            0x84 => { self.reg.a = self.alu_add(self.reg.h); 1 }
            0x85 => { self.reg.a = self.alu_add(self.reg.l); 1 }
            0x86 => { self.reg.a = self.alu_add(self.mmu.read_byte(self.reg.get_hl())); 2 }
            0x87 => { self.reg.a = self.alu_add(self.reg.a); 1 }
            0x88 => { self.reg.a = self.alu_adc(self.reg.b); 1 }
            0x89 => { self.reg.a = self.alu_adc(self.reg.c); 1 }
            0x8A => { self.reg.a = self.alu_adc(self.reg.d); 1 }
            0x8B => { self.reg.a = self.alu_adc(self.reg.e); 1 }
            0x8C => { self.reg.a = self.alu_adc(self.reg.h); 1 }
            0x8D => { self.reg.a = self.alu_adc(self.reg.l); 1 }
            0x8E => { self.reg.a = self.alu_adc(self.mmu.read_byte(self.reg.get_hl())); 2 }
            0x8F => { self.reg.a = self.alu_adc(self.reg.a); 1 }

            0x90 => { self.reg.a = self.alu_sub(self.reg.b); 1}
            0x91 => { self.reg.a = self.alu_sub(self.reg.c); 1}
            0x92 => { self.reg.a = self.alu_sub(self.reg.d); 1}
            0x93 => { self.reg.a = self.alu_sub(self.reg.e); 1}
            0x94 => { self.reg.a = self.alu_sub(self.reg.h); 1}
            0x95 => { self.reg.a = self.alu_sub(self.reg.l); 1}
            0x96 => { self.reg.a = self.alu_sub(self.mmu.read_byte(self.reg.get_hl())); 2}
            0x97 => { self.reg.a = self.alu_sub(self.reg.a); 1}
            0x98 => { self.reg.a = self.alu_sbc(self.reg.b); 1}
            0x99 => { self.reg.a = self.alu_sbc(self.reg.c); 1}
            0x9A => { self.reg.a = self.alu_sbc(self.reg.d); 1}
            0x9B => { self.reg.a = self.alu_sbc(self.reg.e); 1}
            0x9C => { self.reg.a = self.alu_sbc(self.reg.h); 1}
            0x9D => { self.reg.a = self.alu_sbc(self.reg.l); 1}
            0x9E => { self.reg.a = self.alu_sbc(self.mmu.read_byte(self.reg.get_hl())); 2}
            0x9F => { self.reg.a = self.alu_sbc(self.reg.a); 1}



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
    fn alu_sub(&mut self, n: u8) -> u8{
        self.flags.H = (self.reg.a & 0xf) < (n & 0xf) ;
        self.flags.C = self.reg.a < n;
        let new_val = self.reg.a.wrapping_sub(n);
        // Setting Flags
        self.flags.Z = self.reg.a == 0;
        self.flags.N = true;
        new_val
    }

    fn alu_sbc(&mut self, n: u8) -> u8{
        match self.flags.C{
            true => self.alu_sub(n + 1),
            false => self.alu_sub(n)
        }
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

    fn alu_adc(&mut self, n: u8) -> u8 {
        match self.flags.C{
            true => self.alu_add(n + 1),
            false => self.alu_add(n)
        }
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

    fn alu_and()
}