use crate::lexer::{Lexer, Token, Group, Op};
use Token::*;
use Group::*;
use Op::*;

pub enum Node {
    Char(char),
    BinaryExpr(BinaryExprNode),
    UnaryExpr(UnaryExprNode),
}

pub struct BinaryExprNode {
    left:  Option<Box<Node>>,
    right: Option<Box<Node>>,
    op: Op,
}

pub struct UnaryExprNode {
    child: Option<Box<Node>>,
    op: Op,
}

pub struct Match {
    expr: Node,
    name: Option<String>
}

struct Parser { 
    cur: Token,
    lexer: Lexer,
    matches: Vec<Match>
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        Parser { 
            cur: lexer.next().unwrap(), 
            lexer, 
            matches: Vec::new() 
        }
    }

    fn advance(&mut self) { 
        self.cur = self.lexer.next().unwrap(); 
    }

    fn consume(&mut self, token: Token) {
        if token == self.cur {
            self.advance();
        } else {
            panic!("Expected Different Token!");
        }
    }

    pub fn parse(&mut self) {
        while let GROUP(DBQ) = self.cur {
            self.consume(GROUP(DBQ));
            let expr = self.expr();
            self.consume(GROUP(DBQ));
            if let SEMI = self.cur {
                self.consume(SEMI);
                self.matches.push(Match { expr, name: None });
            } else { 
                let name = self.name(); 
                self.matches.push(Match { expr, name: Some(name) });
            }
        }
        if self.cur != EOF { 
            panic!("Invalid Construction!");
        }
    }

    fn expr(&mut self) -> Node {
        let mut root = self.term();
        while let OP(BAR) = self.cur {
            self.advance();
            let term = self.term();
            let new_root = BinaryExprNode {
                op: BAR,
                left: Some(Box::new(root)),
                right: Some(Box::new(term))
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
                left: Some(Box::new(root)),
                right: Some(Box::new(node))
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
                    op, child: Some(Box::new(node))
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
        self.consume(GROUP(LBR));
        let mut root = BinaryExprNode {
            op: DASH, left: None, right: None
        };
        let mut c = self.cur.char();
        root.left = Some(Box::new(Node::Char(c)));
        if c.is_digit(10) {
            self.advance();
            self.consume(OP(DASH));
            c = self.cur.char();
            if c.is_digit(10) { 
                root.right = Some(Box::new(Node::Char(c)));  
                return Node::BinaryExpr(root);
            }
        } else if c.is_alphabetic() {
            self.advance();
            self.consume(OP(DASH));
            c = self.cur.char();
            if c.is_alphabetic() {
                root.right = Some(Box::new(Node::Char(c)));  
                return Node::BinaryExpr(root);
            }
        } else {
            panic!("INVALID DASH");
        }
        self.consume(GROUP(RBR));
        return Node::BinaryExpr(root);
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
    #[test]
    fn test() {
        assert_eq!(4, 4);
    }
}