use std::env;
use std::fs;
use tokenizer::{Lexer, Token};
fn main() {
    let args: Vec<String> = env::args().collect();
    let inpath = args.get(1).unwrap();
    let outpath = args.get(2).unwrap();
    let file = File::new(outpath);
    let lx = Lexer::new(inpath);
    let mut tokens: String = String::new();
    loop { match lx.next() {
        Ok(tk) => {
            tokens.push_str(format!("{:?}, ", tk));
            if tk == EOF { 
                tokens.push_str(format!("{:?}", tk));
                break;
            }
        },
        Err(tk) => panic!("{:?}", tk)
    }}
    assert!(tokens == fs::read_to_string(file));
}
