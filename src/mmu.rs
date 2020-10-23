use std::fs;
pub struct MMU{
     mem: [u8;256]
}

// Need to implement custom get and set operations for different mem regions
impl MMU{

    pub fn new(file: &String) -> MMU{
        let mut mmu = MMU{
            mem: [0;256]
        };
        mmu.read_rom(file);
        return mmu;   
    }

    // Used to initialize boot rom portion
    pub fn read_rom(&mut self, file: &String){
        println!("Reading Rom File");
        let data = fs::read(file).expect("Unable to read boot ROM");

        for (i,op) in data.iter().enumerate(){
            self.mem[i] = *op;
        }
    }

}