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
    pub name: String
}
impl Node {
    pub fn char(&self) -> char {
        if let Node::Char(c) = *self { return c; }
        panic!("Not A Letter!");
    }
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
                out.push_str(&format!(
                    "{tabs}<\"{}\"> </\"{}\">\n",
                    c.escape_debug(), c.escape_debug()
                ));
            }
        }
    }
}