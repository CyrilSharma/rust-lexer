use crate::nfa::{NFA};
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
            for c in 0..=127u8 {
                let mut nxt: Vec<usize> = Vec::new();
                for d in &d_states[index] { 
                    nxt.push(nfa.jumps[*d][c as usize])
                }
                let U = DFA::eps_closure(
                    &nfa,
                    nxt,
                );
                let mut has = false;
                for d in &d_states {
                    if *d == U { has = true; }
                }
                if has { continue; }
                d_states.push(U.clone());
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
            for nbr in &nfa.eps[s] {
                if has[*nbr] { continue; }
                has[*nbr] = true;
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
            let acc = nfa.accepts[s];
            // TODO: Tiebreaker should be longest matched token, not longest label length.
            if acc != usize::MAX && nfa.labels[acc].len() > bestLen {
                best = s;
                bestLen = nfa.labels[acc].len();
            }
        }
        return best;
    }

    fn addState(&mut self) -> usize {
        self.ncount += 1;
        self.jumps.push([0; u8::MAX as usize]);
        self.accepts.push(0);
        return self.ncount - 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn thing() {}
}