extern crate minifb;

use std::f32::consts::LOG2_E;

use minifb::{Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

const OAM_CYCLES: u16 = 80;
const DRAW_CYCLES: u16 = 172;
const HBLANK_CYCLES:u16 = 204;
const VBANK_CYCLES: u16 = 456;

// -------------- LCDC Masks --------------------
const LCD_DISPLAY_ENABLE_MASK :u8 = 0b10000000;
const WINDOW_TILE_MAP_MASK: u8    = 0b01000000;
const WINDOW_DISPLAY_MASK: u8     = 0b00100000;
const TILE_DATA_SELECT_MASK: u8   = 0b00010000;
const BG_MAP_SELECT_MASK: u8      = 0b00001000;
const SPRITE_SIZE_MASK: u8        = 0b00000100;
const SPRITE_ENABLE_MASK: u8      = 0b00000010;
const BG_WIND_ENABLE_MASK: u8     = 0b00000001;
// ----------------------------------------------


enum PPU_MODE{
    OAM,
    HBLANK,
    VBLANK,
    DRAW
}

pub struct PPU{
    ppu_clock: u16,
    mode: PPU_MODE,
    back_buffer: [u32; 160*144],
    //debug_window: Window,
    window: Window,
    vram: [u8; 8192],

    lcd_control: u8, // Addr: 0xFF40
    lcd_stat: u8, // Addr: 0xFF41 
    ly: u8, // Addr: 0xFF44 - Current Scanline number
    scx: u8, // Addr: 0xFF43
    scy: u8, // Addr: 0xFF42
    bg_pallete: u8,// Addr: 0xFF47

    colors: [u32; 4],
    sprite_line_data: [u8; 10] // Used to hold sprite data for current line
}

impl PPU{
    pub fn new() -> PPU{
        PPU{
            ppu_clock: 0,
            mode: PPU_MODE::VBLANK, // Check : TODO
            back_buffer : [1; 160*144],
            window : Window::new(
                "Rust-gb",
                WIDTH,
                HEIGHT,
                WindowOptions::default(),
            ).unwrap_or_else(|e|{
                panic!("{}", e)
            }),
            /*debug_window : Window::new(
                "Tile Map",
                255,
                255,
                WindowOptions::default(),
            ).unwrap_or_else(|e|{
                panic!("{}", e)
            }),*/
            vram: [0; 8192],
            lcd_control: 0x91,
            lcd_stat: 0xff,
            ly: 0,
            scx: 0,
            scy: 0,
            bg_pallete: 0xFC,
            colors: [ 0x00ffffff,0x00C0C0C0,0x00606060, 0 ], // Minifb pixel data format
            sprite_line_data: [0; 10]
        }
    }


    pub fn fill_scanline(&mut self){
        let tile_map_line = self.ly.wrapping_add(self.scy+1); // Check the +1
        let line_in_tile = tile_map_line % 8; // Need to account for 16 size tiles mode
       
        //println!("SCY: {} ", self.scy);

        let tile_map_row = tile_map_line / 8;

        for tile_map_col in 0..=19{ // Need to account for SCX later on - 18 tiles in row of viewport map

            let tile_num_addr: u16 = 0x9800 + ( (tile_map_row) as u16* 32) as u16 + (tile_map_col as u16) - 0x8000;

            let tile_num = self.vram[tile_num_addr as usize];

            let tile_data_loc: u16 = 0x8000 + (tile_num as u16 * 16) as u16 - 0x8000;
           
            let row_byte_1 = self.vram[(tile_data_loc + (line_in_tile * 2) as u16) as usize];
            let row_byte_2 = self.vram[(tile_data_loc + (line_in_tile * 2 + 1) as u16)  as usize];

            let low = tile_map_col * 8;
            let high = low + 8;

            let mut bit_mask = 0x80;

            for i in low..high{

                let high_bit = (row_byte_2 & bit_mask) >> (7 - (i %8));
                let low_bit = (row_byte_1 & bit_mask) >> (7 - (i%8)); 

                let color_num = (high_bit << 1) | low_bit;
                
                let color_idx = ( self.bg_pallete >> (color_num*2) ) & 0x03;

                self.back_buffer[ ( (self.ly) as u16 *160 +  i)  as usize] = self.colors[color_idx as usize];
                bit_mask = bit_mask >> 1;

            }
        }
    }

    pub fn draw_frame(&mut self){
        self.window.update_with_buffer(&self.back_buffer, 160, 144).unwrap(); // Need to error handle.
    }

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

                    self.fill_scanline(); // Now only filling background. Need to add window and sprites later
                    
                }
            },
            PPU_MODE::HBLANK => {
                if self.ppu_clock >= HBLANK_CYCLES{

                    self.ly += 1; // Increment Scanline
                    self.ppu_clock %= HBLANK_CYCLES;

                    self.mode = PPU_MODE::OAM;

                    if self.ly == 143 {

                        // Update Window's buffer with new data                            
                        self.draw_frame();
                        self.mode = PPU_MODE::VBLANK;
                    }
                }
            },
            PPU_MODE::VBLANK => {
                if self.ppu_clock >= VBANK_CYCLES{
                    self.ppu_clock %= VBANK_CYCLES;
                    self.ly += 1;

                    if(self.ly > 153){
                        self.ly = 0;
                        self.mode = PPU_MODE::OAM;
                    }

                }
            }


        }

    }

    pub fn write_byte(&mut self, loc: u16, val: u8){
        match loc{
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => self.ly = val,
            0xFF47 => self.bg_pallete = val,
            _ => self.vram[(loc - 0x8000) as usize] = val
        }
        
    }

    pub fn read_byte(&self, loc: u16) -> u8{
        match loc{
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF47 => self.bg_pallete,
            _ => self.vram[(loc - 0x8000) as usize] 
        }
    }

}