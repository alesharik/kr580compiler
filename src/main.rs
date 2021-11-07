#[macro_use] extern crate lalrpop_util;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use clap::Parser;
use crate::compiler::Compiler;

mod ast;
mod compiler;
lalrpop_mod!(pub grammar);

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Aleksei A. <alesharik4@gmail.com>")]
struct Opts {
    input: String,
    #[clap(short, long)]
    table: bool,
}

fn build_offset(off: usize) -> Vec<u8> {
    let mut v = Vec::<u8>::with_capacity(off);
    for _ in 0..off {
        v.push(0);
    }
    v
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    let mut content = String::new();
    let filename = Path::new(&opts.input).file_stem().unwrap().to_os_string().to_str().unwrap().to_owned();
    std::fs::File::open(opts.input)?.read_to_string(&mut content)?;
    let tokens = grammar::FileParser::new().parse(&content).unwrap();
    let result = Compiler::new().compile(&tokens);
    if opts.table {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(format!("{}_table.csv", filename))?;
        file.write_all("ADDRES;CODE;ASM\n".as_bytes())?;
        for x in &result.table {
            file.write_all(format!("{}\n", x).as_bytes())?;
        }
        file.flush()?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("{}.bin", filename))?;
    file.write_all(&build_offset(0x8200))?;
    file.write_all(&result.data)?;
    Ok(())
}
