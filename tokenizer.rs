use Token::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
	WHILE(String),
	FOR(String),
	LPAR(String),
	RPAR(String),
	IDENT(String),
}
pub enum TokenErr {
   Err
}
pub struct Lexer {
  chars:   Vec<char>,
  pos:     usize,
  accepts: [usize; 21]
}
impl Lexer {
    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = fs::read_to_string(fname)?
            .chars()
            .collect();
		let accepts = [0, 0, 0, 3, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 5, 2, 0, 0, 6];
        return Ok(Lexer { chars, pos: 0, accepts });
    }

    fn nextchar(&mut self) -> char {
        self.pos += 1;
        return self.chars[self.pos - 1];
    }
	fn next(&mut self) -> Result<Token, TokenErr> {
		let mut stk: Vec<usize> = Vec::new();
		const dead: usize = 1;
		let mut state: usize = 0;
		while state != dead {
			let c = self.nextchar();
			state = match state {
				0 => match c {
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
				1 => match c {
					_ => 1
				},
				2 => match c {
					'\t' => 18,
					_ => 1
				},
				3 => match c {
					_ => 1
				},
				4 => match c {
					_ => 1
				},
				5 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='z' => 11,
					_ => 1
				},
				6 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='z' => 11,
					_ => 1
				},
				7 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='n' => 11,
					'o' => 16,
					'p'..='z' => 11,
					_ => 1
				},
				8 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='g' => 11,
					'h' => 12,
					'i'..='z' => 11,
					_ => 1
				},
				9 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='z' => 11,
					_ => 1
				},
				10 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='z' => 11,
					_ => 1
				},
				11 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='z' => 11,
					_ => 1
				},
				12 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='h' => 11,
					'i' => 13,
					'j'..='z' => 11,
					_ => 1
				},
				13 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='k' => 11,
					'l' => 14,
					'm'..='z' => 11,
					_ => 1
				},
				14 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='d' => 11,
					'e' => 15,
					'f'..='z' => 11,
					_ => 1
				},
				15 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='z' => 11,
					_ => 1
				},
				16 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='q' => 11,
					'r' => 17,
					's'..='z' => 11,
					_ => 1
				},
				17 => match c {
					'0'..='9' => 9,
					'A'..='Z' => 10,
					'a'..='z' => 11,
					_ => 1
				},
				18 => match c {
					'\n' => 19,
					_ => 1
				},
				19 => match c {
					'\r' => 20,
					_ => 1
				},
				20 => 0,
			};
			stk.push(state);
		}
		while self.accepts[state] == 0 {
		   state = stk.pop();
		}
		let word : String = stk.iter.collect();
		match self.accepts[state] {
			3 => return LPAR(word),
			4 => return RPAR(word),
			5 => return IDENT(word),
			6 => return IDENT(word),
			7 => return IDENT(word),
			8 => return IDENT(word),
			9 => return IDENT(word),
			10 => return IDENT(word),
			11 => return IDENT(word),
			12 => return IDENT(word),
			13 => return IDENT(word),
			14 => return IDENT(word),
			15 => return WHILE(word),
			16 => return IDENT(word),
			17 => return FOR(word),
			_ => return TokenErr::Err
		}
	}
}
