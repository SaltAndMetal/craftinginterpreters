#![allow(non_snake_case)]

const DEBUG_TRACE_EXECUTION: bool = true;

mod loxErr;
use loxErr::*;

mod chunk;

mod value;

mod compiler;
use compiler::Compiler;

mod vm;
use vm::VM;

use std::env;

fn run_repl() {
    loop {
        println!("> ");
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Error reading input");

        //Removes newline
        line.pop();
        interpret(line);
    }
}

fn run_file(path: String) {
    let source =
        std::fs::read_to_string(path).expect(format!("Error reading file {}", path).as_str());
    let result = interpret(source);
    match result {
        Err(LoxErr::Runtime(s)) => {
            println!("{}", s);
            std::process::exit(70);
        }
        Err(LoxErr::Compile(s)) => {
            println!("{}", s);
            std::process::exit(65);
        }
        _ => (),
    }
}

fn interpret(source: String) -> Result<LoxOk, LoxErr> {
    let code = compile(source)?;

    let vm = VM::new(code);
    vm.execute()?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_repl(),
        2 => run_file(args[1]),
        _ => {
            println!("Usage: clox [path]");
            std::process::exit(64);
        }
    }
}
