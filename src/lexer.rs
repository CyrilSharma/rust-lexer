use std::fs;
use Token::*;
use Group::*;
use Op::*;

pub trait TokenGiver { fn next(&mut self) -> Token; }

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
    pos: u32,
    enclosed: bool
}

impl Lexer {
    pub fn new(fname: &str) -> Self {
        let chars = fs::read_to_string(fname)
            .unwrap()
            .chars()
            .collect();
        Lexer { chars, pos: 0, enclosed: false }
    }

    fn nextchar(&mut self) -> char {
        self.pos += 1;
        return self.chars[(self.pos - 1) as usize];
    }
}

impl TokenGiver for Lexer {
    fn next(&mut self) -> Token {
        loop {
            if self.pos == self.chars.len() as u32 { 
                return EOF
            }
            match self.nextchar() {
                '\t' | '\n' | '\r' => {}
                '"' => { 
                    self.enclosed = !self.enclosed; 
                    return GROUP(DBQ); 
                },
                '[' => return GROUP(LBR),
                ']' => return GROUP(RBR),
                '{' => return GROUP(LCR),
                '}' => return GROUP(RCR),
                '(' => return GROUP(LPR),
                ')' => return GROUP(RPR),
                '-' => return OP(DASH),
                '*' => return OP(STAR),
                '+' => return OP(PLUS),
                ';' => return SEMI,
                '#' => while self.pos < self.chars.len() as u32 {
                    if self.nextchar() == '\n' { break }
                },
                other => match other {
                    '\\' => {
                        if self.pos == (self.chars.len() - 1) as u32 
                            { panic!("{:?}", TokenErr::InvalidExpr); }
                        let c = self.nextchar();
                        match c {
                            'n'  => return CHAR('\n'),
                            't'  => return CHAR('\t'),
                            'r'  => return CHAR('\r'),
                            '\\' | ']' | '[' | ')' | '(' |
                            '-' | '*' | ';' | '+' | '"' => {
                                if self.enclosed { return CHAR(c); }
                                else { panic!("{:?}", TokenErr::InvalidExpr) }
                            },
                            _    => panic!("{:?}", TokenErr::InvalidEscape),
                        }
                    }
                    ' ' => if self.enclosed { return CHAR(' ') },
                    '\n' | '\r' => panic!("{:?}", TokenErr::InvalidExpr),
                    _ => return CHAR(other)
                }
            }
        }
    }
}

// Figure out how to test this...
#[cfg(test)]
mod tests {
    #[test]
    // Add -- --nocapture to see output.
    fn inspection() {
        use super::*;
        let mut lx = Lexer::new("src/example.tk");
        loop {
            let tk = lx.next();
            println!("{:?}", tk);
            if tk == EOF { break }
        }
        assert_eq!(4, 4);
    }
}