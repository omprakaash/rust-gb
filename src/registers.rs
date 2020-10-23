pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16
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
            pc:0
        }
    }

    fn get_bc() -> u16 {
        return 1
    }

    fn set_bc() {

    }

}