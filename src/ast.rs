use crate::lexer::Op;

pub enum Node {
    Char(char),
    BinaryExpr(BinaryExprNode),
    UnaryExpr(UnaryExprNode),
}

pub struct BinaryExprNode {
    pub left:  Box<Node>,
    pub right: Box<Node>,
    pub op: Op,
}

pub struct UnaryExprNode {
    pub child: Box<Node>,
    pub op: Op,
}

pub struct AST {
    pub root: Node,
    pub name: Option<String>
}
impl AST {
    pub fn print(&self) {
        self._print(&self.root, 0);
    }
    /* OUTPUT TO XML? */
    fn _print(&self, node: &Node, depth: u32) {
        let mut tabs = String::new();
        for _ in 0..depth { tabs.push_str("  "); }
        match node {
            Node::BinaryExpr(n) => {
                println!("{tabs}<{:?}>", n.op);
                self._print(&n.left, depth+1);
                self._print(&n.right, depth+1);
                println!("{tabs}</{:?}>", n.op);
            },
            Node::UnaryExpr(n) => {
                println!("{tabs}<{:?}>", n.op);
                self._print(&n.child, depth+1);
                println!("{tabs}</{:?}>", n.op);
            },
            Node::Char(c) => {
                // just so I don't get xml issues.
                let name: String = match c {
                    'A'..='Z' | 'a'..='z' => c.to_string(),
                    '0' => "ZERO".to_string(),
                    '1' => "ONE".to_string(),
                    '2' => "TWO".to_string(),
                    '3' => "THREE".to_string(),
                    '4' => "FOUR".to_string(),
                    '5' => "FIVE".to_string(),
                    '6' => "SIX".to_string(),
                    '7' => "SEVEN".to_string(),
                    '8' => "EIGHT".to_string(),
                    '9' => "NINE".to_string(),
                    '|' => "BAR".to_string(),
                    ']' => "SQRBRKR".to_string(),
                    '[' => "SQRBRKL".to_string(),
                    '}' => "SQRBRKR".to_string(),
                    '{' => "SQRBRKL".to_string(),
                    '\\' => "BSLASH".to_string(),
                    '/' => "FSLASH".to_string(),
                    '\n' => "NEWLINE".to_string(),
                    _ => panic!("Not Implemented!")
                };
                println!("{tabs}<{name}> </{name}>");
                
            }
        }
    }
}