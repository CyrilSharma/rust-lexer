use std::rc::{Rc};
use crate::nfa::{NFA};

struct Node {
    jumps: Vec<Rc<Node>>,
    accept: Option<String>
}
struct DFA { start: Rc<Node> }
impl DFA {
   /*  
    pub fn new(nfa: NFA) -> Self {

    } 
    */
}