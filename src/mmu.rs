use std::{fs::File, io::Read};

use crate::{cartridge, timer::Timer};
use crate::ppu::PPU;
use crate::cartridge::Cartridge;

const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9fff;

pub struct MMU{
    pub mem: [u8;65536],
    timer: Timer,
    ppu: PPU,
    cartridge: Cartridge
}

// Need to implement custom get and set operations for different mem regions
impl MMU{

    pub fn new(file: &String) -> MMU{
        let mut mmu = MMU{
            mem: [0;65536],
            timer: Timer::new(),
            ppu: PPU::new(),
            cartridge: Cartridge::new(file)
        };
        mmu.write_byte(0xFF0F, 0xE0);
        return mmu;   
    }

    pub fn step(&mut self, m_cycles: u8){
        self.timer.step_cycle(m_cycles);
        self.ppu.ppu_step(m_cycles);
    }

    pub fn read_byte(&self, loc: u16) -> u8{

        match loc{
            0x0000..=0x7FFF =>{ // Check the end value ( inclusive or exclusive )
                self.cartridge.read_byte(loc)
            }

            0xFF42 | 0xFF43 | 0xFF44| 0xFF47| VRAM_START..=VRAM_END => {
                self.ppu.read_byte(loc)
            }

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
        /*self.mem[loc as usize] as u16 | ((self.mem[(loc.wrapping_add(1)) as usize] as u16) << 8)*/
        self.read_byte(loc) as u16 | ((self.read_byte(loc.wrapping_add(1)) as u16) << 8)
    }

    pub fn write_byte(&mut self, loc: u16, val: u8){
        match loc{
           
            0x0000..=0x7FFF => {
                self.cartridge.write_byte(loc, val);
            }

            0xFF42 | 0xFF43 | 0xFF44 | 0xFF47 | VRAM_START..=VRAM_END => {
                self.ppu.write_byte(loc, val);
            }
            0xFF0F | 0xFF04..=0xFF07  => {
                self.timer.write_byte(loc, val);
            }

            _ => {self.mem[loc as usize] = val;}
        }
    }
    
    pub fn write_word(&mut self, loc: u16, val: u16){
        self.mem[loc as usize] = (val & 0xFF) as u8;
        self.mem[(loc + 1) as usize] = (val >> 8) as u8;
    }

}