use std::fs;
use Token::*;
use Group::*;
use Op::*;

pub trait TokenGiver { 
    fn next(&mut self) -> Result<Token, TokenErr>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    STAR,
    PLUS,
    QUESTION,
    BAR,
    DASH,
    AND
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Group {
    DBQ,
    LBR,
    RBR,
    LCR,
    RCR,
    LPR,
    RPR
}

// ", [, ], {, }, (, ), -, *, ;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
    OP(Op),
    GROUP(Group),
    CHAR(char),
    SEMI,
    EOF
}

impl Token {
    pub fn char(&self) -> char {
        if let CHAR(c) = *self {
            return c;
        }
        panic!("Not A Letter!");
    }
}

#[derive(Debug)]
pub enum TokenErr {
    InvalidExpr,
    InvalidEscape
}


pub struct Lexer { 
    chars: Vec<char>,
    pos: usize,
    enclosed: bool
}

impl Lexer {
    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = fs::read_to_string(fname)?
            .chars()
            .collect();
        return Ok(Lexer { chars, pos: 0, enclosed: false });
    }

    fn nextchar(&mut self) -> char {
        self.pos += 1;
        return self.chars[self.pos - 1];
    }
}

impl TokenGiver for Lexer {
    fn next(&mut self) -> Result<Token, TokenErr> {
        loop {
            if self.pos == self.chars.len() { 
                return Ok(EOF)
            }
            match self.nextchar() {
                '\t' | '\n' | '\r' => {}
                '"' => { 
                    self.enclosed = !self.enclosed; 
                    return Ok(GROUP(DBQ)) 
                },
                '[' => return Ok(GROUP(LBR)),
                ']' => return Ok(GROUP(RBR)),
                '{' => return Ok(GROUP(LCR)),
                '}' => return Ok(GROUP(RCR)),
                '(' => return Ok(GROUP(LPR)),
                ')' => return Ok(GROUP(RPR)),
                '-' => return Ok(OP(DASH)),
                '*' => return Ok(OP(STAR)),
                '+' => return Ok(OP(PLUS)),
                '|' => return Ok(OP(BAR)),
                ';' => return Ok(SEMI),
                '#' => while self.pos < self.chars.len() {
                    if self.nextchar() == '\n' { break }
                },
                other => match other {
                    '\\' => {
                        if self.pos == (self.chars.len() - 1)
                            { return Err(TokenErr::InvalidExpr); }
                        let c = self.nextchar();
                        match c {
                            'n'  => return Ok(CHAR('\n')),
                            't'  => return Ok(CHAR('\t')),
                            'r'  => return Ok(CHAR('\r')),
                            '\\' | ']' | '[' | ')' | '(' |
                            '-' | '*' | ';' | '+' | '"' | '\'' => {
                                if self.enclosed { return Ok(CHAR(c)); }
                                else { return Err(TokenErr::InvalidExpr); }
                            },
                            _    => return Err(TokenErr::InvalidEscape),
                        }
                    }
                    ' ' => if self.enclosed { return Ok(CHAR(' ')); },
                    '\n' | '\r' => return Err(TokenErr::InvalidExpr),
                    _ => return Ok(CHAR(other))
                }
            }
        }
    }
}

// Figure out how to test this...
#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::Path};

    #[test]
    fn right_wrong() {
        let path = "tests/data/lexer/input";
        for id in ["right", "wrong"] {
            let mut i = 0;
            while Path::new(&format!("{path}/{id}-{i}.txt")).exists() {
                let mut lx = Lexer::new(&format!("{path}/{id}-{i}.txt")).unwrap();
                loop { match lx.next() {
                    Ok(tk) => {
                        if tk != EOF { continue; }
                        assert!("right" == id);
                        break;
                    },
                    Err(tk) => {
                        println!("{:?}", tk);
                        assert!("wrong" == id);
                    }
                }}
                i += 1;
            }
        }
    }

    // Add -- --nocapture to see output.
    #[allow(dead_code)]
    fn inspection() {
        use super::*;
        let mut lx = Lexer::new("src/example.tk").expect("File not found.");
        loop { match lx.next() {
            Ok(tk) => {
                println!("{:?}", tk);
                if tk == EOF { break }
            },
            Err(tk) => println!("{:?}", tk)
        }}
        assert!(true);
    }
}