extern crate minifb;

use std::{borrow::Borrow, f32::consts::LOG2_E};
use crate::util::*;


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
const BG_MAP_SELECT_POS: u8      =  3;
const SPRITE_SIZE_BIT_POS: u8     = 2;
const SPRITE_ENABLE_MASK: u8      = 1;
const BG_WIND_ENABLE_BIT_POS: u8  = 0;
// ----------------------------------------------


// ---------------LCD STAT Bit Positions ----------
const STAT_LYC_BIT_POS: u8 = 6; 
const STAT_COINCIDENCE_BIT_POS:u8 = 2;


//-------------------------------------------------

const WHITE: u32 = 0x00ffffff;

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
    window: Window,
    vram: [u8; 8192],
    oam_mem: [u8; 160], // 0xFE00 - 0xFE9F

    lcd_control: u8,   // Addr: 0xFF40
    lcd_stat: u8,      // Addr: 0xFF41 
    scy: u8,           // Addr: 0xFF42
    scx: u8,           // Addr: 0xFF43
    ly: u8,            // Addr: 0xFF44 - Current Scanline number
    lyc: u8,           // Addr: 0xFF45
    bg_pallete: u8,    // Addr: 0xFF47
    obj_pallete_1: u8, // Addr: 0xFF48
    obj_pallete_2: u8, // Addr: 0xFF49

    colors: [u32; 4],
    sprite_line_data:[u8; 10], // Used to hold sprite data for current line

    pub interrupt: u8,

}

impl PPU{
    pub fn new() -> PPU{
        PPU{
            ppu_clock: 0,
            mode: PPU_MODE::VBLANK, // Check : TODO
            back_buffer : [0x00ffffffff; 160*144],
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
            oam_mem: [0; 160],
            lcd_control: 0x91,
            lcd_stat: 0xff,
            scx: 0,
            scy: 0,
            ly: 0,
            lyc: 0, 
            bg_pallete: 0xFC,
            obj_pallete_1: 0xFF,
            obj_pallete_2: 0xFF,
            colors: [0x00ffffff ,0x00A0A0A0,0x00555555, 0 ], // Minifb pixel data format
            sprite_line_data: [0; 10],
            interrupt: 0,
        }
    }


