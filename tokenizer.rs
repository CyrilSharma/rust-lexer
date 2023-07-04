use std::fs;
use Token::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
	WHILE(String),
	FOR(String),
	LPAR(String),
	RPAR(String),
	IDENT(String),
	EOF
}
#[derive(Debug, PartialEq, Eq)]
pub enum TokenErr {
   Err
}
pub struct Lexer {
  chars:   Vec<char>,
  pos:     usize,
  accepts: [usize; 13]
}
impl Lexer {
    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = fs::read_to_string(fname)?
            .chars()
            .collect();
		let accepts = [
			0, 			0, 			3, 			4, 			5,
			5, 			5, 			5, 			5, 			5,
			1, 			5, 			2
		];
        return Ok(Lexer { chars, pos: 0, accepts });
    }

    fn nextchar(&mut self) -> char {
        self.pos += 1;
        return self.chars[self.pos - 1];
    }
	pub fn next(&mut self) -> Result<Token, TokenErr> {
		if self.pos == self.chars.len() { return Ok(EOF); }
		let mut stk: Vec<usize> = Vec::new();
		let mut chars: Vec<char> = Vec::new();
		let mut state: usize = 0;
		loop {
			if self.pos == self.chars.len() { break; }
			let c = self.nextchar();
			state = match state {
				0 => match c {
					'\t' => continue,
					'\n' => continue,
					'\r' => continue,
					' ' => continue,
					'(' => 2,
					')' => 3,
					'A'..='Z' => 4,
					'a'..='e' => 4,
					'f' => 5,
					'g'..='v' => 4,
					'w' => 6,
					'x'..='z' => 4,
					_ => 1
				},
				1 => break,
				2 => match c {
					_ => 1
				},
				3 => match c {
					_ => 1
				},
				4 => match c {
					'0'..='9' => 4,
					'A'..='Z' => 4,
					'a'..='z' => 4,
					_ => 1
				},
				5 => match c {
					'0'..='9' => 4,
					'A'..='Z' => 4,
					'a'..='n' => 4,
					'o' => 11,
					'p'..='z' => 4,
					_ => 1
				},
				6 => match c {
					'0'..='9' => 4,
					'A'..='Z' => 4,
					'a'..='g' => 4,
					'h' => 7,
					'i'..='z' => 4,
					_ => 1
				},
				7 => match c {
					'0'..='9' => 4,
					'A'..='Z' => 4,
					'a'..='h' => 4,
					'i' => 8,
					'j'..='z' => 4,
					_ => 1
				},
				8 => match c {
					'0'..='9' => 4,
					'A'..='Z' => 4,
					'a'..='k' => 4,
					'l' => 9,
					'm'..='z' => 4,
					_ => 1
				},
				9 => match c {
					'0'..='9' => 4,
					'A'..='Z' => 4,
					'a'..='d' => 4,
					'e' => 10,
					'f'..='z' => 4,
					_ => 1
				},
				10 => match c {
					'0'..='9' => 4,
					'A'..='Z' => 4,
					'a'..='z' => 4,
					_ => 1
				},
				11 => match c {
					'0'..='9' => 4,
					'A'..='Z' => 4,
					'a'..='q' => 4,
					'r' => 12,
					's'..='z' => 4,
					_ => 1
				},
				12 => match c {
					'0'..='9' => 4,
					'A'..='Z' => 4,
					'a'..='z' => 4,
					_ => 1
				},
				_ => return Err(TokenErr::Err)
			};
			stk.push(state);
			chars.push(c);
		}
		while stk.len() > 0 &&
		   self.accepts[stk[stk.len() - 1]] == 0 {
		   stk.pop().unwrap();
		   chars.pop().unwrap();
		   self.pos -= 1;
		}
		if stk.len() == 0 { return Err(TokenErr::Err); }
		let word : String = chars.iter().collect();
		match stk[stk.len() - 1] {
			2    => return Ok(LPAR(word)),
			3    => return Ok(RPAR(word)),
			4    => return Ok(IDENT(word)),
			5    => return Ok(IDENT(word)),
			6    => return Ok(IDENT(word)),
			7    => return Ok(IDENT(word)),
			8    => return Ok(IDENT(word)),
			9    => return Ok(IDENT(word)),
			10   => return Ok(WHILE(word)),
			11   => return Ok(IDENT(word)),
			12   => return Ok(FOR(word)),
			_    => return Err(TokenErr::Err)
		}
	}
}
