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

pub struct Match {
    pub root: Node,
    pub name: Option<String>
}
impl Node {
    pub fn print(&self) {
        print!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        self._print(&self, 0, &mut out);
        return out;
    } 

    /* OUTPUT TO XML? */
    fn _print(&self, node: &Node, depth: u32, out: &mut String) {
        let mut tabs = String::new();
        for _ in 0..depth { tabs.push_str("  "); }
        match node {
            Node::BinaryExpr(n) => {
                out.push_str(&format!("{tabs}<{:?}>\n", n.op));
                self._print(&n.left, depth+1, out);
                self._print(&n.right, depth+1, out);
                out.push_str(&format!("{tabs}</{:?}>\n", n.op));
            },
            Node::UnaryExpr(n) => {
                out.push_str(&format!("{tabs}<{:?}>\n", n.op));
                self._print(&n.child, depth+1, out);
                out.push_str(&format!("{tabs}</{:?}>\n", n.op));
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
                out.push_str(&format!("{tabs}<{name}> </{name}>\n"));
            }
        }
    }
}