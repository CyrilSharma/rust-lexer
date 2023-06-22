use std::fs;

// [, ], {, }, (, ), -, *
#[derive(Debug, Eq)]
enum Token {
    LBR,
    RBR,
    LPR,
    RPR,
    DASH,
    STAR,
    CHAR(char),
    EOF
}

#[derive(Debug)]
enum TokenErr {
    InvalidExpr,
    InvalidEscape
}


struct Lexer { 
    chars: Vector<char>,
    pos: u32
}

impl Lexer {
    fn new(fname: String) -> Self {
        chars = fs::read_to_string(fname)
            .unwrap()
            .chars()
            .collect();
        Lexer { chars, pos: 0 }
    }

    fn nextchar() {
        pos += 1;
        return chars[pos - 1];
    }

    fn next() -> Result<Token, TokenErr> {
        loop {
            if (pos == chars.len()) { 
                return Ok(Token::EOF)
            }
            match nextchar() {
                ' ' | '\t' | '\n' | '\r' => {}
                '[' => return Ok(Token::LBR),
                ']' => return Ok(Token::RBR),
                '{' => return Ok(Token::LCR),
                '}' => return Ok(Token::RCR),
                '(' => return Ok(Token::LPR),
                ')' => return Ok(Token::RPR),
                '-' => return Ok(Token::DASH),
                '*' => return Ok(Token::STAR),
                '#' => {
                    while (pos < chars.len()) {
                        if nextchar() == '\n' { break }
                    }
                }
                other => {
                    match other {
                        ']' => break,
                        '\\' => {
                            if pos == chars.len() - 1 { return Err(TokenErr::InvalidExpr) }
                            let c = nextchar();
                            match c {
                                'n'  => return Ok(Char('\n')),
                                't'  => return Ok(Char('\t')),
                                'r'  => return Ok(Char('\r')),
                                '\\' => return Ok(Char('\\')),
                                ']'  => return Ok(Char('\r')),
                                _    => return Err(Token::InvalidEscape)
                            }
                        }
                        '\n' | '\r' => return Err(TokenErr::InvalidExpr),
                        _ => return Ok(Token(other))
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
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}