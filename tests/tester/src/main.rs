use std::env;
use std::fs;

mod tokenizer;
use tokenizer::{Lexer, Token};
fn main() {
    let args: Vec<String> = env::args().collect();
    let inpath = args.get(1).unwrap();
    let outpath = args.get(2).unwrap();
    let mut lx = Lexer::new(inpath).expect("File Exists");
    let mut tokens: String = String::new();
    loop { match lx.next() {
        Ok(tk) => {
            if tk == Token::EOF { 
                tokens.push_str(&format!("{:?}", tk));
                break;
            } else {
                tokens.push_str(&format!("{:?}, ", tk));
            }
        },
        Err(tk) => {
            eprintln!("{:?}", tk);
            std::process::exit(1)
        }
    }}
    if tokens == fs::read_to_string(outpath).expect("File exists") {
        std::process::exit(0);
    } else {
        eprintln!("{}", tokens);
        std::process::exit(1);
    }
}
