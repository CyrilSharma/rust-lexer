use crate::lexer::{TokenGiver, Token, TokenErr, Group, Op};
use crate::ast::{Node, BinaryExprNode, UnaryExprNode, Match};
use Token::*;
use Group::*;
use Op::*;

#[derive(Debug)]
pub enum ParseError {
    Parse(String),
    Token(TokenErr)
}
impl From<TokenErr> for ParseError {
    fn from(err: TokenErr) -> ParseError {
        return ParseError::Token(err);
    }
}

pub struct Parser<T: TokenGiver> {
    cur: Token,
    lexer: T,
}

impl<T: TokenGiver> Parser<T> {
    pub fn new(mut lexer: T) -> Result<Self, ParseError> {
        return Ok(Parser { 
            cur: lexer.next()?, 
            lexer,
        });
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        let temp = self.cur;
        self.cur = self.lexer.next()?;
        return Ok(temp);
    }

    fn consume(&mut self, token: Token, caller: &str) -> Result<(), ParseError> {
        if token == self.cur {
            self.advance()?;
            return Ok(())
        } else {
            return Err(ParseError::Parse(
                format!("{}: Expected {:?} but got {:?}",
                    caller, token, self.cur)
            ));
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Match>, ParseError> {
        let mut matches = Vec::new();
        while let GROUP(DBQ) = self.cur {
            self.consume(GROUP(DBQ), "Parse")?;
            let root = self.expr()?;
            self.consume(GROUP(DBQ), "Parse")?;
            let name = self.name()?; 
            matches.push(Match { root, name: name });
        }
        if self.cur != EOF { 
            return Err(ParseError::Parse(
                format!("Parse: Expected EOF but got {:?}", self.cur)
            ));
        }
        return Ok(matches);
    }

    fn expr(&mut self) -> Result<Node, ParseError> {
        let mut root = self.term()?;
        while let OP(BAR) = self.cur {
            self.advance()?;
            let term = self.term()?;
            let new_root = BinaryExprNode {
                op: BAR,
                left: Box::new(root),
                right: Box::new(term)
            };
            root = Node::BinaryExpr(new_root);
        }
        return Ok(root);
    }

    fn term(&mut self) -> Result<Node, ParseError> {
        let mut root = self.factor()?;
        while let CHAR(_) | GROUP(LPR) | GROUP(LBR) = self.cur {
            let node= self.factor()?;
            let new_root = BinaryExprNode {
                op: AND,
                left: Box::new(root),
                right: Box::new(node)
            };
            root = Node::BinaryExpr(new_root);
        }
        return Ok(root);
    }

    fn factor(&mut self) -> Result<Node, ParseError> {
        let node = self.atom()?;
        if let OP(op) = self.cur {
            if let QUESTION | STAR | PLUS = op {
                let root = UnaryExprNode { 
                    op, child: Box::new(node)
                };
                self.consume(OP(op), "Factor")?;
                return Ok(Node::UnaryExpr(root));
            }
        }
        return Ok(node);
    }

    fn atom(&mut self) -> Result<Node, ParseError> {
        match self.advance()? {
            GROUP(LPR) => { 
                let node = self.expr()?;
                self.consume(GROUP(RPR), "Atom")?;
                return Ok(node);
            },
            CHAR(c) => return Ok(Node::Char(c)),
            GROUP(LBR) => return Ok(self.dashes()?),
            token => Err(ParseError::Parse(
                format!("Atom: Expected CHAR, [, (, but found {:?}", token)
            ))
        }
    }

    fn dashes(&mut self) -> Result<Node, ParseError> {
        let mut root: Option<Node> = None;
        loop { match self.cur {
            CHAR(_) => {
                let dash = self.dash()?;
                match root {
                    None => root = Some(dash),
                    Some(node) => root = Some(Node::BinaryExpr(
                        BinaryExprNode { 
                            left: Box::new(node), 
                            right: Box::new(dash), 
                            op: BAR 
                        }
                    ))
                }
            },
            _ => break
        }}
        self.consume(GROUP(RBR), "Dashes")?;
        match root {
            None => return Err(ParseError::Parse(
                format!("Dashes: No Dash Found")
            )),
            Some(node) => return Ok(node)
        }
    }

    fn dash(&mut self) -> Result<Node, ParseError> {
        let root: Node;
        let c = self.advance()?.char();
        if c.is_digit(10) {
            self.consume(OP(DASH), "Dash")?;
            let d = self.advance()?.char();
            if d.is_digit(10) { 
                root = Node::BinaryExpr(BinaryExprNode {
                    op: DASH,
                    left: Box::new(Node::Char(c)),
                    right: Box::new(Node::Char(d))
                });
            } else {
                return Err(ParseError::Parse(
                    format!("Dash: Expected Num-Num but got Num-{}", d)
                ));
            }
        } else if c.is_alphabetic() {
            self.consume(OP(DASH), "Dash")?;
            let d = self.advance()?.char();
            if d.is_alphabetic() {
                root = Node::BinaryExpr(BinaryExprNode {
                    op: DASH,
                    left: Box::new(Node::Char(c)),
                    right: Box::new(Node::Char(d))
                });
            } 
            else {
                return Err(ParseError::Parse(
                    format!("Dash: Expected Alpha-Alpha but got Alpha-{}", d)
                ));
            }
        } else {
            return Err(ParseError::Parse(
                format!("Dash: Expected Alphanumeric but got {}", c)
            ));
        }
        return Ok(root);
    }

    fn name(&mut self) -> Result<String, ParseError> {
        let mut name: String = String::new();
        while let CHAR(c) = self.cur {
            name.push(c);
            self.advance()?;
        }
        self.consume(SEMI, "Name")?;
        return Ok(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::lexer::Lexer;
    struct TokenReader { pos: u32, tokens: Vec<Token> } 
    impl TokenReader {
        pub fn new(path: String) -> Self {
            let mut tokens: Vec<Token> = Vec::new();
            let words: Vec<String> = fs::read_to_string(path)
                .expect("File Doesn't Exist")
                .replace(' ', "")
                .replace('\n', "")
                .split(',')
                .map(|s| s.to_string())
                .collect();
            for word in words {
                let token = match word.as_str() {
                    "STAR" | "OP(STAR)"         => OP(STAR),
                    "PLUS" | "OP(PLUS)"         => OP(PLUS),
                    "QUESTION" | "OP(QUESTION)" => OP(QUESTION),
                    "BAR" | "OP(BAR)"           => OP(BAR),
                    "DASH" | "OP(DASH)"         => OP(DASH),
                    "AND" | "OP(AND)"           => OP(AND),
                    "DBQ" | "GROUP(DBQ)"        => GROUP(DBQ),
                    "LBR" | "GROUP(LBR)"        => GROUP(LBR),
                    "RBR" | "GROUP(RBR)"        => GROUP(RBR),
                    "LCR" | "GROUP(LCR)"        => GROUP(LCR),
                    "RCR" | "GROUP(RCR)"        => GROUP(RCR),
                    "LPR" | "GROUP(LPR)"        => GROUP(LPR),
                    "RPR" | "GROUP(RPR)"        => GROUP(RPR),
                    ";" | "SEMI"                => SEMI,
                    "EOF"                       => EOF,
                    "CHAR('')"                  => CHAR(' '),   // edge case
                    s => {
                        let res: Token;
                        let parsed_chars: Vec<char> = s.chars().collect();
                        if s.len() == 1      { res = CHAR(parsed_chars[0]); }
                        else if s.len() >= 9 {
                            if parsed_chars[6] == '\\' {
                                match parsed_chars[7] {
                                    'n'  => res = CHAR('\n'),
                                    't'  => res = CHAR('\t'),
                                    'r'  => res = CHAR('\r'),
                                    '\\' | ']' | '[' | ')' | '(' |
                                    '-' | '*' | ';' | '+' | '"' | '\'' => {
                                        res = CHAR(parsed_chars[7]);
                                    }
                                    _ => panic!("Unrecognized Token")
                                }
                            } else {
                                res = CHAR(parsed_chars[6]);
                            }
                        }
                        else {
                            panic!("Unrecognized Token: {}", s);
                        }
                        res
                    }
                };
                tokens.push(token);
            }
            return TokenReader { pos: 0, tokens };
        }
    }

    impl TokenGiver for TokenReader {
        fn next(&mut self) -> Result<Token, TokenErr> {
            self.pos += 1; 
            //println!("Token: {:?}", self.tokens[(self.pos - 1) as usize]);
            return Ok(self.tokens[(self.pos - 1) as usize]);
        }
    }

    #[test]
    fn test_parser() {
        let path = "src/test_data/parser".to_string();
        if let Ok(entries) = fs::read_dir(format!("{path}/input")) {
            for entry in entries {
                if entry.is_err() { panic!("Invalid Directory"); }
                let os_str = entry.unwrap().file_name();
                let file_name = os_str.to_str().unwrap();
                let ident = file_name
                    .trim()
                    .replace('\n', "")
                    .split("-")
                    .map(|s| s.to_string())
                    .nth(0)
                    .expect("Filename should have non-zero length");
                match ident.as_str() {
                    "rightnp" | "wrongnp" => assert!(right_wrongnp(
                        format!("{path}/input/{file_name}"),
                        ident
                    )),
                    "AST" => assert!(ast(
                        format!("{path}/input/{file_name}"),
                        format!("{path}/output/{file_name}")
                    )),
                    _ => ()
                }
            }
        }
    }

    fn right_wrongnp(path: String, ans: String) -> bool {
        let tr = TokenReader::new(path);
        let mut parser = Parser::new(tr).expect("Invalid Token Stream");
        let matches = parser.parse();
        match matches {
            Ok(_) => return "rightnp" == ans,
            Err(e) => {
                println!("Error: {:?}", e);
                return "wrongnp" == ans
            }
        }
    }

    fn ast(inpath: String, outpath: String) -> bool {
        let tr = Lexer::new(&inpath).expect("File Doesn't Exist");
        let mut parser = Parser::new(tr).expect("Invalid Token Stream");
        let matches = parser.parse().expect("Expression should be valid.");
        for m in matches { 
            let ans: String = fs::read_to_string(&outpath).expect("File doesn't exist.");
            if ans.trim() != m.root.to_string().trim() { 
                println!("{}", ans.trim());
                println!("________________");
                println!("{}", m.root.to_string().trim());
                return false; 
            }
            //m.print(); 
        }
        return true;
    }
}