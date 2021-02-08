use std::{fs::File, io::Read};

// Supports No MBC and MBC-1
pub struct Cartridge{
    mbc_type: u8, // Type of mbc
    rom_size: u32,
    ram_size: u32,
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    mbc_mode: u8, // Ram mode or Rom mode
    storage: Vec<u8>,
}

impl Cartridge{
    pub fn new(file_name: &String) -> Cartridge{
        let mut init_hdr =  [0; 0x014F]; // Temp structure to parse cartridge header

        let mut f = File::open(file_name).expect("Unable to open rom file");
        f.read(&mut init_hdr).expect("Error loading init_hdr");

        f = File::open(file_name).expect("HAHA");

        let rom_size:u32 = (init_hdr[0x0148] + 1 ) as u32  * 0x7FFF + 1 ; // currently not supporting fractional MB

        println!("Size of external RAM: {} ", init_hdr[0x0149] );

        let mbc_type = match init_hdr[0x0147]{
            0x00 => {0}
            0x01..=0x03 => {1}
            _ => {0}
        };

        println!("Size of cartidge: {}", rom_size);

        let mut cartridge: Cartridge = Cartridge{
            mbc_type: mbc_type,
            rom_size: rom_size,
            ram_size: init_hdr[0x0149] as u32,
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            mbc_mode: 0,
            storage: vec![0; rom_size as usize]
        };
        
        cartridge.load_rom(f);

        cartridge
    }

    pub fn load_rom(&mut self, mut file: File){
        file.read(&mut self.storage).expect("Could not load rom into storage");
    }

    pub fn write_byte(&mut self, loc: u16, val: u8){
        if self.mbc_type != 0 {

            match loc{

                0x0000..=0x1FFF => {
                    println!("Writing to RAM enable register: {:2x?}", val);
                    let lower_4_bits = val & 0x0F;
                    if lower_4_bits == 0x0A{
                        self.ram_enabled = true;
                    }
                    else{
                        self.ram_enabled = false;
                    }
                    
                }

                0x2000..=0x3FFF => {
                    println!("VAL: {}", val);
                    let low_5_bits = match val & 0x1F{ // Check the +1 logic
                        0x0 => {
                            println!("Bank numbers last 5 bits is zero");
                            1
                        }
                        _ => val & 0x1F
                    };
                    //print!("Previous value of Rom Bank# : {:02X?}", self.rom_bank);
                    self.rom_bank = ((self.rom_bank & 0xE0) | low_5_bits) % 4;
                    println!(" Low 5 bits: {:02X? }, Rom Bank# set to: {:02X?}", low_5_bits, self.rom_bank);
                    if self.rom_bank == 0{
                        self.rom_bank += 1;
                    }
                    
                }

                0x4000..=0x5FFF => {


                    // If ROM mode - Change upper 2 bits of rom_bank
                    if self.mbc_mode == 0{
                        print!("Changing rom Bank _ upper 2 bits with val: {:02X}", val);
                        self.rom_bank = (self.rom_bank & 0x1F) | ((val & 0x03) << 5) ;
                        if self.rom_bank == 0{
                            self.rom_bank += 1;
                        }
                    }
                    else{
                        println!("Maybe need to impl RAM banking");
                    }
                }

                0x6000..=0x8000 => {
                    println!("Changing modes");
                    if val & 0x01 == 0 {
                        self.mbc_mode = 0;
                    }
                    else{
                        self.mbc_mode = 1;
                    }
                }

                _ => {
                    println!("Writing to another men regioon at loc: {:04X?}", loc);
                    self.storage[loc as usize] = val;
                }

            }


        }
        else{
            println!("Writing to cartridge when no MBC is present");
        }
    }

    pub fn read_byte(&self, loc: u16) -> u8{

        if self.mbc_type == 0{
            self.storage[loc as usize]
        }
        else{// Mbc type 1
            match loc{
                0x0000..=0x3FFF => {  self.storage[loc as usize] }
                0x4000..=0x7FFF => {
                    let rom_offset: u32 =(self.rom_bank  )as u32 * 0x4000 ;
                    //println!("ROM BANK: {}, loc: {:04X?}", self.rom_bank, loc);
                    let addr: usize = (rom_offset + (loc - 0x4000) as u32) as usize;
                    //println!("Addr: {:04X?}", addr);
                    self.storage[ (rom_offset + (loc - 0x4000) as u32) as usize]
                }
                _ => { 1 }        
            }
        }
        
    }

}

    
