#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
	WHILE(String),
	FOR(String),
	LPAR(String),
	RPAR(String),
	IDENT(String),
}
pub struct Lexer {
  chars: Vec<char>,
  pos:   usize
}
impl Lexer {
    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = fs::read_to_string(fname)?
            .chars()
            .collect();
        return Ok(Lexer { chars, pos: 0 });
    }

    fn nextchar(&mut self) -> char {
        self.pos += 1;
        return self.chars[self.pos - 1];
    }
	fn next(&mut self) -> Result<Token, TokenErr> {
		const dead: u32 = 1;
		let mut state: u32 = 0;
		while state != dead {
			match state {
				0 => state = match self.char {
					' ' => 2,
					'(' => 3,
					')' => 4,
					'A'..='Z' => 5,
					'a'..='e' => 6,
					'f' => 7,
					'g'..='v' => 6,
					'w' => 8,
					'x'..='z' => 6,
					_ => 1
				},
				1 => state = match self.char {
					_ => 1
				},
				2 => state = match self.char {
					'\t' => 18,
					_ => 1
				},
				3 => return LPAR,
				4 => return RPAR,
				5 => return IDENT,
				6 => return IDENT,
				7 => return IDENT,
				8 => return IDENT,
				9 => return IDENT,
				10 => return IDENT,
				11 => return IDENT,
				12 => return IDENT,
				13 => return IDENT,
				14 => return IDENT,
				15 => return WHILE,
				16 => return IDENT,
				17 => return FOR,
				18 => state = match self.char {
					'\n' => 19,
					_ => 1
				},
				19 => state = match self.char {
					'\r' => 20,
					_ => 1
				},
			}
		}
	}
}
