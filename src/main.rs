extern crate guava_parser;
extern crate guava_ir;

use std::{env, fmt, fs, io};
use std::io::{stdout, Write};
use std::path::PathBuf;
use clap::Parser;
use guava_parser::ast;
use guava_ir::ir;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Main guava file to execute
    #[arg(short, long, value_name = "FILE")]
    exec: Option<PathBuf>
}

fn input(message: &str) -> String {
    print!("{}", message);
    let mut input_line = String::new();
    let _= stdout().flush();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    input_line
}

fn main() {
    let args = Args::parse();

    let mut s: Option<String> = None;

    if args.exec.is_some() {
        match fs::read_to_string(args.exec.unwrap()) {
            Ok(code) => s = Some(code),
            Err(e) => panic!("{}", e)
        }

        let code: ast::Code = guava_parser::parse(s.unwrap().as_str());
        let ir: ir::Code = guava_ir::convert_ast(code);
    }
    else {
        loop {
            let s = input(">> ");
            let code: ast::Code = guava_parser::parse(s.as_str());
            let ir: ir::Code = guava_ir::convert_ast(code);
        }
    }
}
