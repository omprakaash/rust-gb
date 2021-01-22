use std::{fs::File, io::Read};

use crate::timer::Timer;

pub struct MMU{
    pub mem: [u8;65536],
    timer: Timer
}

// Need to implement custom get and set operations for different mem regions
impl MMU{

    pub fn new(file: &String) -> MMU{
        let mut mmu = MMU{
            mem: [0;65536],
            timer: Timer::new()
        };
        
        // Loading Rom
        let mut f= File::open(file).expect("Unable to open file");
        f.read(&mut mmu.mem).expect("Unable to read boot rom file");

        for (i, val) in mmu.mem.iter().enumerate(){
            
            //print!("{:x?}", val);
        }
        print!("\n");
        mmu.write_byte(0xFF0F, 0xE0);
        return mmu;   
    }

    pub fn step(&mut self, m_cycles: u8){
        self.timer.step_cycle(m_cycles);
    }

    pub fn read_byte(&self, loc: u16) -> u8{

        match loc{
            0xFF04..=0xFF07 => {
                self.timer.read_byte(loc)
            },
            0xFF0F => {
                let mut ret = self.mem[loc as usize];
                ret = ret | (self.timer.interrupt << 2);
                ret
            },
            _ => self.mem[loc as usize]
        }

    }

    pub fn read_word(&self, loc: u16) -> u16{
        self.mem[loc as usize] as u16 | ((self.mem[(loc+1) as usize] as u16) << 8)
    }

    pub fn write_byte(&mut self, loc: u16, val: u8){
        /*if loc == 0xFF44{
            println!("Writing to Loc: 0xFF44");
        }
        if val == 0x90{
            println!("Val == 0x90");
        }*/
        match loc{
            
            0xFF0F | 0xFF04..=0xFF07  => {
                self.timer.write_byte(loc, val);
            }
            _ => {}
        }

        self.mem[loc as usize] = val;
        
    }

    pub fn write_word(&mut self, loc: u16, val: u16){
        self.mem[loc as usize] = (val & 0xFF) as u8;
        self.mem[(loc + 1) as usize] = (val >> 8) as u8;
    }


}