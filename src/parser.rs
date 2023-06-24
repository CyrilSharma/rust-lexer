use crate::lexer::{TokenGiver, Token, TokenErr, Group, Op};
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

pub enum Node {
    Char(char),
    BinaryExpr(BinaryExprNode),
    UnaryExpr(UnaryExprNode),
}

pub struct BinaryExprNode {
    left:  Box<Node>,
    right: Box<Node>,
    op: Op,
}

pub struct UnaryExprNode {
    child: Box<Node>,
    op: Op,
}

pub struct AST {
    root: Node,
    name: Option<String>
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

    fn advance(&mut self) -> Result<(), ParseError> { 
        self.cur = self.lexer.next()?;
        return Ok(());
    }

    fn consume(&mut self, token: Token, caller: &str) -> Result<(), ParseError> {
        if token == self.cur {
            return Ok(self.advance()?)
        } else {
            return Err(ParseError::Parse(
                format!("{}: Expected {:?} but got {:?}",
                    caller, token, self.cur)
            ));
        }
    }

    pub fn parse(&mut self) -> Result<Vec<AST>, ParseError> {
        self.advance()?;
        let mut matches = Vec::new();
        while let GROUP(DBQ) = self.cur {
            self.consume(GROUP(DBQ), "Parse")?;
            let root = self.expr()?;
            self.consume(GROUP(DBQ), "Parse")?;
            if let SEMI = self.cur {
                self.consume(SEMI, "Parse")?;
                matches.push(AST { root, name: None });
            } else { 
                let name = self.name()?; 
                matches.push(AST { root, name: Some(name) });
            }
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
        while let CHAR(_) | GROUP(LPR) = self.cur {
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
                return Ok(Node::UnaryExpr(root));
            }
            return Ok(node);
        }
        return Err(ParseError::Parse(
            format!("Factor: Expected Operator but got {:?}", self.cur)
        ));
    }

    fn atom(&mut self) -> Result<Node, ParseError> {
        match self.cur {
            GROUP(LPR) => { 
                self.consume(GROUP(LPR), "Atom")?;
                let node = self.expr();
                self.consume(GROUP(RPR), "Atom")?;
                return node;
            },
            CHAR(c) => return Ok(Node::Char(c)),
            GROUP(LBR) => return Ok(self.dash()?),
            token => Err(ParseError::Parse(
                format!("Atom: Expected CHAR, [, (, but found {:?}", token)
            ))
        }
    }

    fn dash(&mut self) -> Result<Node, ParseError> {
        let root: Node;
        self.consume(GROUP(LBR), "Dash")?;
        let c = self.cur.char();
        if c.is_digit(10) {
            self.advance()?;
            self.consume(OP(DASH), "Dash")?;
            let d = self.cur.char();
            if d.is_digit(10) { 
                root = Node::BinaryExpr(BinaryExprNode {
                    op: DASH,
                    left: Box::new(Node::Char(c)),
                    right: Box::new(Node::Char(d))
                });
            } else {
                return Err(ParseError::Parse(
                    format!("Dash: Expected Num-Num but got Num-{}", c)
                ));
            }
        } else if c.is_alphabetic() {
            self.advance()?;
            self.consume(OP(DASH), "Dash")?;
            let d = self.cur.char();
            if d.is_alphabetic() {
                root = Node::BinaryExpr(BinaryExprNode {
                    op: DASH,
                    left: Box::new(Node::Char(c)),
                    right: Box::new(Node::Char(d))
                });
            } 
            else {
                return Err(ParseError::Parse(
                    format!("Dash: Expected Alpha-Alpha but got Alpha-{}", c)
                ));
            }
        } else {
            return Err(ParseError::Parse(
                format!("Dash: Expected Alphanumeric but got {}", c)
            ));
        }
        self.consume(GROUP(RBR), "Dash")?;
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
    use std::io::{BufReader, BufRead};
    struct TokenReader { pos: u32, tokens: Vec<Token> } 
    impl TokenReader {
        pub fn new(path: String) -> Self {
            let file = fs::File::open(path).expect("Failed to open file");
            let reader = BufReader::new(file);
            let mut tokens: Vec<Token> = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    let temp: Vec<&str> = line.split(',').collect();
                    for word in temp { 
                        let token = match word {
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
                            s => {
                                let parsed_chars: Vec<char> = s
                                    .escape_default()
                                    .collect();
                                if s.len() == 1     { CHAR(parsed_chars[0]); }
                                else if s.len() > 9 { CHAR(parsed_chars[6]); }
                                panic!("Unrecognized Token");
                            }
                        };
                        tokens.push(token);
                    }       
                }
            }
            return TokenReader { pos: 0, tokens };
        }
    }

    impl TokenGiver for TokenReader {
        fn next(&mut self) -> Result<Token, TokenErr> {
            self.pos += 1; 
            return Ok(self.tokens[(self.pos - 1) as usize]);
        }
    }

    #[test]
    fn find_valid_invalid() {
        // figure out why I need to_owned() here.
        let in_path = "./test_data/parser/input";
        let out_path = "./test_data/parser/output";
        if let Ok(entries) = fs::read_dir(in_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let tr = TokenReader::new(
                        format!("{}/{:?}", in_path, file_name)
                    );

                    let ans= fs::read_to_string(
                        format!("{}/{:?}", out_path, file_name)
                        )
                        .unwrap()
                        .chars()
                        .nth(0)
                        .expect("Shouldn't Be Empty!");
                    let mut parser = Parser::new(tr).expect("Invalid Token Stream");
                    let matches = parser.parse();
                    assert!(matches.is_err() == (ans == 'F'));
                }
            }
        } else {
            println!("Failed to read directory");
        }
        assert_eq!(4, 4);
    }
}