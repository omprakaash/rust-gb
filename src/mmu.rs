use std::fs::File;
use std::io;
use std::io::prelude::*;
pub struct MMU{
     pub mem: [u8;65536]
}

// Need to implement custom get and set operations for different mem regions
impl MMU{

    pub fn new(file: &String) -> MMU{
        let mut mmu = MMU{
            mem: [0;65536]
        };
        
        // Loading Rom
        let mut f= File::open(file).expect("Unable to open file");
        f.read(&mut mmu.mem).expect("Unable to read boot rom file");


        for (i, val) in mmu.mem.iter().enumerate(){
            if (i < 256){
                println!("{:x?}", val);
            }
            else{
                break;
            }
        }
        return mmu;   
    }

    pub fn read_byte(&self, loc: u16) -> u8{
        self.mem[loc as usize]
    }

    pub fn read_word(&self, loc: u16) -> u16{
        (self.mem[loc as usize] as u16) << 8 | (self.mem[(loc+1) as usize]) as u16 
    }

    pub fn write_byte(&mut self, loc: u16, val: u8){
        self.mem[loc as usize] = val;
    }


}