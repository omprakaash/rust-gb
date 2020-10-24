use std::fs::File;
use std::io;
use std::io::prelude::*;
pub struct MMU{
     pub mem: [u8;256]
}

// Need to implement custom get and set operations for different mem regions
impl MMU{

    pub fn new(file: &String) -> MMU{
        let mut mmu = MMU{
            mem: [0;256]
        };
        
        // Loading Rom
        let mut f= File::open(file).expect("Unable to open file");
        f.read_exact(&mut mmu.mem).expect("Unable to read boot rom file");

        println!("{:x?}", mmu.mem);

        for val in mmu.mem.iter(){
            println!("{:#x?}", val);
        }
        return mmu;   
    }

    // Used to initialize boot rom portion

    pub fn read_word(&self, loc: u16) -> u16{
        (self.mem[loc as usize] as u16) << 8 | (self.mem[(loc+1) as usize]) as u16 
    }

}