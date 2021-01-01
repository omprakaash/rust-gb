use std::collections::HashMap;

pub struct InstructionMap<'a>{
    assemblyMap: HashMap<u8, &'a str>
}

impl<'a> InstructionMap<'a>{

    pub fn new() -> InstructionMap<'a>{
        let hashMap = [
            (0x00, "NOP"),
            (0x01, "LD BC, u16"),
            (0x02, "LD (BC), A"),
            (0x03, "inC BC"),
            (0x04, "INC B"),
            
        
        
        
        
        
        ].iter().cloned().collect();
        InstructionMap{
            assemblyMap: hashMap
        }
    }


    pub fn printInstruction(&self, opCode: u8){
        match self.assemblyMap.get(&opCode){
            Some(instr)=>println!("{}", instr),
            None => println!("Unknown Instruction")
        }
    }

}

pub enum Instructions{
    ADD{op1: Operand, op2: Operand},
    ADC,
    SUB,
    SUBC,
    DEC{op: Operand},
    INC{op: Operand},
    OR,
    XOR,
    CP{op: Operand},
    LD{op1: Operand, op2: Operand },
}

pub enum Operand{
    OP8,
    OP16,
    n8,
    n16
}

pub enum OP8{ // Operand
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum OP16{
    BC,
    DE,
    HL,
    SP,
    PC,
}