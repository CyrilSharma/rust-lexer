use crate::nfa::{NFA};
const NULL: usize = usize::MAX;
pub struct DFA {
    pub ncount:  usize,
    pub jumps:   Vec<[usize; u8::MAX as usize]>,
    pub accepts: Vec<usize>,
    pub labels:  Vec<String>
}

#[allow(dead_code)]
impl DFA {
    pub fn new() -> Self {
        return DFA {
            ncount:  0,
            jumps:   Vec::new(),
            accepts: Vec::new(),
            labels:  Vec::new()
        };
    }

    fn subset_construction(&mut self, nfa: NFA) {
        let mut d_states: Vec<Vec<usize>> = Vec::new();
        d_states.push(DFA::eps_closure(&nfa, vec![0; 1]));
        let mut unmarked = vec![0usize; 1];
        while let Some(index) = unmarked.pop() {
            for c in 0..u8::MAX {
                // MOVE
                let mut has = vec![false; nfa.ncount];
                let mut nxt: Vec<usize> = Vec::new();
                for d in &d_states[index] {
                    if has[*d] { continue; }
                    let mv = nfa.jumps[*d][c as usize];
                    if mv == NULL { continue; }
                    nxt.push(mv);
                    has[*d] = true;
                }

                let state = DFA::eps_closure(&nfa,nxt);

                // Seen Before?
                let mut u: usize = self.jumps.len();
                for i in 0..d_states.len() {
                    if d_states[i] == state {
                        u = i;
                        break;
                    }
                }
                if u == self.jumps.len() { 
                    d_states.push(state.clone()); 
                    u = self.add_state(DFA::accepts(&nfa, state));
                    unmarked.push(u);
                }
                self.jumps[index][c as usize] = u;
            }
        }
    }

    fn eps_closure(nfa: &NFA, set: Vec<usize>) -> Vec<usize> {
        let mut has = vec![false; nfa.ncount];
        let mut closure: Vec<usize> = set;
        let mut stack: Vec<usize> = Vec::new();
        for s in &closure { 
            has[*s] = true;
            stack.push(*s); 
        }
        while let Some(s) = stack.pop() {
            for nbr in &nfa.eps[s] {
                if has[*nbr] { continue; }
                has[*nbr] = true;
                closure.push(*nbr);
                stack.push(*nbr);
            }
        } 
        return closure;
    }

    fn accepts(nfa: &NFA, set: Vec<usize>) -> usize {
        for s in set {
            let acc = nfa.accepts[s];
            if acc != 0 { return acc; }
        }
        return 0;
    }

    fn add_state(&mut self, accept: usize) -> usize {
        self.ncount += 1;
        self.jumps.push([usize::MAX; u8::MAX as usize]);
        self.accepts.push(accept);
        return self.ncount - 1;
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, fs::File, io::{BufReader, BufRead}};
    use crate::{lexer::Lexer, parser::Parser, nfa::NFA};
    use super::*;

    impl DFA {
        fn takes(&self, s: &str) -> bool {
            let mut state = 0;
            let chars = s.chars();
            for c in chars {
                let nxt = self.jumps[state][c as usize];
                if nxt == NULL { return false; }
                state = nxt;
            }
            return self.accepts[state] != 0;
        }
        fn print_dot(&self) {
            println!("digraph TransitionTable {{");
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
                            state, nbr, ind as char
                        );
                    } else {
                        println!("\t{} -> {} [label=\"{}\"];",
                            state, nbr,
                            format!("{}-{}", start as char, ind as char)
                        );
                    }
                    ind += 1;
                }
            }
            println!("}}");
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
            let mut nfa = NFA::new();
            nfa.build_from_matches(&parser.parse().expect("Invalid parse"));
            let mut dfa = DFA::new();
            dfa.subset_construction(nfa);

            dfa.print_dot();
            for id in ["right", "wrong"] {
                let file = File::open(&format!("{path}/{id}-words-{i}.txt"))
                    .expect("Should be valid...");
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(word) = line {
                        assert!(dfa.takes(&word) == (id == "right"));
                    }
                }
            }
            i += 1;
        }
    }
}