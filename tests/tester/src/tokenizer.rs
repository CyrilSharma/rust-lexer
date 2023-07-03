use std::fs;
use Token::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
	SUNSHINE(String),
	MOON(String),
	JOY(String),
	HAPPY(String),
	TRAVEL(String),
	EXPLORE(String),
	EOF
}
#[derive(Debug, PartialEq, Eq)]
pub enum TokenErr {
   Err
}
pub struct Lexer {
  chars:   Vec<char>,
  pos:     usize,
  accepts: [usize; 39]
}
impl Lexer {
    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = fs::read_to_string(fname)?
            .chars()
            .collect();
		let accepts = [
			0, 			0, 			7, 			7, 			7,
			7, 			0, 			0, 			0, 			0,
			0, 			0, 			0, 			0, 			0,
			0, 			5, 			0, 			0, 			0,
			0, 			0, 			0, 			1, 			0,
			0, 			2, 			0, 			3, 			0,
			0, 			0, 			4, 			0, 			0,
			0, 			0, 			0, 			6
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
					'e' => 6,
					'h' => 7,
					'j' => 8,
					'm' => 9,
					's' => 10,
					't' => 11,
					_ => 1
				},
				1 => break,
				2 => { self.pos -= 1; state = 0; continue; },
				3 => { self.pos -= 1; state = 0; continue; },
				4 => { self.pos -= 1; state = 0; continue; },
				5 => { self.pos -= 1; state = 0; continue; },
				6 => match c {
					'x' => 33,
					_ => 1
				},
				7 => match c {
					'a' => 29,
					_ => 1
				},
				8 => match c {
					'o' => 27,
					_ => 1
				},
				9 => match c {
					'o' => 24,
					_ => 1
				},
				10 => match c {
					'u' => 17,
					_ => 1
				},
				11 => match c {
					'r' => 12,
					_ => 1
				},
				12 => match c {
					'a' => 13,
					_ => 1
				},
				13 => match c {
					'v' => 14,
					_ => 1
				},
				14 => match c {
					'e' => 15,
					_ => 1
				},
				15 => match c {
					'l' => 16,
					_ => 1
				},
				16 => match c {
					_ => 1
				},
				17 => match c {
					'n' => 18,
					_ => 1
				},
				18 => match c {
					's' => 19,
					_ => 1
				},
				19 => match c {
					'h' => 20,
					_ => 1
				},
				20 => match c {
					'i' => 21,
					_ => 1
				},
				21 => match c {
					'n' => 22,
					_ => 1
				},
				22 => match c {
					'e' => 23,
					_ => 1
				},
				23 => match c {
					_ => 1
				},
				24 => match c {
					'o' => 25,
					_ => 1
				},
				25 => match c {
					'n' => 26,
					_ => 1
				},
				26 => match c {
					_ => 1
				},
				27 => match c {
					'y' => 28,
					_ => 1
				},
				28 => match c {
					_ => 1
				},
				29 => match c {
					'p' => 30,
					_ => 1
				},
				30 => match c {
					'p' => 31,
					_ => 1
				},
				31 => match c {
					'y' => 32,
					_ => 1
				},
				32 => match c {
					_ => 1
				},
				33 => match c {
					'p' => 34,
					_ => 1
				},
				34 => match c {
					'l' => 35,
					_ => 1
				},
				35 => match c {
					'o' => 36,
					_ => 1
				},
				36 => match c {
					'r' => 37,
					_ => 1
				},
				37 => match c {
					'e' => 38,
					_ => 1
				},
				38 => match c {
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
			16   => return Ok(TRAVEL(word)),
			23   => return Ok(SUNSHINE(word)),
			26   => return Ok(MOON(word)),
			28   => return Ok(JOY(word)),
			32   => return Ok(HAPPY(word)),
			38   => return Ok(EXPLORE(word)),
			_    => return Err(TokenErr::Err)
		}
	}
}
