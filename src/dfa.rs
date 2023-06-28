use crate::nfa::{NFA};
struct DFA {
    ncount: usize,
    jumps: Vec<[usize; 128]>,
    accepts: Vec<usize>,
    labels: Vec<String>
}

impl DFA {
    pub fn new() -> Self {
        return DFA {
            ncount: 0,
            jumps: Vec::new(),
            accepts: Vec::new(),
            labels: Vec::new()
        };
    }

    /* A few optimization can be performed.
     * Compression into a bit vector.
     * Hashing? (input size may be too small to be effective)
     */
    fn subset_construction(&mut self, nfa: NFA) {
        let mut Dstates: Vec<Vec<usize>> = Vec::new();
        Dstates.push(DFA::eps_closure(&nfa, vec![0; 1]));
        let mut unmarked = vec![0usize; 1];
        while let Some(index) = unmarked.pop() {
            for c in 0..=127u8 {
                let mut nxt: Vec<usize> = Vec::new();
                for d in &Dstates[index] { 
                    nxt.push(nfa.nodes[*d].jumps[c as usize])
                }
                let U = DFA::eps_closure(
                    &nfa,
                    nxt,
                );
                let mut has = false;
                for d in &Dstates {
                    if *d == U { has = true; }
                }
                if has { continue; }
                Dstates.push(U.clone());
                let u = self.addState();
                self.jumps[u][c as usize] = index;
                self.accepts[u] = DFA::accepts(&nfa, U);
                unmarked.push(u);
            }
        }
    }

    fn eps_closure(nfa: &NFA, T: Vec<usize>) -> Vec<usize> {
        let mut has = vec![false; nfa.ncount];
        let mut closure: Vec<usize> = T;
        let mut stack: Vec<usize> = Vec::new();
        for s in &closure { stack.push(*s); }
        while let Some(s) = stack.pop() {
            for nbr in &nfa.nodes[s].eps {
                if has[*nbr] { continue; }
                has[nfa.nodes[*nbr].id] = true;
                closure.push(*nbr);
                stack.push(*nbr);
            }
        } 
        return closure;
    }

    fn accepts(nfa: &NFA, T: Vec<usize>) -> usize {
        let mut best: usize = 0;
        let mut bestLen: usize = 0;
        for s in T {
            let acc = nfa.nodes[s].accept;
            if acc != 0 && nfa.labels[acc].len() > bestLen {
                best = s;
                bestLen = nfa.labels[acc].len();
            }
        }
        return best;
    }

    fn addState(&mut self) -> usize {
        self.ncount += 1;
        self.jumps.push([0; 128]);
        self.accepts.push(0);
        return self.ncount - 1;
    }
}