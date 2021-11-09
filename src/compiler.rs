use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::ast::{File, Statement, StatementKind, Register, RegisterPair, MovArg};

pub struct CompilerResult {
    pub data: Vec<u8>,
    pub pretty_instructions: Vec<String>,
    pub table: Vec<String>,
}

impl Debug for CompilerResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompilerResult ( {:?} ): [\n", &self.pretty_instructions)?;
        for x in &self.table {
            writeln!(f, "{}", x)?;
        }
        writeln!(f, "]\n")
    }
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(&self, tokens: &File) -> CompilerResult {
        let mut labels = HashMap::<String, u16>::new();
        let mut pretty_out = Vec::<String>::new();
        let mut data = Vec::<u8>::new();
        let mut table = Vec::<String>::new();
        let mut code_ptr = 0x8200u16;

        for (idx, statement) in tokens.statements.iter().enumerate() {
            if let StatementKind::Lset(addr) = &statement.kind {
                if let Some(label) = &statement.label {
                    labels.insert(label.to_owned(), *addr);
                    labels.insert(format!(".{}", label), *addr);
                } else {
                    println!("WARNING! No label at lset at pos {}", idx + 1);
                }
                continue;
            }
            if let Some(label) = &statement.label {
                labels.insert(label.to_owned(), code_ptr);
                labels.insert(format!(".{}", label), code_ptr);
            }
            if let Some((code, pretty)) = Self::compile_statement(&statement, &labels) {
                pretty_out.push(pretty.to_uppercase());
                let mut code_str = String::new();
                for b in &code {
                    data.push(*b);
                    code_str += &format!("{:02X} ", b);
                }
                table.push(format!("{:04X};{};{};{}", code_ptr, code_str, &statement.label.clone().unwrap_or("".to_owned()), pretty));
                code_ptr += code.len() as u16;
            }
        }
        CompilerResult { pretty_instructions: pretty_out, data, table }
    }

    fn format_label(label: &str) -> &str {
        if label.starts_with(".") {
            &label[1..]
        } else {
            label
        }
    }

    fn prepend_to_addr(val: u8, addr: u16) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.push(val);
        vec.extend_from_slice(&addr.to_le_bytes());
        vec
    }

    fn check_8bit_const(c: &u16) -> Option<()> {
        if *c > 255u16 {
            eprintln!("Cannot add two-byte constant");
            None
        } else {
            Some(())
        }
    }

    fn compile_statement(statement: &Statement, label_map: &HashMap<String, u16>) -> Option<(Vec<u8>, String)> {
        match &statement.kind {
            StatementKind::Lset(_) => None,
            StatementKind::Cmc => Some((vec![0x3f], "cmc".to_owned())),
            StatementKind::Nop => Some((vec![0x00], "nop".to_owned())),
            StatementKind::Ral => Some((vec![0x17], "ral".to_owned())),
            StatementKind::Rar => Some((vec![0x1F], "rar".to_owned())),
            StatementKind::Rlc => Some((vec![0x07], "rlc".to_owned())),
            StatementKind::Rrc => Some((vec![0x0F], "rrc".to_owned())),
            StatementKind::Stc => Some((vec![0x37], "stc".to_owned())),
            StatementKind::Daa => Some((vec![0x27], "daa".to_owned())),
            StatementKind::Hlt => Some((vec![0x76], "hlt".to_owned())),
            StatementKind::Cli => Some((vec![0xF3], "di".to_owned())),
            StatementKind::Sti => Some((vec![0xFB], "ei".to_owned())),
            StatementKind::Pchl => Some((vec![0xE9], "pchl".to_owned())),
            StatementKind::In(port) => Some((vec![0xDB, *port], format!("in {}", port))),
            StatementKind::Out(port) => Some((vec![0xD3, *port], format!("out {}", port))),
            StatementKind::Db(dat) => Self::check_8bit_const(dat).map(|_| (vec![*dat as u8], format!("{:02X}", dat))),
            StatementKind::Dw(dat) => Some((dat.to_be_bytes().to_vec(), format!("{:04X}", dat))),
            StatementKind::Jmp(label, typ) => {
                let addr = *label_map.get(label).expect(&format!("Label {} not found", label));
                Some((Self::prepend_to_addr(typ.code(), addr), format!("{} {}", typ.name(), Self::format_label(label))))
            }
            StatementKind::Ret(typ) => Some((vec![typ.code()], format!("{}", typ.name()))),
            StatementKind::Rst(code) => Self::compile_rst(*code),
            StatementKind::Push(pair) => {
                if pair == &RegisterPair::SP {
                    eprintln!("Cannot push SP onto stack");
                    None
                } else {
                    Some((vec![pair.push_code()], format!("push {}", pair.name())))
                }
            },
            StatementKind::Pop(pair) => {
                if pair == &RegisterPair::SP {
                    eprintln!("Cannot push SP onto stack");
                    None
                } else {
                    Some((vec![pair.pop_code()], format!("pop {}", pair.name())))
                }
            }
            StatementKind::Pushpsw => Some((vec![0xF5], format!("push psw"))),
            StatementKind::Poppsw => Some((vec![0xF1], format!("pop psw"))),
            StatementKind::Arif(reg, typ) => Some((vec![typ.reg_code() + reg.code_off()], format!("{} {}", typ.name(), reg.name()))),
            StatementKind::Arifn(c, typ) => Self::check_8bit_const(c).map(|_| (vec![typ.const_code(), *c as u8], format!("{} {:02X}", typ.const_name(), c))),
            StatementKind::Dad(pair) => Some((vec![0x09 + pair.left_table_x_off()], format!("dat {}", pair.name()))),
            StatementKind::Incp(pair) => Some((vec![pair.left_table_x_off() + 0x03], format!("inx {}", pair.name()))),
            StatementKind::Dcrp(pair) => Some((vec![pair.left_table_x_off() + 0x0B], format!("dcx {}", pair.name()))),
            StatementKind::Inc(reg) => {
                let cmd = if reg.left_table_is_down() { 0x0C } else { 0x04 } + reg.left_table_x_off();
                Some((vec![cmd], format!("inr {}", reg.name())))
            },
            StatementKind::Dcr(reg) => {
                let cmd = if reg.left_table_is_down() { 0x0D } else { 0x05 } + reg.left_table_x_off();
                Some((vec![cmd], format!("dcr {}", reg.name())))
            },
            StatementKind::Neg(reg) => {
                match reg {
                    Register::A => Some((vec![0x2F], format!("cma"))),
                    Register::C => Some((vec![0x3F], format!("cmc"))),
                    _ => {
                        eprintln!("Neg is not supported for register {}", reg.name());
                        None
                    }
                }
            },
            StatementKind::Mov(a, b) => {
                match a {
                    MovArg::Constant(_) => {
                        eprintln!("Cannot move into constant");
                        None
                    },
                    MovArg::Register(a_reg) => Self::compile_mov_register(a_reg, b),
                    MovArg::RegisterPair(pair) => Self::compile_mov_regpair(pair, b),
                    MovArg::MemoryIndirect(pair) => Self::compile_mov_mem_indirect(b, pair),
                    MovArg::MemoryDirect(mem) => Self::compile_mov_mem_direct(mem, b),
                }
            }
        }
    }

    fn compile_rst(code: u8) -> Option<(Vec<u8>, String)> {
        match code {
            0 => Some((vec![0xC7], "rst 0".to_owned())),
            16 => Some((vec![0xD7], "rst 16".to_owned())),
            32 => Some((vec![0xE7], "rst 32".to_owned())),
            48 => Some((vec![0xF7], "rst 48".to_owned())),
            8 => Some((vec![0xCF], "rst 8".to_owned())),
            24 => Some((vec![0xDF], "rst 24".to_owned())),
            40 => Some((vec![0xEF], "rst 40".to_owned())),
            56 => Some((vec![0xFF], "rst 56".to_owned())),
            _ => {
                eprintln!("RST code {} not supported", code);
                None
            }
        }
    }

    fn compile_mov_mem_direct(mem: &u16, b: &MovArg) -> Option<(Vec<u8>, String)> {
        if let MovArg::Register(reg) = b {
            if reg != &Register::A {
                eprintln!("Cannot store register {} into memory", reg.name());
                None
            } else {
                Some((Self::prepend_to_addr(0x32, *mem), format!("sta {:04X}", mem)))
            }
        } else if let MovArg::RegisterPair(pair) = b {
            if pair != &RegisterPair::HL {
                eprintln!("Storing value from pair {} is not supported", pair.name());
                None
            } else {
                Some((Self::prepend_to_addr(0x22, *mem), format!("shld {:04X}", mem)))
            }
        } else {
            eprintln!("Cannot load something in direct memory other than registers");
            None
        }
    }

    fn compile_mov_mem_indirect(b: &MovArg, pair: &RegisterPair) -> Option<(Vec<u8>, String)> {
        if let MovArg::Register(b_reg) = b {
            if b_reg != &Register::A {
                eprintln!("Cannot store register {} into indirect memory", b_reg.name());
                None
            } else {
                match pair {
                    RegisterPair::BC => Some((vec![0x02], "stax b".to_owned())),
                    RegisterPair::DE => Some((vec![0x12], "stax d".to_owned())),
                    RegisterPair::HL | RegisterPair::SP => {
                        eprintln!("Cannot load register {} from indirect memory at {}", b_reg.name(), pair.name());
                        None
                    }
                }
            }
        } else if let MovArg::RegisterPair(b_pair) = b {
            if pair == &RegisterPair::SP && b_pair == &RegisterPair::HL {
                Some((vec![0xE3], format!("xthl")))
            } else {
                eprintln!("Indirect memory access is supported only for HL from SP");
                None
            }
        } else {
            eprintln!("Indirect moving into memory is supported only for registers");
            None
        }
    }

    fn compile_mov_regpair(pair: &RegisterPair, b: &MovArg) -> Option<(Vec<u8>, String)> {
        match b {
            MovArg::Constant(c) => {
                Some((Self::prepend_to_addr(0x01 + pair.left_table_x_off(), *c), format!("lxi {}, {:04X}", pair.name(), c)))
            },
            MovArg::Register(_) => {
                eprintln!("Loading register into register pair not supported");
                None
            },
            MovArg::RegisterPair(p) => {
                if pair == &RegisterPair::SP && p == &RegisterPair::HL {
                    Some((vec![0xF9], format!("sphl")))
                } else if (pair == &RegisterPair::DE && p == &RegisterPair::HL) || (pair == &RegisterPair::HL && p == &RegisterPair::DE) {
                    Some((vec![0xEB], format!("xchg")))
                } else {
                    eprintln!("Moving values between common register pairs is not supported");
                    None
                }
            },
            MovArg::MemoryDirect(addr) => {
                if pair != &RegisterPair::HL {
                    eprintln!("Loading value in pair {} is not supported", pair.name());
                    None
                } else {
                    Some((Self::prepend_to_addr(0x2A, *addr), format!("lhld {:04X}", addr)))
                }
            },
            MovArg::MemoryIndirect(p) => {
                if pair == &RegisterPair::HL && p == &RegisterPair::SP {
                    Some((vec![0xE3], format!("xthl")))
                } else {
                    eprintln!("Indirect memory access is supported only for HL from SP");
                    None
                }
            }
        }
    }

    fn compile_mov_register(a_reg: &Register, b: &MovArg) -> Option<(Vec<u8>, String)> {
        match b {
            MovArg::Register(b_reg) => {
                if *a_reg == Register::M && *b_reg == Register::M {
                    eprintln!("mov m, m not supported");
                    None
                } else {
                    Some((vec![a_reg.mov_base_off() + b_reg.code_off()], format!("mov {}, {}", a_reg.name(), b_reg.name())))
                }
            }
            MovArg::Constant(c) => {
                if *c >= 256u16 {
                    eprintln!("Cannot put 16-bit constant into 8-bit register");
                    None
                } else {
                    let cmd = if a_reg.left_table_is_down() { 0x0E } else { 0x06 } + a_reg.left_table_x_off();
                    Some((vec![cmd, *c as u8], format!("mvi {}, {:02X}", a_reg.name(), c)))
                }
            }
            MovArg::RegisterPair(_) => {
                eprintln!("Cannot move register pair into register");
                None
            }
            MovArg::MemoryDirect(mem) => {
                if *a_reg != Register::A {
                    eprintln!("Cannot load register {} from memory", a_reg.name());
                    None
                } else {
                    Some((Self::prepend_to_addr(0x3A, *mem), format!("lda {:04X}", mem)))
                }
            }
            MovArg::MemoryIndirect(mem) => {
                if *a_reg != Register::A {
                    eprintln!("Cannot load register {} from indirect memory", a_reg.name());
                    None
                } else {
                    match mem {
                        RegisterPair::BC => Some((vec![0x0A], "ldax b".to_owned())),
                        RegisterPair::DE => Some((vec![0x1A], "ldax d".to_owned())),
                        RegisterPair::HL | RegisterPair::SP => {
                            eprintln!("Cannot load register {} from indirect memory at {}", a_reg.name(), mem.name());
                            None
                        }
                    }
                }
            }
        }
    }
}