use crate::nfa::{NFA};
use crate::nfa;
struct DFA {
    ncount: usize,
    jumps: Vec<[usize; 128]>,
    accepts: Vec<[usize; 128]>,
    labels: Vec<String>
}

/* NFA MUST be well-formatted. */
impl DFA {
    pub fn new() -> Self {
        return DFA {
            jumps: Vec::new(),
            accepts: Vec::new(),
            labels: Vec::new()
        };
    }

    /* A few optimization can be performed.
     * Compression into a bit vector.
     * Hashing? (input size may be too small to be effective)
     */
    fn subset_construction(&self, nfa: NFA) {
        let mut Dstates: Vec<Vec<*mut nfa::Node>> = Vec::new();
        Dstates.push(DFA::eps_closure(vec![nfa.start; 1], nfa.ncount));
        let mut unmarked = vec![0usize; 1];
        while let Some(index) = unmarked.pop() {
            for c in 0..=127u8 {
                let nxt: Vec<*mut nfa::Node> = Vec::new();
                for s in Dstates[index] { 
                    nfa::Node::mv(s, c as char);
                }
                let (U, accepts) = DFA::eps_closure(
                    nxt,
                    nfa.ncount
                );
                let mut has = false;
                for D in Dstates {
                    if D == U { has = true; }
                }
                if has { continue; }
                
                Dstates.push(U);
                let u = self.addState();
                self.jumps[u][c as usize] = index;
                self.accepts[u] = accepts;
                unmarked.push(u);
            }
        }
    }

    fn eps_closure(T: Vec<*mut nfa::Node>, sz: usize) -> Vec<*mut nfa::Node> {
        let mut has = vec![false; sz];
        let mut closure: Vec<*mut nfa::Node> = T;
        let mut stack: Vec<*mut nfa::Node> = Vec::new();
        for s in closure { stack.push(s); }
        while let Some(s) = stack.pop() {
            for nbr in nfa::Node::eps(s) {
                if has[nfa::Node::id(nbr)] { continue; }
                has[nfa::Node::id(nbr)] = true;
                closure.push(nbr);
                stack.push(nbr);
            }
        } 
        return closure;
    }

    fn accepts(&self, T: Vec<*mut nfa::Node>) -> usize {
        let mut best: i32 = 0;
        let mut bestLen = 0;
        for s in T {
            let res = nfa::Node::accepts(nbr);
            if res != 0 {
                best = res;
                bestLen = [self.] find string
            }
        }
        return best;
    }

    fn addState(&self) -> usize {
        self.ncount += 1;
        self.jumps.push([0; 128]);
        self.accepts.push([0; 128]);
        return self.ncount - 1;
    }
}