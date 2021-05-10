
use core::time;

use crate::mmu::MMU;

pub struct Timer{
    div_helper: u16, // Helper to ensure  that the div register gets incremented every 255 cycles.
    time_helper: u16,
    div: u8,  // 0xFF04 - Divider Register
    tima: u8, // 0xFF05 - Timer Counter
    tma: u8,  // 0xFF06 - Timer Modulo
    tac: u8,  // 0xFF07 - Timer Control
    pub interrupt: u8 // 1 - true, 0 - false
}

impl Timer{

    pub fn new() -> Timer{
        let mut timer = Timer{
            div_helper: 0,
            time_helper:0,
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            interrupt: 0
        };
        timer
    }

    pub fn read_byte(&self, loc: u16) -> u8{
        match loc{
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => { panic!("Mem not in timer: read")  }
        }   
    }

pub fn write_byte(&mut self, loc: u16, val: u8){
        match loc{
            0xFF04 => { self.div = 0x00; self.div_helper = 0; self.time_helper = 0},
            0xFF05 => self.tima = val,
            0xFF06 => self.tma = val,
            0xFF07 => {self.tac = val; },
            0xFF0F => {
                if (val & 0x04) > 0 {
                    self.interrupt = 1;
                }
                else{
                    self.interrupt = 0;
                }
            }
            _ => {panic! ( "Mem not in timer: write")}
        }   
    }

pub fn step_cycle(&mut self, m_cycles: u8){
    
    self.div_helper += (m_cycles * 4  ) as u16; 
    if self.div_helper > 255 {
        self.div_helper -= 255;
        self.div = self.div.wrapping_add(1);
    }

    if ((self.tac >> 2) & 0x01) != 0{ // If timer is enabled

        self.time_helper = self.time_helper.wrapping_add((m_cycles * 4) as u16);

        let mut freq:u32 = 4096;
        if (self.tac & 3) == 1{
            freq = 262144;
        }
        else if(self.tac & 3) == 2{
            freq = 65536;
        }
        else if (self.tac & 3) == 3{
            freq = 16384;
        }

        while self.time_helper >= (4194304 / freq) as u16 {
            self.tima = self.tima.wrapping_add(1);
            if self.tima == 0{
                self.interrupt = 1;
                self.tima = self.tma;
            }
            self.time_helper -=  (4194304 / freq) as u16 ;
        }

    }
}

}