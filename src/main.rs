mod parser;
mod tokenizer;

mod codegen;

use codegen::gen_code;
use parser::Parser;
use tokenizer::Lexer;

use std::io::Write;

const ASM_PATH: &str = "main.s";

fn get_file_path() -> String {
    let mut args = std::env::args();
    if let Some(_) = args.next()
        && let Some(filepath) = args.next()
    {
        filepath
    } else {
        panic!("cargo run filepath");
    }
}

fn get_contents(path: String) -> String {
    if let Ok(contents) = std::fs::read_to_string(&path) {
        contents
    } else {
        panic!("file: {} not found", path);
    }
}

fn write_asm(contents: String) -> Option<()> {
    let mut file = std::fs::File::create(ASM_PATH).ok()?;
    file.write_all(contents.as_bytes())
        .ok()
}

fn run_asm() {
    let out = std::process::Command::new("make")
        .arg("run")
        .output()
        .expect("Failed to run");
    let res = String::from_utf8_lossy(&out.stdout);
    let lines: Vec<&str> = res
        .lines()
        .collect();
    for line in lines
        .get(2..lines.len() - 1)
        .unwrap()
    {
        println!("{}", line);
    }

    let _ = std::process::Command::new("make")
        .arg("clean")
        .output()
        .expect("Failed to clean");
}

fn display_tokens(tokens: Lexer) {
    for t in tokens {
        println!("{:?}", t);
    }
}

fn main() {
    let contents = get_contents(get_file_path());
    let tokens = Lexer::parse(&contents);
    display_tokens(tokens.clone());
    let prog = match Parser::parse(tokens) {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("{:?}", e);
            e.highlight_error(&contents);
            return;
        }
    };
    println!("{:#?}", &prog);
    let code = match gen_code(prog) {
        Ok(code) => code,
        Err(_) => {
            eprintln!("Codegen Error");
            return;
        }
    };
    println!("{}", &code);

    if write_asm(code.clone()).is_none() {
        eprintln!("failed to write to file");
    }

    run_asm();

    //println!("{}", code);
}
