pub enum Instruction{
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