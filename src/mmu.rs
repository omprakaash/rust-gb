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
    serial_interrupt: u8,
    cartridge: Cartridge
}

// Need to implement custom get and set operations for different mem regions
impl MMU{

    pub fn new(file: &String) -> MMU{
        let mut mmu = MMU{
            mem: [0;65536],
            timer: Timer::new(),
            ppu: PPU::new(),
            serial_interrupt: 0,
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

            0xFE00..=0xFE9F | 0xFF40 | 0xFF42 | 0xFF43 | 0xFF44| 0xFF45 |0xFF47..=0xFF49 | VRAM_START..=VRAM_END => {
                self.ppu.read_byte(loc)
            }

            // To counter bug in cpu_instr test
            0xFF4D =>{
                0xFF
            }

            0xFF04..=0xFF07 => {
                self.timer.read_byte(loc)
            },
            0xFF0F => {
                //let mut ret = self.mem[loc as usize];
                let mut ret = 0xE0;
                ret = ret | (self.ppu.vblank_interrupt) | (self.ppu.stat_interrupt << 1)  |(self.timer.interrupt << 2) | (self.serial_interrupt << 3);
               
                if ((ret >> 2) & 1) == 1{
                    println!("Finally a timer INTERRUPT");
                }

                ret
            },
            _ => self.mem[loc as usize]
        }

    }

    pub fn read_word(&self, loc: u16) -> u16{
        /*self.mem[loc as usize] as u16 | ((self.mem[(loc.wrapping_add(1)) as usize] as u16) << 8)*/
        self.read_byte(loc) as u16 | ((self.read_byte(loc.wrapping_add(1)) as u16) << 8)
    }

    fn update_interrupts(&mut self, new_if: u8){
        self.timer.write_byte(0xFF0F, new_if);
        
        // Only supporting seral and timer interrupts for now
        if new_if & (1 << 3) > 0 {
            self.serial_interrupt = 1;
        }
        else{
            self.serial_interrupt = 0;
        }

    }

    pub fn write_byte(&mut self, loc: u16, val: u8){
        match loc{
            0x0000..=0x7FFF => {
                self.cartridge.write_byte(loc, val);
            }
            0xFE00..=0xFE9F | 0xFF40 | 0xFF42 | 0xFF43 | 0xFF44  | 0xFF45| 0xFF47..=0xFF49 | VRAM_START..=VRAM_END => {
                self.ppu.write_byte(loc, val);
            }
            0xFF04..=0xFF07  => {
                self.timer.write_byte(loc, val);
            }
            0xFF0F => {
                self.update_interrupts(val);
            }
            _ => {self.mem[loc as usize] = val;}
        }
    }
    
    pub fn write_word(&mut self, loc: u16, val: u16){
        self.mem[loc as usize] = (val & 0xFF) as u8;
        self.mem[(loc + 1) as usize] = (val >> 8) as u8;
    }

}