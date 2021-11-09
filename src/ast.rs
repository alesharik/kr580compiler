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

    pub fn push_code(&self) -> u8 {
        match self {
            RegisterPair::HL => 0xE5,
            RegisterPair::DE => 0xD5,
            RegisterPair::BC => 0xC5,
            RegisterPair::SP => unreachable!(),
        }
    }

    pub fn pop_code(&self) -> u8 {
        match self {
            RegisterPair::HL => 0xE1,
            RegisterPair::DE => 0xD1,
            RegisterPair::BC => 0xC1,
            RegisterPair::SP => unreachable!(),
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

#[derive(Eq, PartialEq, Debug)]
pub enum JmpType {
    Jmp,
    Jz,
    Jnz,
    Jc,
    Jnc,
    Jm,
    Jp,
    Jpo,
    Jpe,
    Call,
    Cz,
    Cnz,
    Cc,
    Cnc,
    Cpo,
    Cpe,
    Cp,
    Cm,
    Ret,
    Rz,
    Rnz,
    Rc,
    Rnc,
    Rpo,
    Rpe,
    Rp,
    Rm,
}

impl JmpType {
    pub fn code(&self) -> u8 {
        match self {
            JmpType::Jmp => 0xC3,
            JmpType::Jz => 0xCA,
            JmpType::Jnz => 0xC2,
            JmpType::Jc => 0xDA,
            JmpType::Jnc => 0xD2,
            JmpType::Jm => 0xFA,
            JmpType::Jp => 0xF2,
            JmpType::Jpo => 0xE2,
            JmpType::Jpe => 0xEA,
            JmpType::Call => 0xCD,
            JmpType::Cz => 0xCC,
            JmpType::Cc => 0xDC,
            JmpType::Cpe => 0xED,
            JmpType::Cm => 0xFD,
            JmpType::Cnz => 0xC4,
            JmpType::Cnc => 0xD4,
            JmpType::Cpo => 0xE4,
            JmpType::Cp => 0xF4,
            JmpType::Ret => 0xC9,
            JmpType::Rz => 0xC8,
            JmpType::Rc => 0xD8,
            JmpType::Rpe => 0xE8,
            JmpType::Rm => 0xF8,
            JmpType::Rnz => 0xC0,
            JmpType::Rnc => 0xD0,
            JmpType::Rpo => 0xE0,
            JmpType::Rp => 0xF0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            JmpType::Jmp => "jmp",
            JmpType::Jz => "jz",
            JmpType::Jnz => "jnz",
            JmpType::Jc => "jc",
            JmpType::Jnc => "jnc",
            JmpType::Jp => "jp",
            JmpType::Jm => "jm",
            JmpType::Jpo => "jpo",
            JmpType::Jpe => "jpe",
            JmpType::Call => "call",
            JmpType::Cz => "cz",
            JmpType::Cc => "cc",
            JmpType::Cpe => "cpe",
            JmpType::Cm => "cm",
            JmpType::Cnz => "cnz",
            JmpType::Cnc => "cnc",
            JmpType::Cpo => "cpo",
            JmpType::Cp => "cp",
            JmpType::Ret => "ret",
            JmpType::Rz => "rz",
            JmpType::Rnz => "rnz",
            JmpType::Rc => "rc",
            JmpType::Rnc => "rnc",
            JmpType::Rpo => "rpo",
            JmpType::Rpe => "rpe",
            JmpType::Rp => "rp",
            JmpType::Rm => "rm",
        }
    }
}

#[derive(Debug)]
pub enum RetType {
    Ret,
    Rz,
    Rnz,
    Rc,
    Rnc,
    Rpo,
    Rpe,
    Rp,
    Rm,
}

impl RetType {
    pub fn code(&self) -> u8 {
        match self {
            RetType::Ret => 0xC9,
            RetType::Rz => 0xC8,
            RetType::Rc => 0xD8,
            RetType::Rpe => 0xE8,
            RetType::Rm => 0xF8,
            RetType::Rnz => 0xC0,
            RetType::Rnc => 0xD0,
            RetType::Rpo => 0xE0,
            RetType::Rp => 0xF0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            RetType::Ret => "ret",
            RetType::Rz => "rz",
            RetType::Rnz => "rnz",
            RetType::Rc => "rc",
            RetType::Rnc => "rnc",
            RetType::Rpo => "rpo",
            RetType::Rpe => "rpe",
            RetType::Rp => "rp",
            RetType::Rm => "rm",
        }
    }
}

#[derive(Debug)]
pub enum ArithmeticType {
    Adc,
    Add,
    Sub,
    Sbb,
    And,
    Xor,
    Or,
    Cmp,
}

impl ArithmeticType {
    pub fn reg_code(&self) -> u8 {
        match self {
            ArithmeticType::Adc => 0x88,
            ArithmeticType::Add => 0x80,
            ArithmeticType::Sub => 0x90,
            ArithmeticType::Sbb => 0x98,
            ArithmeticType::And => 0xA0,
            ArithmeticType::Xor => 0xA8,
            ArithmeticType::Or =>  0xB0,
            ArithmeticType::Cmp => 0xB8,
        }
    }

    pub fn const_code(&self) -> u8 {
        match self {
            ArithmeticType::Adc => 0xCE,
            ArithmeticType::Add => 0xC6,
            ArithmeticType::Sub => 0xD6,
            ArithmeticType::Sbb => 0xDE,
            ArithmeticType::And => 0xF6,
            ArithmeticType::Xor => 0xE6,
            ArithmeticType::Or =>  0xEE,
            ArithmeticType::Cmp => 0xFE,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ArithmeticType::Adc => "adc",
            ArithmeticType::Add => "add",
            ArithmeticType::Sub => "sub",
            ArithmeticType::Sbb => "sbb",
            ArithmeticType::And => "ana",
            ArithmeticType::Xor => "xra",
            ArithmeticType::Or => "ora",
            ArithmeticType::Cmp => "cmp",
        }
    }

    pub fn const_name(&self) -> &'static str {
        match self {
            ArithmeticType::Adc => "aci",
            ArithmeticType::Add => "adi",
            ArithmeticType::Sub => "sui",
            ArithmeticType::Sbb => "sbi",
            ArithmeticType::And => "ani",
            ArithmeticType::Xor => "xri",
            ArithmeticType::Or => "ori",
            ArithmeticType::Cmp => "cpi",
        }
    }
}

#[derive(Debug)]
pub enum StatementKind {
    Nop,
    Mov(MovArg, MovArg),
    Rlc,
    Rrc,
    Ral,
    Rar,
    Stc,
    Cmc,
    Arif(Register, ArithmeticType),
    Arifn(u16, ArithmeticType),
    Rst(u8),
    Out(u8),
    In(u8),
    Neg(Register),
    Inc(Register),
    Incp(RegisterPair),
    Dcr(Register),
    Dcrp(RegisterPair),
    Daa,
    Dad(RegisterPair),
    Hlt,
    Pchl,
    Jmp(String, JmpType),
    Ret(RetType),
    Cli,
    Sti,
    Push(RegisterPair),
    Pushpsw,
    Pop(RegisterPair),
    Poppsw,
    Db(u16),
    Dw(u16),
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