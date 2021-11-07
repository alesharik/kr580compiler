#[derive(Eq, PartialEq, Debug)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
}

impl RegisterPair {
    pub fn name(&self) -> &'static str {
        match self {
            RegisterPair::BC => "b",
            RegisterPair::DE => "d",
            RegisterPair::HL => "h",
            RegisterPair::SP => "sp",
        }
    }

    pub fn left_table_x_off(&self) -> u8 {
        match self {
            RegisterPair::BC => 0x00,
            RegisterPair::DE => 0x10,
            RegisterPair::HL => 0x20,
            RegisterPair::SP => 0x30,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M
}

impl Register {
    pub fn code_off(&self) -> u8 {
        match self {
            Register::A => 7,
            Register::B => 0,
            Register::C => 1,
            Register::D => 2,
            Register::E => 3,
            Register::H => 4,
            Register::L => 5,
            Register::M => 6,
        }
    }

    pub fn mov_base_off(&self) -> u8 {
        match self {
            Register::A => 0x78,
            Register::B => 0x40,
            Register::C => 0x48,
            Register::D => 0x50,
            Register::E => 0x58,
            Register::H => 0x60,
            Register::L => 0x68,
            Register::M => 0x70,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Register::A => "a",
            Register::B => "b",
            Register::C => "c",
            Register::D => "d",
            Register::E => "e",
            Register::H => "h",
            Register::L => "l",
            Register::M => "m",
        }
    }

    pub fn left_table_x_off(&self) -> u8 {
        match self {
            Register::A => 0x30,
            Register::B => 0x00,
            Register::C => 0x00,
            Register::D => 0x10,
            Register::E => 0x10,
            Register::H => 0x20,
            Register::L => 0x20,
            Register::M => 0x30,
        }
    }

    pub fn left_table_is_down(&self) -> bool {
        match self {
            Register::B | Register::D | Register::H | Register::M => false,
            Register::C | Register::E | Register::L | Register::A => true,
        }
    }
}

#[derive(Debug)]
pub enum MovArg {
    Register(Register),
    RegisterPair(RegisterPair),
    MemoryDirect(u16),
    MemoryIndirect(RegisterPair),
    Constant(u16),
}

#[derive(Debug)]
pub enum StatementKind {
    Nop,
    Mov(MovArg, MovArg),
    // cyclic <<
    Rlc,
    // cyclic >>
    Rrc,
    // carry <<
    Ral,
    // carry >>
    Rar,
    // set carry flag
    Stc,
    // invert carry flag
    Cmc,
    Add(Register),
    Adc(Register),
    Sub(Register),
    Sbb(Register),
    And(Register),
    Or(Register),
    Xor(Register),
    Cmp(Register),
    Jmp(String),
    Rst(u8),
    Out(u8),
    In(u8),
    Jz(String),
    Jnz(String),
    Jc(String),
    Jnc(String),
    Jpo(String),
    Jpe(String),
    Jp(String),
    Jm(String),
    Neg(Register),
    // set label to specific address
    Lset(u16),
}

#[derive(Debug)]
pub struct Statement {
    pub label: Option<String>,
    pub kind: StatementKind,
}

#[derive(Debug)]
pub struct File {
    pub statements: Vec<Box<Statement>>,
}