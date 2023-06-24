use crate::lexer::{TokenGiver, Token, Group, Op};
use Token::*;
use Group::*;
use Op::*;

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

pub struct Match {
    expr: Node,
    name: Option<String>
}

struct Parser<T: TokenGiver> { 
    cur: Token,
    lexer: T,
}

impl<T: TokenGiver> Parser<T> {
    pub fn new(mut lexer: T) -> Self {
        Parser { 
            cur: lexer.next(), 
            lexer,
        }
    }

    fn advance(&mut self) { 
        self.cur = self.lexer.next();
    }

    fn consume(&mut self, token: Token) {
        if token == self.cur {
            self.advance();
        } else {
            panic!("Expected Different Token!");
        }
    }

    pub fn parse(&mut self) -> Vec<Match> {
        let mut matches = Vec::new();
        while let GROUP(DBQ) = self.cur {
            self.consume(GROUP(DBQ));
            let expr = self.expr();
            self.consume(GROUP(DBQ));
            if let SEMI = self.cur {
                self.consume(SEMI);
                matches.push(Match { expr, name: None });
            } else { 
                let name = self.name(); 
                matches.push(Match { expr, name: Some(name) });
            }
        }
        if self.cur != EOF { 
            panic!("Invalid Construction!");
        }
        return matches;
    }

    fn expr(&mut self) -> Node {
        let mut root = self.term();
        while let OP(BAR) = self.cur {
            self.advance();
            let term = self.term();
            let new_root = BinaryExprNode {
                op: BAR,
                left: Box::new(root),
                right: Box::new(term)
            };
            root = Node::BinaryExpr(new_root);
        }
        return root;
    }

    fn term(&mut self) -> Node {
        let mut root = self.factor();
        while let CHAR(_) | GROUP(LPR) = self.cur {
            let node= self.factor();
            let new_root = BinaryExprNode {
                op: AND,
                left: Box::new(root),
                right: Box::new(node)
            };
            root = Node::BinaryExpr(new_root);
        }
        return root;
    }

    fn factor(&mut self) -> Node {
        let node = self.atom();
        if let OP(op) = self.cur {
            if let QUESTION | STAR | PLUS = op {
                let root = UnaryExprNode { 
                    op, child: Box::new(node)
                };
                return Node::UnaryExpr(root);
            }
            return node;
        }
        panic!("Expected Operator!");
    }

    fn atom(&mut self) -> Node {
        match self.cur {
            GROUP(LPR) => { 
                self.consume(GROUP(LPR));
                let node = self.expr();
                self.consume(GROUP(RPR));
                return node;
            },
            CHAR(c) => return Node::Char(c),
            GROUP(LBR) => return self.dash(),
            _ => panic!("Invalid ATOM!")
        }
    }

    fn dash(&mut self) -> Node {
        let mut root: Node;
        self.consume(GROUP(LBR));
        let c = self.cur.char();
        if c.is_digit(10) {
            self.advance();
            self.consume(OP(DASH));
            let d = self.cur.char();
            if d.is_digit(10) { 
                root = Node::BinaryExpr(BinaryExprNode {
                    op: DASH,
                    left: Box::new(Node::Char(c)),
                    right: Box::new(Node::Char(d))
                });
            } 
            else { panic!("Invalid Dash!"); }
        } else if c.is_alphabetic() {
            self.advance();
            self.consume(OP(DASH));
            let d = self.cur.char();
            if d.is_alphabetic() {
                root = Node::BinaryExpr(BinaryExprNode {
                    op: DASH,
                    left: Box::new(Node::Char(c)),
                    right: Box::new(Node::Char(d))
                });
            } 
            else { panic!("Invalid Dash!"); }
        } else {
            panic!("INVALID DASH");
        }
        self.consume(GROUP(RBR));
        return root;
    }

    fn name(&mut self) -> String {
        let mut name: String = String::new();
        while let CHAR(c) = self.cur {
            name.push(c);
            self.advance();
        }
        if self.cur == SEMI {
            return name;
        }
        panic!("Invalid Characters In Name!");
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Parser};
    use crate::lexer::{TokenGiver, Token, Group, Op};
    use Token::*;
    use Group::*;
    use Op::*;
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    struct TokenReader { pos: u32, tokens: Vec<Token> } 
    impl TokenReader {
        pub fn new(path: String) -> Self {
            let file = File::open(path).expect("Failed to open file");
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
        fn next(&mut self) -> Token {
            self.pos += 1; 
            return self.tokens[(self.pos - 1) as usize];
        }
    }

    #[test]
    fn test_discrimination() {
        // figure out why I need to_owned() here.
        let dir_path = "./test_data/parser/input";
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let mut tr = TokenReader::new(
                        dir_path.to_owned() +
                        file_name.to_str().unwrap()
                    );
                    let mut parser = Parser::new(tr);
                    let matches = parser.parse();
                    let result = std::panic::catch_unwind(|| parser.parse());
                    assert!(result.is_err() == );
                }
            }
        } else {
            println!("Failed to read directory");
        }
        assert_eq!(4, 4);
    }
}