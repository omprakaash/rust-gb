use crate::registers::Registers;
use crate::registers::Flags;
use crate::mmu::MMU;
use crate::instructions;

pub struct CPU<'a>{
    mmu: &'a mut  MMU,
    reg: Registers,
    instMap: instructions::InstructionMap<'a>,
    halted: bool
}

impl<'a> CPU<'a>{

    pub fn new(mmu: &'a mut MMU) -> CPU<'a>{
        let cpu = CPU{
            mmu: mmu,
            reg: Registers::new(),
            instMap : instructions::InstructionMap::new(),
            halted: false // Temp solution
        };
        return cpu;
    }


    pub fn start(&mut self){
        while !self.halted {
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

        //println!("pc: {:#06X?}, OP: {:X?}, Z: {}, N: {},  C: {}, AF: {:#06X?}, BC: {:#06X?}, SP: {:#06X?}, HL: {:#06X?}, DE: {:#06X?}", self.reg.pc, instr, self.reg.get_zero(), self.reg.get_neg(), self.reg.get_carry(), self.reg.get_af(),self.reg.get_bc(), self.reg.sp, self.reg.get_hl(), self.reg.get_de());

        // Print to debug
        //self.instMap.printInstruction(instr);

        //print!("{}", self.mmu.read_byte(0xFF02));
        if self.mmu.read_byte(0xff02) == 0x81 {
            let c: char = self.mmu.read_byte(0xff01) as char;
            print!("{}", c );
            self.mmu.write_byte(0xff02, 0x00);
        }

        let cur_pc = self.reg.pc;
        self.reg.pc += 1;
        match instr {
            0x00 => { 1}
            0x01 => { let nn = self.read_next_word(); self.reg.set_bc(nn); 3}
            0x03 => { let val = self.reg.get_bc().wrapping_add(1); self.reg.set_bc(val); 2}
            0x04 => { self.reg.b = self.alu_inc(self.reg.b);  1 }
            0x05 => { self.reg.b = self.alu_dec(self.reg.b) ; 1}
            0x06 => {let val = self.read_next_byte(); self.reg.b = val; 2}
            0x07 => {  self.alu_rlca(); 1} 
            0x08 => {let mem_address = self.read_next_word(); self.mmu.write_word(mem_address, self.reg.sp); 5} 
            0x09 => { let val = self.alu_addnn(self.reg.get_bc()); self.reg.set_hl(val) ; 2}
            0x0A => { let val = self.mmu.read_byte(self.reg.get_bc()); self.reg.a = val; 2}
            0x0B => { let val = self.reg.get_bc() - 1; self.reg.set_bc(val); 2 }
            0x0C => { self.reg.c = self.alu_inc(self.reg.c); 1}
            0x0D => { self.reg.c = self.alu_dec(self.reg.c); 1}
            0x0E => { let n = self.read_next_byte(); self.reg.c = n; 2}
            0x0F => { self.alu_rrca(); 1} // TODO - SET CARRY BIT ?

            0x10 => { self.halted = true; 2 } // TODO
            0x11 => { let nn = self.read_next_word(); self.reg.set_de(nn); 3}
            0x12 => { self.mmu.write_byte(self.reg.get_de(), self.reg.a); 2}
            0x13 => {let val = self.reg.get_de() + 1; self.reg.set_de(val) ; 2}
            0x14 => { self.reg.d = self.alu_inc(self.reg.d); 1 }
            0x15 => { self.reg.d = self.alu_dec(self.reg.d) ; 1}
            0x16 => { self.reg.d = self.read_next_byte() ; 2}
            0x17 => { self.alu_rla() ; 1} 
            0x18 => { self.jr(); 3}
            0x19 => { let val = self.alu_addnn(self.reg.get_de()); self.reg.set_hl(val); 2}
            0x1A => { self.reg.a = self.mmu.read_byte(self.reg.get_de()); 2}
            0x1B => {  let val = self.reg.get_de().wrapping_sub(1); self.reg.set_de(val) ; 2}
            0x1C => { self.reg.e = self.alu_inc(self.reg.e); 1}
            0x1D => { self.reg.e = self.alu_dec(self.reg.e) ; 1}
            0x1E => { self.reg.e = self.read_next_byte(); 2}
            0x1F => { self.alu_rra(); 1} // TODO ??

            0x20 => {
                 
                if !self.reg.get_zero() {
                    self.jr();
                    3
                } 
                else{
                    //print!( "NZ NOT JUMPING \n\n\n\n\n\n");
                    self.reg.pc += 1;
                    3
                } 
            }
            0x21 => {let nn = self.read_next_word(); self.reg.set_hl(nn); 3}
            0x22 => { self.mmu.write_byte(self.reg.get_hl(), self.reg.a); self.inc_hl(); 2} // TODO
            0x23 => { let val = self.reg.get_hl() + 1; self.reg.set_hl(val); 2}
            0x24 => { self.reg.h = self.alu_inc(self.reg.h); 1}
            0x25 => { self.reg.h = self.alu_dec(self.reg.h); 1}
            0x26 => { self.reg.h = self.read_next_byte(); 2}
            0x27 => { 1 } // TODO - OP: DAA
            0x28 => {
                if self.reg.get_zero() {
                    self.jr();
                    3
                } 
                else{
                    self.reg.pc += 1;
                    3
                } 
            } // TODO
            0x29 => { let val = self.alu_addnn(self.reg.get_hl()); self.reg.set_hl(val) ;  2}
            0x2A => {let val = self.reg.get_hl(); self.reg.a = self.mmu.read_byte(val); self.reg.set_hl(val + 1); 2 } // TODO
            0x2B => { let val = self.reg.get_hl() - 1; self.reg.set_hl(val); 2  }
            0x2C => { self.reg.l = self.alu_inc(self.reg.l); 1}
            0x2D => { self.reg.l = self.alu_inc(self.reg.l); 1}
            0x2E => { self.reg.l = self.read_next_byte(); 2}
            0x2F => { self.alu_cpl(); 1 } // TODO


            // TODO : Check on get_hld()
            0x30 => {
                if ! self.reg.get_carry(){
                    self.jr();
                    3
                }
                else{
                    self.reg.pc += 1;
                    2
                }
            } 
            0x31 => { self.reg.sp = self.read_next_word(); 3 }
            0x32 => {let loc = self.reg.get_hld(); self.mmu.write_byte(loc, self.reg.a)  ; 2}
            0x33 => {let val = self.reg.get_hl() + 1; self.reg.set_hl(val); 2}
            0x34 => { let loc = self.reg.get_hl(); let new_val = self.mmu.read_byte(loc) + 1; self.mmu.write_byte(loc, new_val)  ; 3  }
            0x35 => { let loc = self.reg.get_hl(); let new_val = self.mmu.read_byte(loc).wrapping_sub(1); self.mmu.write_byte(loc, new_val)  ; 3  }
            0x36 => { let val = self.read_next_byte(); self.mmu.write_byte(self.reg.get_hl(), val); 3}
            0x37 => { self.scf() ; 1}
            0x38 => { 
                if self.reg.get_carry(){
                    self.jr();
                    3
                }
                else{
                    //print!("NOT JUMPING \n\n\n\n\n");
                    self.reg.pc += 1;
                    2
                }
            } 
            0x39 => { let val = self.alu_addnn(self.reg.sp); self.reg.set_hl(val); 2 }
            0x3A => { let val = self.reg.get_hl(); self.reg.a = self.mmu.read_byte(val); self.reg.set_hl(val - 1) ;2}
            0x3B => { self.reg.sp -= 1; 2 }
            0x3C => { self.reg.a = self.alu_inc(self.reg.a); 1}
            0x3D => { self.reg.a = self.alu_dec(self.reg.a); 1}
            0x3E => { self.reg.a = self.read_next_byte(); 2}
            0x3F => { self.ccf(); 1} 

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
            0x76 => { self.halted = true; 1 } // TODO: HALT
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

            0xA0 => { self.alu_and(self.reg.b); 1 }
            0xA1 => { self.alu_and(self.reg.c); 1 }
            0xA2 => { self.alu_and(self.reg.d); 1 }
            0xA3 => { self.alu_and(self.reg.e); 1 }
            0xA4 => { self.alu_and(self.reg.h); 1 }
            0xA5 => { self.alu_and(self.reg.l); 1 }
            0xA6 => { self.alu_and(self.mmu.read_byte(self.reg.get_hl())); 2 }
            0xA7 => { self.alu_and(self.reg.a); 1 }
            0xA8 => { self.alu_xor(self.reg.b); 1 }
            0xA9 => { self.alu_xor(self.reg.c); 1 }
            0xAA => { self.alu_xor(self.reg.d); 1 }
            0xAB => { self.alu_xor(self.reg.e); 1 }
            0xAC => { self.alu_xor(self.reg.h); 1 }
            0xAD => { self.alu_xor(self.reg.l); 1 }
            0xAE => { self.alu_xor(self.mmu.read_byte(self.reg.get_hl())); 2 }
            0xAF => { self.alu_xor(self.reg.a); 1 }

            0xB0 => { self.alu_or(self.reg.b); 1 }
            0xB1 => { self.alu_or(self.reg.c); 1 }
            0xB2 => { self.alu_or(self.reg.d); 1 }
            0xB3 => { self.alu_or(self.reg.e); 1 }
            0xB4 => { self.alu_or(self.reg.h); 1 }
            0xB5 => { self.alu_or(self.reg.l); 1 }
            0xB6 => { self.alu_or(self.mmu.read_byte(self.reg.get_hl())); 2 }
            0xB7 => { self.alu_or(self.reg.a); 1 }
            0xB8 => { self.alu_cmp(self.reg.b); 1}
            0xB9 => { self.alu_cmp(self.reg.c); 1}
            0xBA => { self.alu_cmp(self.reg.d); 1}
            0xBB => { self.alu_cmp(self.reg.e); 1}
            0xBC => { self.alu_cmp(self.reg.h); 1}
            0xBD => { self.alu_cmp(self.reg.l); 1}
            0xBE => { self.alu_cmp(self.mmu.read_byte(self.reg.get_hl())); 2}
            0xBF => { self.alu_cmp(self.reg.a); 1}

            // RET NZ - Check Timing 
            0xC0 => {  
                if !self.reg.get_zero()  {
                    let ret_address = self.pop();
                    self.reg.pc = ret_address;
                }
                2
            }

            0xC1 => {let val = self.pop(); self.reg.set_bc(val) ; 3}
            0xC2 => {
                if !self.reg.get_zero() {
                    let jp_address = self.read_next_word();
                    self.reg.pc = jp_address;
                }
                else{
                    self.reg.pc += 2;
                }
                3
            }
            0xC3 => {
                let jp_address = self.read_next_word();
                //print!("Jumping to {:x?}", jp_address);
                self.reg.pc = jp_address;
                3
            }
            0xC4 => {
                if !self.reg.get_zero(){
                
                    // Pushing address of next instr onto stack
                    self.push(self.reg.pc + 2);
    
                    // Jumping to jpAddress
                    self.reg.pc = self.read_next_word();
                    4
                }
                else{
                    self.reg.pc += 2;
                    3
                }
            }
            0xC5 => { self.push(self.reg.get_bc()); 4}
            0xC6 => { let val = self.read_next_byte(); self.reg.a = self.alu_add(val); 2 }
            0xC7 => { self.push(cur_pc); self.reg.pc = 0x0000; 4}
            0xC8 => {  
                if self.reg.get_zero(){
                    let ret_address = self.pop();
                    self.reg.pc = ret_address;
                }
                2
            }
            0xC9 => {
                self.reg.pc = self.pop();
                4
            }
            0xCA => {
                if self.reg.get_zero(){
                    self.reg.pc = self.read_next_word();
                    4
                }
                else{
                    self.reg.pc += 2;
                    3
                }
            }
            0xCB =>{
                //print! ("PREFIX CB");
                1
            }
            0xCC => {
                if self.reg.get_zero(){
                    self.push(self.reg.pc + 2);
                    self.reg.pc = self.read_next_word();
                    6
                }
                else { 
                    self.reg.pc += 2;
                    3 }
            }
            0xCD => {
                self.push(self.reg.pc + 2);
                self.reg.pc = self.read_next_word();
                6
            }
            0xCE => {let val = self.read_next_byte(); self.reg.a = self.alu_adc(val) ; 2}
            0xCF => { self.push(cur_pc); self.reg.pc = 0x0008; 4}

            0xD0 => { 
                if ! self.reg.get_carry() {
                    self.reg.pc = self.pop(); 
                    5
                }  
                else{
                    2
                }
            }
            0xD1 => { let val = self.pop(); self.reg.set_de(val); 3  }
            0xD2 => {
                if !self.reg.get_carry(){
                    let jp_address = self.read_next_word();
                    self.reg.pc = jp_address;
                }
                else{
                    self.reg.pc += 2;
                }
                3
            }
            0xD4 => {
                if !self.reg.get_carry() {
                
                    // Pushing address of next instr onto stack
                    self.push(self.reg.pc + 2);
    
                    // Jumping to jpAddress
                    self.reg.pc = self.read_next_word();
                    4
                }
                else{
                    self.reg.pc += 2;
                    3
                }
            }
            0xD5 => {self.push(self.reg.get_de()); 4 }
            0xD6 => {let val = self.read_next_byte(); self.reg.a = self.alu_sub(val); 2 }
            0xD7 => { self.push(cur_pc); self.reg.pc = 0x0010; 4  }
            0xD8 => {
                if self.reg.get_carry() {
                    self.reg.pc = self.pop();
                    5
                }
                else {
                    2
                 }
            }
            0xD9 => { // TODO - Interrupts
                1
            }
            0xDA => {
                if self.reg.get_carry() {
                    self.reg.pc= self.read_next_word();
                    4
                }
                else{
                    self.reg.pc += 2;
                    3
                }
            }
            0xDC => {
                if self.reg.get_carry() {
                    self.push(self.reg.pc + 2);
                    self.reg.pc = self.read_next_word();
                    6
                }
                else{
                    self.reg.pc += 2;
                    3
                } 
            }
            0xDE => { let val = self.read_next_byte(); self.reg.a = self.alu_sbc(val); 2}
            0xDF => { self.push(cur_pc); self.reg.pc = 0x0018; 4}

            0xE0 => { let val = self.read_next_byte() as u16; self.mmu.write_byte(0xff00 + val, self.reg.a); 3 }
            0xE1 => { let val = self.pop(); self.reg.set_hl(val); 3}
            0xE2 => { self.mmu.write_byte(0xff00 + (self.reg.c as u16), self.reg.a); 2}
            0xE5 => { self.push(self.reg.get_hl()); 4 }
            0xE6 => { let val = self.read_next_byte(); self.alu_and(val); 2}
            0xE7 => { self.push(cur_pc); self.reg.pc = 0x0020; 4 }
            0xE8 => { 4 } // TODO
            0xE9 => { self.reg.pc = self.reg.get_hl(); 2  }
            0xEA => { let write_address = self.read_next_word(); self.mmu.write_byte(write_address, self.reg.a); 4 }
            0xEE => { let val = self.read_next_byte(); self.alu_xor(val); 2}
            0xEF => { self.push(cur_pc); self.reg.pc = 0x0028; 4}

            0xF0 => { let val = self.read_next_byte() as u16; self.reg.a = self.mmu.read_byte(0xFF00 + val); 3}
            0xF1 => { let val = self.pop(); self.reg.set_af(val); 3}
            0xF2 => { self.reg.a = self.mmu.read_byte(0xff00 + (self.reg.c as u16)); 2 }
            0xF3 => { 1 } // TODO - Disable Interrupts after the next instruction
            0xF5 => { self.push(self.reg.get_af()); 4 }
            0xF6 => { let val = self.read_next_byte(); self.alu_or(val); 2 }
            0xF7 => { self.push(cur_pc); self.reg.pc = 0x0030; 4 }
            0xF8 => { 3 } // TODO
            0xF9 => { self.reg.sp = self.reg.get_hl(); 2}
            0xFA => { let mem_address = self.read_next_word(); let val = self.mmu.read_byte(mem_address); self.reg.a = val; 4}
            0xFB => {1} // TODO - Enable interrupts after the next instruction
            0xFE => { let val = self.read_next_byte(); self.alu_cmp(val); 2}
            0xFF => { self.push(cur_pc); self.reg.pc = 0x0038; 4}

            _ => {panic!("Unrecognized opcode") ; }
        }
        
    }


    fn push(&mut self, val: u16){
        self.reg.sp -= 1;
        self.mmu.write_byte(self.reg.sp, ((val & 0xFF00) >> 8) as u8);
        
        self.reg.sp -= 1;
        self.mmu.write_byte(self.reg.sp, (val & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16{
        let low_byte = self.mmu.read_byte(self.reg.sp);
        self.reg.sp += 1;

        let high_byte = self.mmu.read_byte(self.reg.sp);
        self.reg.sp += 1;

        ((high_byte as u16) << 8) | (low_byte as u16)
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
        self.reg.set_half((self.reg.a & 0xf) < (n & 0xf)) ;
        self.reg.set_carry(self.reg.a < n);
        let new_val = self.reg.a.wrapping_sub(n);
        // Setting Flags
        self.reg.set_zero(new_val == 0);
        self.reg.set_neg(true);
        new_val
    }

    fn alu_sbc(&mut self, n: u8) -> u8{
        match self.reg.get_carry(){
            true => self.alu_sub(n + 1),
            false => self.alu_sub(n)
        }
    }

    fn alu_cmp(&mut self, val: u8 ){
       self.alu_sub(val);
    } 

    fn alu_xor(&mut self, val: u8) {
        self.reg.a = self.reg.a ^ val;
        self.reg.set_zero(self.reg.a == 0);
        self.reg.set_carry(false);
        self.reg.set_half(false);
        self.reg.set_neg(false);
    }

    fn alu_adc(&mut self, n: u8) -> u8 {
        match self.reg.get_carry(){
            true => self.alu_add(n + 1),
            false => self.alu_add(n)
        }
    }

    fn alu_add(&mut self, n:u8) -> u8{
        let a = self.reg.a;
        let new_a = a.wrapping_add(n);

        self.reg.set_zero(new_a == 0);
        self.reg.set_half(((a & 0x0f) + (n &0x0f)) & 0x10 > 0);
        self.reg.set_carry( (a as u16) + (n as u16) > 0xFF); // Check calc
        self.reg.set_neg(false);

        new_a
    }
 
    fn alu_dec(&mut self, cur_val: u8) -> u8{
        let dec_val = cur_val.wrapping_sub(1);
        self.reg.set_zero( dec_val == 0);
        self.reg.set_neg(true);
        self.reg.set_half((self.reg.a & 0x0f) < (1 & 0x0f));
        dec_val
    }

    // Used only for 8-bit registers
    fn alu_inc(&mut self, cur_val: u8) -> u8{
        let inc_val = cur_val.wrapping_add(1) ;
        self.reg.set_half(((cur_val & 0x0f) + (1 & 0x0f)) & 0x10 > 0); // Check
        self.reg.set_zero( inc_val == 0);
        self.reg.set_neg(false);
        //println!("INC VAL E: {}", inc_val);
        inc_val
    }

    fn alu_addnn(&mut self, val: u16) -> u16{
        let cur_val = self.reg.get_hl();
        let new_val = cur_val.wrapping_add(val);
        self.reg.set_zero(false);
        self.reg.set_half(((cur_val & 0x0f00) + (val & 0x0f00)) & 0x1000 > 0);
        self.reg.set_carry( (cur_val as u32) + (new_val as u32) > 0xFFFF);
        new_val
    }

    fn alu_and(&mut self, val: u8){
        self.reg.a = self.reg.a & val;
        self.reg.set_zero( self.reg.a == 0);
        self.reg.set_neg(false);
        self.reg.set_half(true);
        self.reg.set_carry(false) ;
    }

    fn alu_or(&mut self, val: u8){
        self.reg.a = self.reg.a | val;
        self.reg.set_zero(self.reg.a == 0);
        self.reg.set_carry(false) ;
        self.reg.set_half(false);
        self.reg.set_neg(false);
    }

    fn scf(&mut self){
        self.reg.set_carry(true);
        self.reg.set_neg(false);
        self.reg.set_half(true);
    }

    fn ccf(&mut self){
        self.reg.set_carry( ! self.reg.get_carry()); // Flip carry bit
        self.reg.set_half(false);
        self.reg.set_neg(false);
    }

    fn alu_rlca(&mut self){
        let old_bit_7 = self.reg.a >> 7;
        self.reg.set_carry(old_bit_7 == 1);
        self.reg.set_half(false);
        self.reg.set_neg(false);
        self.reg.set_zero( false);

        self.reg.a = (self.reg.a << 1) | old_bit_7;
    }

    fn alu_rla(&mut self){ // Check
        let carry = match self.reg.get_carry(){
            false => 0x00,
            true => 0x01,
        };
        self.reg.set_zero(false);
        self.reg.set_half(false);
        self.reg.set_neg(false);
        self.reg.set_carry((self.reg.a >> 7) == 1);

        self.reg.a = ((self.reg.a) << 1) | carry;
    }

    fn alu_rra(&mut self){

        let carry = match self.reg.get_carry(){
            false => 0x00,
            true => 0x80,
        };

        self.reg.set_carry((self.reg.a & 0x01) == 1);
        self.reg.set_zero(false);
        self.reg.set_half(false);
        self.reg.set_neg(false);

        self.reg.a = ((self.reg.a) >> 1) | carry;

    }

    fn alu_rrca(&mut self){ // Check
        let old_bit_0 = self.reg.a << 7;
        self.reg.set_half(false);
        self.reg.set_neg(false);
        self.reg.set_zero( false);
        self.reg.set_carry(old_bit_0 == 0x80);

        self.reg.a = ((self.reg.a) << 1) | old_bit_0;

    }

    fn jr(&mut self){
        let inc = self.read_next_byte() as i8;
        let cur_pc = self.reg.pc ;
        ////print!("Increment: {}\n", inc);
        //print!("\nCur pc is : {:x?} ", cur_pc );
       
        //self.reg.pc = (((self.reg.pc - 1 ) as u32 as i32) + (inc as i32)) as u16;
        self.reg.pc = cur_pc.wrapping_add(inc as u16);
        //print!("\nNew pc is : {:x?} ", self.reg.pc  );
    }

    fn alu_cpl(&mut self){
        self.reg.a = ! self.reg.a;
        self.reg.set_neg(true);
        self.reg.set_half(true);
    }

}