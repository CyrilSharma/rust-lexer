use crate::{ast::{self, Match}, lexer};

const NULL: usize = usize::MAX;
pub struct NFA { 
    pub ncount:  usize,
    pub jumps:   Vec<[usize; u8::MAX as usize]>,
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

    pub fn build_from_matches(matches: &Vec<ast::Match>) -> Self {
        let mut nfa = NFA::new();
        let root = nfa.make_node();
        for m in matches {
            if m.name.len() == 0 {
                nfa.handle_whitespace(&m.root);
            } else {
                let node = NFA::build_ast(&mut nfa, m);
                nfa.add_eps(root,node);
            }
        }
        return nfa;
    }

    fn handle_whitespace(&mut self, ast: &ast::Node) {
        match ast {
            ast::Node::BinaryExpr(_) => {
                let mut stk: Vec<&ast::Node> = vec![ast; 1];
                while let Some(state) = stk.pop() {
                    match state {
                        ast::Node::BinaryExpr(node) => {
                            if let lexer::Op::BAR = node.op {
                                stk.push(&node.left);
                                stk.push(&node.right);
                            } else {
                                panic!("You used something besides or in whitespace!");
                            }
                        },
                        ast::Node::Char(c) => self.add(0, 0, *c),
                        _ => panic!("Neither Char no Or in whitespace!")
                    }
                }
            },
            ast::Node::Char(c) => self.add(0, 0, *c),
            _ => panic!("White Space should be single or-seperated tokens!")
        }
    }

    fn build_ast(nfa: &mut NFA, m: &Match) -> usize {
        let (start, end) = nfa.build(&m.root);
        nfa.label(end, m.name.clone());
        return start;
    }


    fn build(&mut self, ast: &ast::Node) -> (usize, usize) {
        return match ast {
            ast::Node::BinaryExpr(node) => {
                let left = self.build(&node.left);
                let right = self.build(&node.right);
                match node.op {
                    lexer::Op::BAR  => self.handle_bar(left, right),
                    lexer::Op::DASH => self.handle_dash(node.left.char(), node.right.char()),
                    lexer::Op::AND  => self.handle_add(left, right),
                    _ => panic!("Expected Binary Op but got {:?}", node.op)
                }
            },
            ast::Node::UnaryExpr(node) => {
                let child = self.build(&node.child);
                match node.op {
                    lexer::Op::STAR     => self.handle_star(child),
                    lexer::Op::PLUS     => self.handle_plus(child),
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
        for c in start..=end { 
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

    fn label(&mut self, i: usize, label: String) {
        self.labels.push(label);
        self.accepts[i] = self.labels.len();
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.jumps.swap(i, j);
        self.eps.swap(i, j);
    }

    fn add_eps(&mut self, i: usize, f: usize) {
        self.eps[i].push(f);
    }

    fn add(&mut self, i: usize, f: usize, c: char) {
        self.jumps[i][c as usize] = f;
    }

    fn make_node(&mut self) -> usize {
        self.ncount += 1;
        self.jumps.push([NULL; u8::MAX as usize]);
        self.eps.push(Vec::new());
        self.accepts.push(0);
        return self.ncount - 1;
    }

    // #[cfg(test)]
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn print_dot(&self) {
        println!("digraph NFA {{");
        for state in 0..self.ncount {
            let mut ind = 0;
            while ind < u8::MAX {
                let nbr = self.jumps[state][ind as usize];
                if nbr == NULL { ind += 1; continue };

                let start = ind;
                while ind + 1 < u8::MAX &&
                    self.jumps[state][(ind + 1) as usize] == nbr {
                    ind += 1;
                }

                if start == ind {
                    println!("\t{} -> {} [label=\"{}\"];",
                        state, nbr, (ind as char).escape_debug()
                    );
                } else {
                    println!("\t{} -> {} [label=\"{}\"];",
                        state, nbr,
                        format!("{}-{}", (start as char).escape_debug(),
                            (ind as char).escape_debug()
                        )
                    );
                }
                ind += 1;
            }
            for nbr in &self.eps[state] {
                println!("\t{} -> {} [label=\"eps\"];",
                    state, *nbr
                );
            }
        }
        println!("}}");
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, fs::File, io::{BufReader, BufRead}};
    use crate::{lexer::Lexer, parser::Parser};

    use super::*;
    impl NFA {
        fn accepts(&self, s: &str) -> bool {
            let mut states: Vec<usize> = self.eps_closure(vec![0; 1]);
            for c in s.chars() {
                let mut has = vec![false; self.ncount];
                let mut mv: Vec<usize> = Vec::new();
                for s in &states {
                    let nxt = self.jumps[*s][c as usize];
                    if nxt != NULL && !has[nxt] { 
                        has[nxt] = true;
                        mv.push(nxt); 
                    } 
                }
                states = self.eps_closure(mv);
            }

            for state in states {
                if self.accepts[state] != 0 { 
                    return true; 
                }
            }
            return false;
        }

        fn eps_closure(&self, set: Vec<usize>) -> Vec<usize> {
            let mut has = vec![false; self.ncount];
            let mut closure: Vec<usize> = set;
            let mut stack: Vec<usize> = Vec::new();
            for s in &closure { 
                stack.push(*s); 
                has[*s] = true;
            }
            while let Some(s) = stack.pop() {
                for nbr in &self.eps[s] {
                    if has[*nbr] { continue; }
                    has[*nbr] = true;
                    closure.push(*nbr);
                    stack.push(*nbr);
                }
            }
            return closure;
        }
    }

    #[test]
    fn test_matches() {
        let path = "tests/data/regex/input";
        let mut i = 0;
        while Path::new(&format!("{path}/match-{i}.txt")).exists() {
            println!("{}", format!("{path}/match-{i}.txt"));
            let lexer = Lexer::new(&format!("{path}/match-{i}.txt")).expect("Invalid Path");
            let mut parser = Parser::new(lexer).expect("File should be non-empty!");
            let nfa = NFA::build_from_matches(&parser.parse().expect("Invalid parse"));
            // nfa.print_dot();
            for id in ["right", "wrong"] {
                let file = File::open(&format!("{path}/{id}-words-{i}.txt"))
                    .expect("Should be valid...");
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(word) = line {
                        assert!(nfa.accepts(&word) == (id == "right"));
                    }
                }
            }
            i += 1;
        }
    }
}