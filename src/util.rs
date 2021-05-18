use std::ops::{BitAnd, Shr};

pub fn test_bit_u8(val: u8, bit: u8) -> bool{
    (val >> bit) & 1 == 1
}

pub fn set_bit_u8(val : &mut u8, bit: u8){
    *val |= 0x01 << bit
}