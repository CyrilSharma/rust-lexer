use crate::{ast, lexer};
use typed_arena::Arena;
/* I got the idea for Arenas from here - 
 * https://github.com/nrc/r4cppp/blob/master/graphs/README.md 
 */

struct Node {
    jumps: Vec<*mut Node>,
    eps_jumps: Vec<*mut Node>,
    accept: Option<String>,
}
/* TODO: some functionality for iterating over and generating DFA */
impl Node {
    fn add(from: *mut Node, c: char, to: *mut Node) {
        unsafe { (*from).jumps[c as usize] = to; }
    }
    fn add_eps(from: *mut Node, to: *mut Node) {
        unsafe { (*from).eps_jumps.push(to); }
    }
    fn copy_from(src: *mut Node, dest: *mut Node) {
        unsafe {
            std::mem::swap(&mut(*dest).jumps, &mut(*src).jumps);
            std::mem::swap(&mut(*dest).eps_jumps, &mut(*src).eps_jumps);
        }
    }
    fn label(from: *mut Node, label: Option<String>) {
        unsafe { (*from).accept = label; }
    }
}
pub struct NFA { 
    arena: Arena<Node>,
    start: *mut Node, 
}
impl NFA {
    pub fn new() -> Self {
        return NFA { 
            arena: Arena::new(),
            start: std::ptr::null_mut(),
        };
    }

    pub fn from_matches(&mut self, matches: &Vec<ast::Match>) {
        let root = self.make_node();
        for m in matches {
            Node::add_eps(
                root,
                self.build_ast(&m.root, m.name.clone())
            );
        }
        self.start = root;
    }

    fn build_ast(&mut self, ast: &ast::Node, label: Option<String>) -> *mut Node {
        let (start, end) = self.build(ast);
        Node::label(end, label);
        return start;
    }


    fn build(&mut self, ast: &ast::Node) -> (*mut Node, *mut Node) {
        return match ast {
            ast::Node::BinaryExpr(node) => {
                let left = self.build(&node.left);
                let right = self.build(&node.right);
                match node.op {
                    lexer::Op::BAR => self.handle_bar(left, right),
                    lexer::Op::PLUS => self.handle_dash(node.left.char(), node.right.char()),
                    lexer::Op::AND => self.handleAdd(left, right),
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

    fn handle_bar(&mut self, left: (*mut Node, *mut Node),
        right: (*mut Node, *mut Node)) -> (*mut Node, *mut Node) {
        let mut i = self.make_node();
        let mut f = self.make_node();
        Node::add_eps(i, left.0);
        Node::add_eps(i, right.0);
        Node::add_eps(left.1, f);
        Node::add_eps(right.1, f);
        return (i, f);
    }

    fn handle_dash(&mut self, start: char, end: char) -> (*mut Node, *mut Node) {
        let i = self.make_node();
        let f = self.make_node();
        for c in start..end { 
            Node::add(i, c, f);
        }
        return (i, f);
    }

    fn handleAdd(&mut self, left: (*mut Node, *mut Node), right: (*mut Node, *mut Node)) 
        -> (*mut Node, *mut Node) {
        let (_, lf) = left;
        let (ri, _) = right;
        Node::copy_from(ri, lf);
        return (left.0, right.1);
    }

    fn handle_question(&mut self, child: (*mut Node, *mut Node)) -> (*mut Node, *mut Node) {
        let (start, end) = child;
        let i = self.make_node();
        let f = self.make_node();
        Node::add_eps(i, start);
        Node::add_eps(i, f);
        Node::add_eps(end, f);
        return (i, f);
    }

    fn handle_plus(&self, child: (*mut Node, *mut Node)) -> (*mut Node, *mut Node) {
        let (start, end) = child;
        let i = self.make_node();
        let f = self.make_node();
        Node::add_eps(i, start);
        Node::add_eps(end, start);
        Node::add_eps(end, f);
        return (i, f);
    }

    fn handle_star(&self, child: (*mut Node, *mut Node)) -> (*mut Node, *mut Node) {
        let (start, end) = child;
        let i = self.make_node();
        let f = self.make_node();
        Node::add_eps(i, start);
        Node::add_eps(i, f);
        Node::add_eps(end, start);
        Node::add_eps(end, f);
        return (i, f);
    }

    fn handle_char<'b>(&'b self, c: char) -> (*mut Node, *mut Node) {
        let i = self.make_node();
        let f = self.make_node();
        Node::add(i, c, f);
        return (i, f);
    }

    fn make_node(&self) -> *mut Node {
        let node = self.arena.alloc(Node {
            jumps: vec![std::ptr::null_mut(); 128],
            eps_jumps: Vec::new(),
            accept: None,
        });
        return node as *mut Node;
    }
}