use std::{fs::File, error::Error};
use std::io::Write;
use crate::dfa::{DFA, self};

struct Generator<'a> { 
    dfa: &'a DFA,
    file: File,
    tabs: usize
}

#[allow(dead_code)]
impl<'a> Generator<'a> {
    pub fn new(dfa: &'a DFA) -> Result<Self, Box<dyn Error>> {
        return Ok(Generator { 
            dfa,
            file: File::create("tokenizer.rs")?,
            tabs: 0
        });
    }
    fn write_inline(&mut self, s: &str) -> Result<(), Box<dyn Error>> {
        write!(self.file, "{}",s)?;
        return Ok(());
    }
    fn write(&mut self, s: &str) -> Result<(), Box<dyn Error>> {
        write!(self.file, "{}", &format!(
            "{}{}", 
            "\t".repeat(self.tabs),
            s
        ))?;
        return Ok(());
    }
    fn writeln(&mut self, s: &str) -> Result<(), Box<dyn Error>> {
        self.write(&format!("{s}\n"))?;
        return Ok(());
    }
    fn write_vec(&mut self, strs: &[&str]) -> Result<(), Box<dyn Error>> {
        for s in strs { self.writeln(s)?; }
        return Ok(());
    }
    fn indent(&mut self)   { self.tabs += 1; }
    fn unindent(&mut self) { self.tabs -= 1; }

    pub fn generate(&mut self) -> Result<(), Box<dyn Error>> {
        self.writeln("#[derive(Copy, Clone, Debug, PartialEq, Eq)]")?;
        self.writeln("pub enum Token {")?;
        self.indent();
        for label in &self.dfa.labels {
            if label.len() == 0 { continue; }
            self.writeln(&format!("{label}(String),"))?;
        }
        self.unindent();
        self.writeln("}")?;
        self.write_vec(&[
            "pub struct Lexer {",
            "  chars: Vec<char>,",
            "  pos:   usize",
            "}",
            "impl Lexer {",
            "    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {",
            "        let chars = fs::read_to_string(fname)?",
            "            .chars()",
            "            .collect();",
            "        return Ok(Lexer { chars, pos: 0 });",
            "    }",
            "",
            "    fn nextchar(&mut self) -> char {",
            "        self.pos += 1;",
            "        return self.chars[self.pos - 1];",
            "    }",
        ])?;
        self.indent();
        self.writeln("fn next(&mut self) -> Result<Token, TokenErr> {")?;
        self.indent();
        self.write_automota()?;
        self.unindent();
        self.writeln("}")?;
        self.unindent();
        self.writeln("}")?;
        return Ok(());
    }

    fn write_automota(&mut self) -> Result<(), Box<dyn Error>> {
        self.writeln(&format!("const dead: u32 = {};", self.dfa.dead))?;
        self.write_vec(&[
            "let mut state: u32 = 0;",
            "while state != dead {",
        ])?;
        self.indent();
        self.writeln("match state {")?;
        self.indent();
        for state in 0..self.dfa.ncount {
            self.write_transitions(state)?;
        }
        self.unindent();
        self.writeln("}")?;
        self.unindent();
        self.writeln("}")?;
        return Ok(());
    }

    fn write_transitions(&mut self, state: usize) -> Result<(), Box<dyn Error>> {
        let accepts = self.dfa.accepts[state];
        if accepts != 0 &&
            self.dfa.labels[accepts - 1].len() > 0 {
            self.writeln(
                &format!("{state} => return {},",
                self.dfa.labels[accepts - 1])
            )?;
            return Ok(());
        } else if accepts != 0 &&
            self.dfa.labels[accepts - 1].len() == 0 {
            return Ok(());
        }

        self.writeln(&format!("{state} => state = match self.char {{"))?;
        self.indent();
        let mut j = 0;
        while j < u8::MAX {
            let nbr = self.dfa.jumps[state][j as usize];
            if nbr == dfa::NULL { j += 1; continue; };
            if self.dfa.dead == nbr { j += 1; continue; }
            //println!("{} - {}", self.dfa.dead, nbr);
            let start =  j;
            while j + 1 < u8::MAX &&
                self.dfa.jumps[state][(j + 1) as usize] == nbr { 
                j += 1 
            }
            //println!("{}", self.dfa.jumps[state][j as usize]);
            if start == j {
                self.writeln(&format!("\'{}\' => {},",
                    escape(start as char),
                    self.dfa.jumps[state][j as usize]
                ))?;
            } else if start + 1 == j {
                self.writeln(&format!("\'{}\' | \'{}\' => {},",
                    escape(start as char), 
                    escape(j as char),
                    self.dfa.jumps[state][j as usize]
                ))?;
            } else {
                self.writeln(&format!(
                    "\'{}\'..=\'{}\' => {},",
                    escape(start as char),
                    escape(j as char),
                    self.dfa.jumps[state][j as usize]
                ))?;
            }
            j += 1;
        }
        self.writeln(&format!("_ => {}", self.dfa.dead))?;
        self.unindent();
        self.writeln("},")?;
        return Ok(());
    }
}

fn escape(c: char) -> String {
    match c {
        '\n' => return "\\n".to_string(),
        '\t' => return "\\t".to_string(),
        '\\' => "\\\\".to_string(),
        '\'' => "\\'".to_string(),
        '"' => "\\\"".to_string(),
        '\r' => "\\r".to_string(),
        _ => return c.to_string()
    }
}

/* TODO - Lexing Testcases... */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser, nfa::NFA};

    #[test]
    #[allow(dead_code)]
    fn visualize() {
        let path = "example.tk";
        let lexer = Lexer::new(path).expect("Invalid Path");
        let mut parser = Parser::new(lexer).expect("File should be non-empty!");
        let nfa = NFA::build_from_matches(&parser.parse().expect("Invalid parse"));
        let dfa = DFA::subset_construction(nfa);
        let mut gen = Generator::new(&dfa).expect("Just Be Better");
        gen.generate().expect("WORK");
    }
}