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
            GROUP(LBR) => return Ok(self.bracketed()?),
            token => Err(ParseError::Parse(
                format!("Atom: Expected CHAR, [, (, but found {:?}", token)
            ))
        }
    }

    fn bracketed(&mut self) -> Result<Node, ParseError> {
        let mut root: Option<Node> = None;
        loop { match self.cur {
            CHAR(_) => {
                let res = match self.lexer.peek()? {
                    OP(DASH) => self.dash()?,
                    CHAR(_) | GROUP(RBR) => self.atom()?,
                    t => return Err(ParseError::Parse(format!(
                        "Expected Char or Dash got {:?}", t)
                    )),
                };
                root = match root {
                    None => Some(res),
                    Some(node) => Some(Node::BinaryExpr(
                        BinaryExprNode { 
                            left: Box::new(node), 
                            right: Box::new(res), 
                            op: BAR 
                        }
                    ))
                }
            },
            GROUP(RBR) => break,
            t => return Err(ParseError::Parse(format!(
                "Expected ] or Char got {:?}", t
            )))
        }}
        self.consume(GROUP(RBR), "Dashes")?;
        match root {
            None => return Err(ParseError::Parse(
                format!("Invalid Bracketed Expression")
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
    use std::{fs, path::Path};
    use crate::lexer::Lexer;

    #[test]
    fn ast() {
        let mut i = 0;
        while Path::new(&format!("tests/data/parser/input/AST-{i}.txt")).exists() {
            let inpath = &format!("tests/data/parser/input/AST-{i}.txt");
            let outpath = &format!("tests/data/parser/output/AST-{i}.txt");
            let tr = Lexer::new(&inpath).expect("File Doesn't Exist");
            let mut parser = Parser::new(tr).expect("Invalid Token Stream");
            let matches = parser.parse().expect("Expression should be valid.");
            for m in matches { 
                let ans: String = fs::read_to_string(outpath).expect("File doesn't exist.");
                assert!(ans.trim() == m.root.to_string().trim());
            }
            i += 1;
        }
    }
}