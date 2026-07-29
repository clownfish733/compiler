mod parser;
mod tokenizer;
use tokenizer::{LexError, Lexer};

//use parser::{ParseError, Parser};

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

fn display_tokens(tokens: Lexer) {
    for t in tokens {
        println!("{:?}", t);
    }
}

fn main() {
    let contents = get_contents(get_file_path());
    let tokens = Lexer::parse(&contents);
    display_tokens(tokens.clone());
    /*
    let prog = match Parser::new(tokens).parse() {
        Ok(prog) => prog,
        Err(e) => match e {
            ParseError::LexError(e) => {
                e.debug(contents);
                return;
            }
            _ => {
                println!("ParseError");
                return;
            }
        },
    };
    */
}