    pub fn fill_scanline(&mut self){

        // Render Background
        if test_bit_u8(self.lcd_control, BG_WIND_ENABLE_BIT_POS)  {
            let background_map_line = self.ly.wrapping_add(self.scy+1); // Check the +1
        
            // The row in the 32x32 tile map -> (32*8 X 32*8 pixel map)
            let tile_map_row = background_map_line / 8;

            // Line # in individual tile
            let line_in_tile = background_map_line % 8; // Need to account for 16 size tiles mode

            for tile_map_col in 0..=19{ // Need to account for SCX later on - 18 tiles in row of viewport map

                let background_map_base = if test_bit_u8(self.lcd_control, 3) {0x9C00} else {0x9800};
                let tile_num_addr: u16 = background_map_base + ( (tile_map_row) as u16* 32) as u16 + (tile_map_col as u16) - 0x8000;

                let tile_num = self.vram[tile_num_addr as usize];

                let tile_data_loc = if test_bit_u8(self.lcd_control, 4){ 
                    (0x8000 + (tile_num as u16 * 16) as u16) - 0x8000
                }
                else{
                    (0x8800 + (((tile_num as i8 as i16 + 128) as u16) * 16))  - 0x8000
                };

                //let tile_data_loc: u16 = 0x8000 + (tile_num as u16 * 16) as u16 - 0x8000;
            
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

        // Rendering Sprites

        // For each sprite in OAM check if sprite appears in current scanline
        if test_bit_u8(self.lcd_control, SPRITE_ENABLE_MASK){
            for oam_idx in 0..40{

                let y_pos:i32 = self.oam_mem[oam_idx*4] as i32 - 16;
                let x_pos:i32 = self.oam_mem[oam_idx*4 + 1] as i32 - 8;
                let mut tile_num = self.oam_mem[oam_idx*4 + 2];
                let sprite_attr = self.oam_mem[oam_idx*4 + 3];
                let sprite_y_size: i32 = if test_bit_u8(self.lcd_control, SPRITE_SIZE_BIT_POS){
                    16
                }else{
                    8
                };
                let x_flip = test_bit_u8(sprite_attr, 5 );
                let y_flip = test_bit_u8(sprite_attr, 6 ) ;
    
                //println!("Y_Pos: {}, sprite_y_size: {}, LY: {}", y_pos, sprite_y_size, self.ly);
    
                if self.ly >= (y_pos as u8) && (self.ly as i32) < y_pos + sprite_y_size{
    
                    //println!("Hello");
    
                    let mut line_in_sprite = self.ly - (y_pos as u8);
    
                    if y_flip{
                        line_in_sprite = sprite_y_size as u8 - line_in_sprite - 1 ;
                    }
    
                    if line_in_sprite as i32 >= sprite_y_size{
                        panic!("Line in sprite can not be greater than sprite size: LINE: {}, SPRITE SIZE: {} ", line_in_sprite, sprite_y_size);
                    }

                    if sprite_y_size == 16 {
                        if line_in_sprite < 8{
                            tile_num &= 0xFE;
                        }
                        else{
                            tile_num |= 0x01;
                        }
                    }
                    
                    let tile_data_loc = if test_bit_u8(self.lcd_control, 4){ 
                        (0x8000 + (tile_num as u16 * 16) as u16) - 0x8000
                    }
                    else{
                        (0x8800 + (((tile_num as i8 as i16 + 128) as u16) * 16))  - 0x8000
                    };
    
                    let row_byte_1 = self.vram[(tile_data_loc + (line_in_sprite * 2) as u16) as usize];
                    let row_byte_2 = self.vram[(tile_data_loc + (line_in_sprite * 2 + 1) as u16)  as usize];
                    let mut bit_mask = 0x80;
    
                    // Stepping through the 8 pixels in a sprites row
                    for sprite_col in 0..8{
                        //println!("X_pos: {}, sprite_col: {}", x_pos, sprite_col);
                        if(x_pos + sprite_col) >= 0{
                            let mut a = sprite_col;
                            if x_flip{
                                //println!("OLd {}", sprite_col);
                                a = 7 - sprite_col;
                            }
    
                            let high_bit = (row_byte_2 >> (7 - a )) & 0x01;
                            let low_bit = (row_byte_1  >> (7 -  a )) & 0x01;
                            let color_num = (high_bit << 1) | low_bit;
                            
                            let color_idx =  if test_bit_u8(sprite_attr, 4)
                            {
                                (self.obj_pallete_2 >> (color_num*2)) & 0x03
                            }else{
                                (self.obj_pallete_1 >> (color_num*2)) & 0x03
                            };
                            
                            let color = self.colors[color_idx as usize];
                            //println!("Drawing a sprite !");
                            if color != WHITE  {
                                self.back_buffer[ (self.ly as u16 * 160 + (x_pos + sprite_col) as u16) as usize ] = self.colors[color_idx as usize];
                            }
                        }
                        bit_mask = bit_mask >> 1;
                    }
    
                }
    
            }
        }
        

    }

    pub fn draw_frame(&mut self){
        self.window.update_with_buffer(&self.back_buffer, 160, 144).unwrap(); 
    }

    pub fn ppu_step(&mut self, m_cycles: u8){
        
        self.ppu_clock += (m_cycles * 4) as u16;
        match self.mode{
            PPU_MODE::OAM => {
                if self.ppu_clock >= OAM_CYCLES{
                    self.mode = PPU_MODE::DRAW;
                    self.ppu_clock %= OAM_CYCLES;
                }
                if self.interrupt == 1{
                    self.interrupt = 0;
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

                    if self.ly == self.lyc {
                        if test_bit_u8(self.lcd_stat,STAT_LYC_BIT_POS){
                            self.interrupt = 1;
                        } 
                        self.lcd_stat |= 0x01 << STAT_COINCIDENCE_BIT_POS;        
                    }

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

                    if self.ly > 153 {
                        self.ly = 0;
                        self.mode = PPU_MODE::OAM;
                    }

                }
            }


        }

    }

    pub fn write_byte(&mut self, loc: u16, val: u8){
        match loc{
            0xFE00..=0xFE9F => self.oam_mem[(loc - 0xFE00) as usize] = val,
            0xFF40 => { /*println!("Writing to lcdc: {:x?}", val); */self.lcd_control = val},
            0xFF41 => self.lcd_stat = self.lcd_stat | (val & 0xF4), // Bits 0 through 2 are read only and set only by the PPU
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => self.ly = val,
            0xFF45 => self.lyc = val,
            0xFF47 => self.bg_pallete = val,
            0xFF48 => {self.obj_pallete_1 = val; println!("Changing obj pallete 1 ") },
            0xFF49 => {self.obj_pallete_2 = val; println!("Changing obj pallete 2")},
            _ => self.vram[(loc - 0x8000) as usize] = val
        }
        
    }

    pub fn read_byte(&self, loc: u16) -> u8{
        match loc{
            0xFE00..=0xFE9F => self.oam_mem[(loc - 0xFE00) as usize],
            0xFF40 => self.lcd_control,
            0xFF41 => self.lcd_stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bg_pallete,
            0xFF48 => self.obj_pallete_1,
            0xFF49 => self.obj_pallete_2,
            _ => self.vram[(loc - 0x8000) as usize] 
        }
    }

}