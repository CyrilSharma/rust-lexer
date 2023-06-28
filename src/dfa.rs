use crate::nfa::{NFA};
struct DFA {
    jumps: Vec<[usize; 128]>,
    accepts: Vec<[usize; 128]>,
    labels: Vec<String>
}
impl DFA {
    pub fn new() -> Self {
        return DFA {
            jumps: Vec::new(),
            accepts: Vec::new(),
            labels: Vec::new()
        };
    }
}