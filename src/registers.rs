use crate::instructions;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16
}

pub struct Flags{
    pub Z: bool,
    pub N: bool,
    pub H: bool,
    pub C: bool
}

impl Flags{
    pub fn new() -> Flags{
        Flags{
            Z: false,
            N: false,
            H: false,
            C: false
        }
    }
}

impl Registers{

    pub fn new() -> Registers{
        Registers{
            a:0,
            b:0,
            c:0,
            d:0,
            e:0,
            h:0,
            l:0,
            pc:0,
            sp:0
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

}