use std::{fs::File, error::Error};
use std::io::Write;
use crate::dfa::{DFA, self};

pub struct Generator<'a> { 
    dfa: &'a DFA,
    file: File,
    tabs: usize,
}

#[allow(dead_code)]
impl<'a> Generator<'a> {
    pub fn new(dfa: &'a DFA, outpath: String) -> Result<Self, Box<dyn Error>> {
        return Ok(Generator { 
            dfa,
            file: File::create(outpath)?,
            tabs: 0,
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
        self.writeln("use std::fs;")?;
        self.writeln("use Token::*;")?;
        self.writeln("#[derive(Debug, PartialEq, Eq)]")?;
        self.writeln("pub enum Token {")?;
        self.indent();
        for label in &self.dfa.labels {
            if label.len() == 0 { continue; }
            self.writeln(&format!("{label}(String),"))?;
        }
        self.writeln("EOF")?;
        self.unindent();
        self.writeln("}")?;
        self.writeln("#[derive(Debug, PartialEq, Eq)]")?;
        self.write_vec(&[
            "pub struct TokenErr {",
            "   pub error: String",
            "}"
        ])?;
        self.write_vec(&[
            "pub struct Lexer {",
            "  chars:   Vec<char>,",
            "  pos:     usize,",
            "  begins:  Vec<usize>,",
            "  tabs:    Vec<usize>,",
            "  column:  usize,",
            &format!("  accepts: [usize; {}]", self.dfa.ncount),
            "}",
            "impl Lexer {",
            "    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {",
            "        let chars = fs::read_to_string(fname)?",
            "            .chars()",
            "            .collect();",
            &self.gen_accepts(),
            "        return Ok(Lexer { ",
            "           chars,",
            "           pos: 0,",
            "           begins: vec![0; 1],",
            "           tabs:   Vec::new(),",
            "           column: 0,",
            "           accepts",
            "        });",
            "    }",
            "",
            "   fn advance(&mut self) -> char {",
            "       let c = self.chars[self.pos];",
            "        match c {",
            "           '\\n' => {",
            "               self.column = 0;",
            "               self.begins.push(self.pos + 1);",
            "           },",
            "           '\\t' => {",
            "               self.tabs.push(self.column);",
            "               self.column += 4 - (self.column % 4);",
            "           }",
            "           _ => self.column += 1",
            "       }",
            "       self.pos += 1;",
            "       return c;",
            "   }",
            "   fn retract(&mut self) {",
            "       self.pos -= 1;",
            "       let c = self.chars[self.pos];",
            "       match c {",
            "           '\\n' => {",
            "               self.begins.pop();",
            "               self.column = self.pos - self.begins[self.begins.len() - 1];",
            "           }",
            "           '\\t' => {",
            "               self.column = self.tabs.pop().unwrap();",
            "           }",
            "           _ => self.column -= 1",
            "       }",
            "   }",
        ])?;
        self.indent();
        self.writeln("pub fn next(&mut self) -> Result<Token, TokenErr> {")?;
        self.indent();
        self.write_automota()?;
        self.unindent();
        self.writeln("}")?;
        self.unindent();
        self.writeln("}")?;
        return Ok(());
    }

    fn write_automota(&mut self) -> Result<(), Box<dyn Error>> {
        self.write_vec(&[
            "if self.pos == self.chars.len() { return Ok(EOF); }",
            "let mut stk: Vec<usize> = Vec::new();",
            "let mut chars: Vec<char> = Vec::new();",
            "let mut state: usize = 0;",
            "loop {",
        ])?;
        self.indent();
        self.writeln("if self.pos == self.chars.len() { break; }")?;
        self.writeln("let c = self.advance();")?;
        self.writeln("state = match state {")?;
        self.indent();
        for state in 0..self.dfa.ncount {
            if state == self.dfa.dead {
                self.write_vec(&[
                    &format!("{} => {{", self.dfa.dead),
                    "\tstk.push(state);",
			        "\tchars.push(c);",
                    "\tbreak;",
                    "}"
                ])?;
            } else {
                self.write_transitions(state)?;
            }
        }
        self.writeln("_ => panic!(\"Invalid State!\")")?;
        self.unindent();
        self.writeln("};")?;
        self.writeln("stk.push(state);")?;
        self.writeln("chars.push(c);")?;
        self.unindent();
        self.writeln("}")?;
        self.write_vec(&[
            "while stk.len() > 0 &&",
            "   self.accepts[stk[stk.len() - 1]] == 0 {",
            "   stk.pop().unwrap();",
            "   chars.pop().unwrap();",
            "   self.retract();",
            "}",
            "if stk.len() == 0 {",
            "    let start = self.begins[self.begins.len() - 1];",
            "    let error_line: String = self.chars[start..]",
            "        .iter()",
            "        .take_while(|&&c| c != '\\n')",
            "        .collect();",
            "    return Err(TokenErr{error: format!(",
            "        \"Failed to lex from: \\n{}\\n{}^\",",
            "        error_line,",
            "        \" \".repeat(self.column)",
            "    )});",
            "}"
        ])?;
        self.writeln("let word : String = chars.iter().collect();")?;
        self.writeln("match self.accepts[stk[stk.len() - 1]] {")?;
        self.indent();
        for (idx, label) in self.dfa.labels.iter().enumerate() {
            if label.len() == 0 { continue; }
            self.writeln(&format!(
                "{:<4} => return Ok({}(word)),",
                idx + 1, self.dfa.labels[idx],
            ))?;
        }
        self.writeln("_    => panic!(\"Invalid Accepting State\")")?;
        self.unindent();
        self.writeln("}")?;
        return Ok(());
    }

    fn write_transitions(&mut self, state: usize) -> Result<(), Box<dyn Error>> {
        self.writeln(&format!("{state} => match c {{"))?;
        self.indent();
        let mut j = 0;
        while j < u8::MAX {
            let nbr = self.dfa.jumps[state][j as usize];
            // the only self-transitions are from whitespace.
            if state == 0 && nbr == 0 {
                self.writeln(&format!("\'{}\' => {},",
                    escape(j as char),
                    "continue"
                ))?;
                j += 1;
                continue;
            }
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

    fn gen_accepts(&self) -> String {
        let mut res = "\t\tlet accepts = [\n".to_string();
        let mut idx = 0;
        while (self.dfa.ncount-idx) / 5 > 0 { 
            for _ in 0..4 {
                res.push_str(&format!("\t\t\t{:>4}, ", self.dfa.accepts[idx]));
                idx += 1;
            }
            res.push_str(&format!("\t\t\t{:>4},\n", self.dfa.accepts[idx]));
            idx += 1;
        }
        if self.dfa.ncount%5 > 0 {
            for _ in 0..(self.dfa.ncount%5 - 1) {
                res.push_str(&format!("\t\t\t{:>4}, ", self.dfa.accepts[idx]));
                idx += 1
            }
            res.push_str(&format!("\t\t\t{:>4}\n", self.dfa.accepts[idx]));
        }
        res.push_str("\t\t];");
        return res;
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

    #[allow(dead_code)]
    fn visualize() {
        let path = "example2.tk";
        let lexer = Lexer::new(path).expect("Invalid Path");
        let mut parser = Parser::new(lexer).expect("File should be non-empty!");
        let nfa = NFA::build_from_matches(&parser.parse().expect("Invalid parse"));
        let dfa = DFA::compress(DFA::subset_construction(nfa));
        let mut gen = Generator::new(&dfa, "tests/tokenizer.rs".to_string())
            .expect("Just Be Better");
        gen.generate().expect("WORK");
    }
}