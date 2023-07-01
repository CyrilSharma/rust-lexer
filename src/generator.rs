use std::{fs::File, error::Error};
use std::io::Write;
use crate::dfa::{DFA};

struct Generator<'a> { 
    dfa: &'a DFA,
    file: File,
    tabs: String
}

#[allow(dead_code)]
impl<'a> Generator<'a> {
    pub fn new(dfa: &'a DFA) -> Result<Self, Box<dyn Error>> {
        return Ok(Generator { 
            dfa: dfa,
            file: File::create("tokenizer.rs")?,
            tabs: String::new()
        });
    }

    pub fn generate(&mut self) -> Result<(), Box<dyn Error>> {
        writeln!(self.file, "#[derive(Copy, Clone, Debug, PartialEq, Eq)]")?;
        writeln!(self.file, "pub enum Token {{")?;
        for label in &self.dfa.labels {
            writeln!(self.file, "\t{label}(String),")?;
        }
        writeln!(self.file, "}}")?;

        writeln!(self.file,
            "\
            pub struct Lexer {{\n\
                chars: Vec<char>,\n\
                pos: usize\n\
            }}\n\
            impl Lexer {{\n\
                pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {{\n\
                    let chars = fs::read_to_string(fname)?\n\
                        .chars()\n\
                        .collect();\n\
                    return Ok(Lexer {{ chars, pos: 0, enclosed: false }});\n\
                }}\n\
                \n
                fn nextchar(&mut self) -> char {{\n\
                    self.pos += 1;\n\
                    return self.chars[self.pos - 1];\n\
                }}\
            ",
        )?;
        self.tabs.push('\t');
        writeln!(self.file,
            "{}fn next(&mut self) -> Result<Token, TokenErr> {{",
            self.tabs
        )?;
        self.tabs.push('\t');
        self.writeAutomota()?;
        self.tabs.pop();
        writeln!(self.file, "}}")?;
        self.tabs.pop();
        writeln!(self.file, "}}")?;
        return Ok(());
    }

    fn writeAutomota(&mut self) -> Result<(), Box<dyn Error>> {
        writeln!(self.file,
            "{}let mut state = 0;\n\
             {}while let Some(c) = self.cur {{\n\
            ",
            self.tabs, self.tabs,
        )?;
        self.tabs.push('\t');
        writeln!(self.file, "{}match state {{", self.tabs)?;

        self.tabs.push('\t');
        for state in 0..self.dfa.ncount {
            self.writeTransitions(state)?;
        }
        writeln!(self.file, "}}")?;
        return Ok(());
    }

    fn writeTransitions(&mut self, state: usize) -> Result<(), Box<dyn Error>> {
        write!(self.file, "{}{} => ", self.tabs, state)?;
        let accepts = self.dfa.accepts[state];
        if accepts != 0 {
            write!(
                self.file,
                "return {},",
                self.dfa.labels[accepts]
            )?;
            return Ok(());
        }

        writeln!(self.file, "{}state = match self.char {{", self.tabs)?;
        self.tabs.push('\t');
        for mut j in 0..128u8 {
            let start =  j;
            while self.dfa.jumps[state][j as usize] != 0 &&
                j <= 128 { 
                j += 1 
            }
            if start == j {
                writeln!(
                    self.file, 
                    "{}{} => {},",
                    self.tabs, start as char,
                    self.dfa.jumps[state][j as usize]
                )?;
            } else if start + 1 == j {
                writeln!(
                    self.file, 
                    "{}{} | {} => {},",
                    self.tabs, start, j,
                    self.dfa.jumps[state][j as usize]
                )?;
            } else {
                writeln!(
                    self.file, 
                    "{}{}..={} => {},",
                    self.tabs, start, j,
                    self.dfa.jumps[state][j as usize]
                )?;
            }
        }
        writeln!(self.file, "{}_ => return LexError()", self.tabs)?;

        self.tabs.pop();
        writeln!(self.file, "{}}}", self.tabs)?;
        return Ok(());
    }
}

/* TODO - Lexing Testcases... */