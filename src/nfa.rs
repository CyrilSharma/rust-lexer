use crate::{ast, lexer};
pub struct NFA { 
    pub ncount:  usize,
    pub jumps:   Vec<Vec<usize>>,
    pub eps:     Vec<Vec<usize>>,
    pub accepts: Vec<usize>,
    pub labels:  Vec<String>
}
impl NFA {
    pub fn new() -> Self {
        return NFA { 
            ncount:  0,
            jumps:   Vec::new(),
            eps:     Vec::new(),
            accepts: Vec::new(),
            labels:  Vec::new()
        };
    }

    pub fn from_matches(&mut self, matches: &Vec<ast::Match>) {
        let root = self.make_node();
        for m in matches {
            let node = self.build_ast(&m.root, m.name.to_string());
            self.add_eps(root,node);
        }
    }

    fn build_ast(&mut self, ast: &ast::Node, label: String) -> usize {
        let (start, end) = self.build(ast);
        self.label(end, label);
        return start;
    }


    fn build(&mut self, ast: &ast::Node) -> (usize, usize) {
        return match ast {
            ast::Node::BinaryExpr(node) => {
                let left = self.build(&node.left);
                let right = self.build(&node.right);
                match node.op {
                    lexer::Op::BAR => self.handle_bar(left, right),
                    lexer::Op::PLUS => self.handle_dash(node.left.char(), node.right.char()),
                    lexer::Op::AND => self.handle_add(left, right),
                    _ => panic!("Expected Binary Op but got {:?}", node.op)
                }
            },
            ast::Node::UnaryExpr(node) => {
                let child = self.build(&node.child);
                match node.op {
                    lexer::Op::STAR => self.handle_star(child),
                    lexer::Op::PLUS => self.handle_plus(child),
                    lexer::Op::QUESTION => self.handle_question(child),
                    _ => panic!("Expected Unary Op but got {:?}", node.op)
                }
            },
            ast::Node::Char(c) => self.handle_char(*c)
        }
    }

    fn handle_bar(&mut self, left: (usize, usize),
        right: (usize, usize)) -> (usize, usize) {
        let i = self.make_node();
        let f = self.make_node();
        self.add_eps(i, left.0);
        self.add_eps(i, right.0);
        self.add_eps(left.1, f);
        self.add_eps(right.1, f);
        return (i, f);
    }

    fn handle_dash(&mut self, start: char, end: char) -> (usize, usize) {
        let i = self.make_node();
        let f = self.make_node();
        for c in start..end { 
            self.add(i, f, c);
        }
        return (i, f);
    }

    fn handle_add(&mut self, left: (usize, usize), right: (usize, usize)) 
        -> (usize, usize) {
        let (_, lf) = left;
        let (ri, _) = right;
        self.swap(lf, ri);
        return (left.0, right.1);
    }

    fn handle_question(&mut self, child: (usize, usize)) -> (usize, usize) {
        let (start, end) = child;
        let i = self.make_node();
        let f = self.make_node();
        self.add_eps(i, start);
        self.add_eps(i, f);
        self.add_eps(end, f);
        return (i, f);
    }

    fn handle_plus(&mut self, child: (usize, usize)) -> (usize, usize) {
        let (start, end) = child;
        let i = self.make_node();
        let f = self.make_node();
        self.add_eps(i, start);
        self.add_eps(end, start);
        self.add_eps(end, f);
        return (i, f);
    }

    fn handle_star(&mut self, child: (usize, usize)) -> (usize, usize) {
        let (start, end) = child;
        let i = self.make_node();
        let f = self.make_node();
        self.add_eps(i, start);
        self.add_eps(i, f);
        self.add_eps(end, start);
        self.add_eps(end, f);
        return (i, f);
    }

    fn handle_char(&mut self, c: char) -> (usize, usize) {
        let i = self.make_node();
        let f = self.make_node();
        self.add(i, f, c);
        return (i, f);
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.jumps.swap(i, j);
        self.eps.swap(i, j);
    }

    fn label(&mut self, i: usize, label: String) {
        self.labels.push(label);
        self.accepts[i] = self.labels.len();
    }

    fn add_eps(&mut self, i: usize, f: usize) {
        self.eps[i].push(f);
    }

    fn add(&mut self, i: usize, f: usize, c: char) {
        self.jumps[i][c as usize] = f;
    }

    fn make_node(&mut self) -> usize {
        self.ncount += 1;
        self.jumps.push(vec![0; usize::MAX]);
        self.eps.push(Vec::new());
        return self.ncount - 1;
    }
}