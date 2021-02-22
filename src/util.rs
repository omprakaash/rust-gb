use std::ops::{BitAnd, Shr};

pub fn test_bit_u8(val: u8, bit: u8) -> bool{
    (val >> bit) & 1 == 1
}