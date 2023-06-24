use std::fs;
use Token::*;
use Group::*;
use Op::*;

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

    pub fn next(&mut self) -> Result<Token, TokenErr> {
        loop {
            if self.pos == self.chars.len() as u32 { 
                return Ok(EOF)
            }
            match self.nextchar() {
                '\t' | '\n' | '\r' => {}
                '"' => { 
                    self.enclosed = !self.enclosed; 
                    return Ok(GROUP(DBQ)); 
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
                ';' => return Ok(SEMI),
                '#' => {
                    while self.pos < self.chars.len() as u32 {
                        if self.nextchar() == '\n' { break }
                    }
                }
                other => {
                    match other {
                        '\\' => {
                            if self.pos == (self.chars.len() - 1) as u32 
                                { return Err(TokenErr::InvalidExpr) }
                            let c = self.nextchar();
                            match c {
                                'n'  => return Ok(CHAR('\n')),
                                't'  => return Ok(CHAR('\t')),
                                'r'  => return Ok(CHAR('\r')),
                                '\\' | ']' | '[' | ')' | '(' |
                                '-' | '*' | ';' | '+' | '"' => {
                                    if self.enclosed { return Ok(CHAR(c)) }
                                    else { return Err(TokenErr::InvalidExpr); }
                                },
                                _    => return Err(TokenErr::InvalidEscape)
                            }
                        }
                        ' ' => if self.enclosed { return Ok(CHAR(' ')) },
                        '\n' | '\r' => return Err(TokenErr::InvalidExpr),
                        _ => return Ok(CHAR(other))
                    }
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
            let tk = lx.next().unwrap();
            println!("{:?}", tk);
            if tk == EOF { break }
        }
        assert_eq!(4, 4);
    }
}