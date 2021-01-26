extern crate minifb;

use minifb::{Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

const OAM_CYCLES: u16 = 80;
const DRAW_CYCLES: u16 = 172;
const HBLANK_CYCLES:u16 = 204;
const VBANK_CYCLES: u16 = 4560;

const LCD_DISPLAY_ENABLE_MASK :u8 = 0b10000000;
const WINDOW_TILE_MAP_MASK: u8    = 0b01000000;
const WINDOW_DISPLAY_MASK: u8     = 0b00100000;

enum PPU_MODE{
    OAM,
    HBLANK,
    VBLANK,
    DRAW
}

pub struct PPU{
    ppu_clock: u16,
    mode: PPU_MODE,
    back_buffer: [u8; 160*144],
    window: Window,

    lcd_control: u8, // Addr: 0xFF40
    lcd_stat: u8, // Addr: 0xFF41 
    ly: u8, // Addr: 0xFF44 - Current Scanline number

    sprite_line_data: [u8; 10] // Used to hold sprite data for current line
}

impl PPU{
    pub fn new() -> PPU{
        PPU{
            ppu_clock: 0,
            mode: PPU_MODE::VBLANK, // Check : TODO
            back_buffer : [0; 160*144],
            window : Window::new(
                "Rust-gb",
                WIDTH,
                HEIGHT,
                WindowOptions::default(),
            ).unwrap_or_else(|e|{
                panic!("{}", e)
            }),
            lcd_control: 0x91,
            lcd_stat: 0xff,
            ly: 0,
            sprite_line_data: [0; 10]
        }
    }


    pub fn fill_scanline(&mut self){}

    pub fn ppu_step(&mut self, m_cycles: u8){
        
        self.ppu_clock += (m_cycles * 4) as u16; 

        match self.mode{
            PPU_MODE::OAM => {
                if self.ppu_clock >= OAM_CYCLES{
                    self.mode = PPU_MODE::DRAW;
                    self.ppu_clock %= OAM_CYCLES;
                }
            },
            PPU_MODE::DRAW => {
                if self.ppu_clock >= DRAW_CYCLES{
                    self.mode = PPU_MODE::HBLANK;
                    self.ppu_clock %=  DRAW_CYCLES;

                    self.fill_scanline(); // Temp only drawing background
                    
                }
            },
            PPU_MODE::HBLANK => {
                if self.ppu_clock >= HBLANK_CYCLES{

                    self.ly += 1; // Increment Scanline
                    self.ppu_clock %= HBLANK_CYCLES;

                    if(self.ly >= 143){

                        // Update Window's buffer with new data

                        self.mode = PPU_MODE::VBLANK;
                    }
                }
            },
            PPU_MODE::VBLANK => {
                if self.ppu_clock >= VBANK_CYCLES{
                    self.mode = PPU_MODE::OAM;
                    self.ppu_clock %= VBANK_CYCLES;
                }
            }


        }

    }


}