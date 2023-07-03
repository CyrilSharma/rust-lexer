use std::fs;
use Token::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
	HELLO(String),
	FOR(String),
	LOL(String),
	BEEP(String),
	HUM(String),
	EOF
}
#[derive(Debug, PartialEq, Eq)]
pub enum TokenErr {
   Err
}
pub struct Lexer {
  chars:   Vec<char>,
  pos:     usize,
  accepts: [usize; 23]
}
impl Lexer {
    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = fs::read_to_string(fname)?
            .chars()
            .collect();
		let accepts = [
			0, 			0, 			6, 			6, 			6,
			6, 			0, 			0, 			0, 			0,
			0, 			3, 			0, 			0, 			5,
			0, 			0, 			1, 			0, 			2,
			0, 			0, 			4
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
					'\t' => 2,
					'\n' => 3,
					'\r' => 4,
					' ' => 5,
					'b' => 6,
					'f' => 7,
					'h' => 8,
					'l' => 9,
					_ => 1
				},
				1 => break,
				2 => 0,
				3 => 0,
				4 => 0,
				5 => 0,
				6 => match c {
					'e' => 20,
					_ => 1
				},
				7 => match c {
					'o' => 18,
					_ => 1
				},
				8 => match c {
					'e' => 12,
					'u' => 13,
					_ => 1
				},
				9 => match c {
					'o' => 10,
					_ => 1
				},
				10 => match c {
					'l' => 11,
					_ => 1
				},
				11 => match c {
					_ => 1
				},
				12 => match c {
					'l' => 15,
					_ => 1
				},
				13 => match c {
					'm' => 14,
					_ => 1
				},
				14 => match c {
					_ => 1
				},
				15 => match c {
					'l' => 16,
					_ => 1
				},
				16 => match c {
					'o' => 17,
					_ => 1
				},
				17 => match c {
					_ => 1
				},
				18 => match c {
					'r' => 19,
					_ => 1
				},
				19 => match c {
					_ => 1
				},
				20 => match c {
					'e' => 21,
					_ => 1
				},
				21 => match c {
					'p' => 22,
					_ => 1
				},
				22 => match c {
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
			11   => return Ok(LOL(word)),
			14   => return Ok(HUM(word)),
			17   => return Ok(HELLO(word)),
			19   => return Ok(FOR(word)),
			22   => return Ok(BEEP(word)),
			_    => return Err(TokenErr::Err)
		}
	}
}
