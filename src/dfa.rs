use std::rc::{Rc, Weak};
use crate::ast;

// Just Tables
const EPS: usize = 128;
const NONE: Option<Rc<Node>> = None;
struct Node {
    jumps: Vec<Option<Rc<Node>>>,
    eps_jumps: Vec<Rc<Node>>,
    accept: bool
}
impl Node {
    pub fn new(accept: bool) -> Self {
        return Node {
            jumps: vec![NONE; 128],
            eps_jumps: Vec::new(),
            accept
        }
    }
    pub fn add(&self, c: char, mut to: Rc<Node>) {
        self.jumps[c as usize] = Some(to);
    }
    pub fn addEps(&self, mut to: Rc<Node>) {
        self.eps_jumps.push(Rc::clone(&to));
    }
}
struct NFA {
    jumps: Vec<[usize; 128]>,
    eps_jumps: Vec<Vec<usize>>,
    accepts: Vec<usize>,
    labels: Vec<String>
}
impl NFA {
    pub fn new() -> Self {
        let mut jumps: Vec<[usize; 128]> = vec![[0; 128]; 1];
        let mut eps_jumps: Vec<Vec<usize>> = vec![Vec::new(); 1];
        let mut accepts: Vec<usize> = vec![0; 1];
        let labels: Vec<String> = Vec::new();
        return NFA { jumps, eps_jumps, accepts, labels }
    }
    pub fn build(&self, ast: &ast::Node, label: &str) {
        let mut dead: Vec<usize> = Vec::new();
        let () = self._build(ast);
        self.labels.push(label.to_string());   
    }
    pub fn _build(&self, ast: &ast::Node) -> (Rc<Node>, Rc<Node>) {
        return match *ast {
            ast::Node::BinaryExpr(node) => {
                let left = self._build(&node.left);
                let right = self._build(&node.right);
                match node.op {
                    BAR => self.handleAdd(left, right),
                    PLUS => self.handleDash(node.left.char(), node.right.char()),
                    ADD => self.handleBar(left, right),
                    _ => panic!("Expected Binary Op but got {:?}", node.op)
                }
            },
            ast::Node::UnaryExpr(node) => {
                let child = self._build(&node.child);
                match node.op {
                    STAR => self.handleStar(child),
                    PLUS => self.handlePlus(child),
                    QUESTION => self.handleQuestion(child),
                    _ => panic!("Expected Unary Op but got {:?}", node.op)
                }
            },
            ast::Node::Char(c) => self.handleChar(c)
        }
    }

    pub fn handleBar(&self, left: (Rc<Node>, Rc<Node>), 
        right: (Rc<Node>, Rc<Node>)) -> (Rc<Node>, Rc<Node>) {
        let mut i = Rc::new(Node::new(false));
        let mut f = Rc::new(Node::new(true));
        i.addEps(left.0);
        i.addEps(right.0);
        left.1.addEps(f);
        right.1.addEps(f);
        return (i, f);
    }

    pub fn handleDash(&self, start: char, end: char) -> (Rc<Node>, Rc<Node>) {
        let mut i = Rc::new(Node::new(false));
        let mut f = Rc::new(Node::new(true));
        for c in start..end { i.add(c, f); }
        return (i, f);
    }

    pub fn handleAdd(&self, left: (Rc<Node>, Rc<Node>), 
        right: (Rc<Node>, Rc<Node>)) -> (Rc<Node>, Rc<Node>) {
        let (_, mut lf) = left;
        let (mut ri, _) = right;
        lf.jumps = ri.jumps;
        lf.eps_jumps = ri.eps_jumps;
        lf.accept = false;
        return (left.0, right.1);
    }

    pub fn handleQuestion(&self, child: (Rc<Node>, Rc<Node>)) 
        -> (Rc<Node>, Rc<Node>) {
        let (mut start, mut end) = child;
        let mut i = Rc::new(Node::new(false));
        let mut f = Rc::new(Node::new(true));
        i.addEps(start);
        i.addEps(f);
        end.addEps(f);
        end.accept = false;
        return (i, f);
    }

    pub fn handlePlus(&self, child: (Rc<Node>, Rc<Node>)) 
        -> (Rc<Node>, Rc<Node>) {
        let (mut start, mut end) = child;
        let mut i = Rc::new(Node::new(false));
        let mut f = Rc::new(Node::new(true));
        i.addEps(start);
        end.addEps(start);
        end.addEps(f);
        end.accept = false;
        return (i, f);
    }

    pub fn handleStar(&self, child: (Rc<Node>, Rc<Node>)) 
        -> (Rc<Node>, Rc<Node>) {
        let (mut start, mut end) = child;
        let mut i = Rc::new(Node::new(false));
        let mut f = Rc::new(Node::new(true));
        i.addEps(start);
        i.addEps(f);
        end.addEps(start);
        end.addEps(f);
        end.accept = false;
        return (i, f)
    }

    pub fn handleChar(&self, c: char) -> (Rc<Node>, Rc<Node>) {
        let mut i = Rc::new(Node::new(false)); 
        let mut f = Rc::new(Node::new(true));
        i.add(c, i);
        return (i, f);
    }
}

struct DFA {
    jumps: Vec<[u32; 129]>,
    accept: Vec<u32>,
    labels: Vec<String>
}
impl DFA {
    pub fn new(nfa: NFA) -> Self {

    }
}

fn buildAutomota(matches: Vec<ast::Match>) -> DFA {

}



