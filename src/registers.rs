pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f : u8,
    pub pc: u16,
    pub sp: u16
}

impl Registers{

    pub fn new() -> Registers{
        Registers{
            a:0x01,
            b:0,
            c:0x13,
            d:0,
            e:0xd8,
            h:0x01,
            l:0x4d,
            f:0xB0,
            pc:0x0100,
            sp:0xfffe,
        }
    }

    pub fn get_zero(&self) -> bool {
         (self.f & 0x80) >> 7 == 1
    }

    pub fn get_neg(&self) -> bool{
         (self.f & 0x40) >> 6 == 1
    }

    pub fn get_half(&self) -> bool {
        (self.f & 0x20) >> 5 == 1
    }

    pub fn get_carry(&self) -> bool {
        (self.f & 0x10) >> 4 == 1
    }

    pub fn set_zero(&mut self, set: bool){
        let mask = 0x80;
        if set{
            self.f = self.f | mask;            
        }
        else{
            self.f = self.f & (! mask);
        }
    }

    pub fn set_neg(&mut self, set: bool){
        let mask = 0x40;
        if set{
            self.f = self.f | mask; 
        }
        else{
            self.f = self.f & (!mask);
        }
    }

    pub fn set_half(&mut self, set: bool){
        let mask = 0x20;
        if set{
            self.f = self.f | mask; 
        }
        else{
            self.f = self.f & (!mask);
        }
    }

    pub fn set_carry(&mut self, set: bool){
        let mask = 0x10;
        if set{
            self.f = self.f | mask; 
        }
        else{
            self.f = self.f & (!mask);
        }
    }

    pub fn get_bc(&self) -> u16 {
        return (self.b as u16) << 8 | self.c as u16;
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = ((val & 0xFF00) >> 8) as u8;
        self.c = (val & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        return (self.d as u16) << 8 | self.e as u16;
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = ((val & 0xFF00) >> 8) as u8;
        self.e = (val & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        return ((self.h as u16) << 8) | self.l as u16;
    }

    pub fn get_hld(&mut self) -> u16{
        let ret = self.get_hl();
        self.set_hl(ret - 1);
        ret
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = ((val & 0xFF00) >> 8) as u8;
        self.l = (val & 0xFF) as u8;
    }

    pub fn set_af(&mut self, val: u16){
        self.a = ((val & 0xFF00) >> 8) as u8;
        self.f = (val & 0xF0) as u8; // Discard the 4 least sig bits of f
    }

    pub fn get_af(&self) -> u16{
        ((self.a as u16) << 8) | self.f as u16
    }

}