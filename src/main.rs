use rflex_lib::{lexer::Lexer, parser::Parser, nfa::NFA, dfa::DFA, generator::Generator};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut inpath = "example.tk".to_string();
    if let Some(s) = args.get(1) {
        inpath = s.clone();
    }
    let mut outpath = "tokenizer.rs".to_string();
    if let Some(s) = args.get(2) {
        outpath = s.clone();
    }
    let lexer = Lexer::new(&inpath).expect("Invalid Path");
    let mut parser = Parser::new(lexer).expect("File should be non-empty!");
    let nfa = NFA::build_from_matches(&parser.parse().expect("Invalid parse"));
    let dfa = DFA::compress(DFA::subset_construction(nfa));
    let mut gen = Generator::new(&dfa, outpath).expect("Outpath doesn't exist");
    gen.generate().expect("Write Error");
}