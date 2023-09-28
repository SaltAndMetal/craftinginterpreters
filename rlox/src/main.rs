#![allow(unused_must_use, non_snake_case, non_camel_case_types)]
use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};

mod token;

mod scan;
use crate::scan::scan;

mod loxStaticError;
use loxStaticError::LoxStaticError;

mod expr;

mod parse;
use parse::Parser;

mod interpret;
use interpret::execute;
use interpret::env::Env;

fn run(source: String, env: Env) -> Result<Env, ()>
{
    fn detectStaticErrors(errors: &Vec<LoxStaticError>) -> Result<(), ()> {
        if !errors.is_empty() {
            for error in errors {
                println!("{}", error);
            }
            return Err(());
        }
        Ok(())
    }

    let (tokens, errors) = scan(source);
    detectStaticErrors(&errors)?;

    //for token in &tokens {
    //    println!("{:?}", token);
    //}

    let parser = Parser::new(tokens);
    let (program, errors) = parser.parse();
    for decl in &program {
        println!("{}", decl);
    }
    detectStaticErrors(&errors)?;

    let result = execute(program, env, &mut None);
    match result {
        Ok(e) => Ok(e),
        Err(e) => { println!("{}", e); Err(())},
    }
}



fn run_file(filename: impl AsRef<Path>)
{
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Error reading file to string");
            run(content, Env::global());
        }
        Err(e) => {
            println!("An error occured opening file: {}", e);
            std::process::exit(66);
        }
    }
}

fn run_prompt()
{
    let mut env = Env::global();
    loop {
        let mut input = String::new();
        print!(">");
        std::io::stdout().flush().expect("Output flush error");
        std::io::stdin().read_line(&mut input).expect("Did not enter a valid string");

        //removes new line
        input.pop();
        if input.is_empty() { break; }
        let archive = env.clone();
        match run(input, env) {
            Ok(e) => env = e,
            Err(_) => env = archive,
        }
    }
}

fn main()
{
    let mut args = std::env::args();
    match args.len() {
        x if x > 2 => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
        2 => {
            run_file(args.nth(1).unwrap());
        }
        _ => {
            run_prompt();
        }
    }
}
