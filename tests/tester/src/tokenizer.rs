use std::fs;
use Token::*;
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
	IF(String),
	ELSE(String),
	INT(String),
	FLOAT(String),
	STRING(String),
	PLUS(String),
	MINUS(String),
	MULTIPLY(String),
	DIVIDE(String),
	ASSIGN(String),
	EQUALS(String),
	NOT_EQUALS(String),
	GREATER_THAN(String),
	LESS_THAN(String),
	GREATER_THAN_OR_EQUAL(String),
	LESS_THAN_OR_EQUAL(String),
	LBRACE(String),
	RBRACE(String),
	LBRACKET(String),
	RBRACKET(String),
	LPAR(String),
	RPAR(String),
	TRUE(String),
	FALSE(String),
	IDENT(String),
	NUMBER(String),
	EOF
}
#[derive(Debug, PartialEq, Eq)]
pub enum TokenErr {
   Err
}
pub struct Lexer {
  chars:   Vec<char>,
  pos:     usize,
  accepts: [usize; 49]
}
impl Lexer {
    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = fs::read_to_string(fname)?
            .chars()
            .collect();
		let accepts = [
			   0, 			   0, 			   0, 			  21, 			  22,
			   8, 			   6, 			   7, 			   9, 			  26,
			  14, 			  10, 			  13, 			  25, 			  19,
			  20, 			  25, 			  25, 			  25, 			  25,
			  25, 			  17, 			  18, 			  25, 			  25,
			  23, 			  25, 			  25, 			  25, 			  25,
			   5, 			   1, 			  25, 			   3, 			  25,
			  25, 			  25, 			  25, 			   4, 			  25,
			  25, 			  24, 			  25, 			  25, 			   2,
			  15, 			  11, 			  16, 			  12
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
					'!' => 2,
					'(' => 3,
					')' => 4,
					'*' => 5,
					'+' => 6,
					'-' => 7,
					'/' => 8,
					'0'..='9' => 9,
					'<' => 10,
					'=' => 11,
					'>' => 12,
					'A'..='Z' => 13,
					'[' => 14,
					']' => 15,
					'a'..='d' => 13,
					'e' => 16,
					'f' => 17,
					'g' | 'h' => 13,
					'i' => 18,
					'j'..='r' => 13,
					's' => 19,
					't' => 20,
					'u'..='z' => 13,
					'{' => 21,
					'}' => 22,
					_ => 1
				},
				1 => break,
				2 => match c {
					'=' => 48,
					_ => 1
				},
				3 => match c {
					_ => 1
				},
				4 => match c {
					_ => 1
				},
				5 => match c {
					_ => 1
				},
				6 => match c {
					_ => 1
				},
				7 => match c {
					_ => 1
				},
				8 => match c {
					_ => 1
				},
				9 => match c {
					'0'..='9' => 9,
					_ => 1
				},
				10 => match c {
					'=' => 47,
					_ => 1
				},
				11 => match c {
					'=' => 46,
					_ => 1
				},
				12 => match c {
					'=' => 45,
					_ => 1
				},
				13 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='z' => 13,
					_ => 1
				},
				14 => match c {
					_ => 1
				},
				15 => match c {
					_ => 1
				},
				16 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='k' => 13,
					'l' => 42,
					'm'..='z' => 13,
					_ => 1
				},
				17 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a' => 34,
					'b'..='k' => 13,
					'l' => 35,
					'm'..='z' => 13,
					_ => 1
				},
				18 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='e' => 13,
					'f' => 31,
					'g'..='m' => 13,
					'n' => 32,
					'o'..='z' => 13,
					_ => 1
				},
				19 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='s' => 13,
					't' => 26,
					'u'..='z' => 13,
					_ => 1
				},
				20 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='q' => 13,
					'r' => 23,
					's'..='z' => 13,
					_ => 1
				},
				21 => match c {
					_ => 1
				},
				22 => match c {
					_ => 1
				},
				23 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='t' => 13,
					'u' => 24,
					'v'..='z' => 13,
					_ => 1
				},
				24 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='d' => 13,
					'e' => 25,
					'f'..='z' => 13,
					_ => 1
				},
				25 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='z' => 13,
					_ => 1
				},
				26 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='q' => 13,
					'r' => 27,
					's'..='z' => 13,
					_ => 1
				},
				27 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='h' => 13,
					'i' => 28,
					'j'..='z' => 13,
					_ => 1
				},
				28 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='m' => 13,
					'n' => 29,
					'o'..='z' => 13,
					_ => 1
				},
				29 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='f' => 13,
					'g' => 30,
					'h'..='z' => 13,
					_ => 1
				},
				30 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='z' => 13,
					_ => 1
				},
				31 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='z' => 13,
					_ => 1
				},
				32 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='s' => 13,
					't' => 33,
					'u'..='z' => 13,
					_ => 1
				},
				33 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='z' => 13,
					_ => 1
				},
				34 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='k' => 13,
					'l' => 39,
					'm'..='z' => 13,
					_ => 1
				},
				35 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='n' => 13,
					'o' => 36,
					'p'..='z' => 13,
					_ => 1
				},
				36 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a' => 37,
					'b'..='z' => 13,
					_ => 1
				},
				37 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='s' => 13,
					't' => 38,
					'u'..='z' => 13,
					_ => 1
				},
				38 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='z' => 13,
					_ => 1
				},
				39 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='r' => 13,
					's' => 40,
					't'..='z' => 13,
					_ => 1
				},
				40 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='d' => 13,
					'e' => 41,
					'f'..='z' => 13,
					_ => 1
				},
				41 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='z' => 13,
					_ => 1
				},
				42 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='r' => 13,
					's' => 43,
					't'..='z' => 13,
					_ => 1
				},
				43 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='d' => 13,
					'e' => 44,
					'f'..='z' => 13,
					_ => 1
				},
				44 => match c {
					'0'..='9' => 13,
					'A'..='Z' => 13,
					'a'..='z' => 13,
					_ => 1
				},
				45 => match c {
					_ => 1
				},
				46 => match c {
					_ => 1
				},
				47 => match c {
					_ => 1
				},
				48 => match c {
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
			3    => return Ok(LPAR(word)),
			4    => return Ok(RPAR(word)),
			5    => return Ok(MULTIPLY(word)),
			6    => return Ok(PLUS(word)),
			7    => return Ok(MINUS(word)),
			8    => return Ok(DIVIDE(word)),
			9    => return Ok(NUMBER(word)),
			10   => return Ok(LESS_THAN(word)),
			11   => return Ok(ASSIGN(word)),
			12   => return Ok(GREATER_THAN(word)),
			13   => return Ok(IDENT(word)),
			14   => return Ok(LBRACKET(word)),
			15   => return Ok(RBRACKET(word)),
			16   => return Ok(IDENT(word)),
			17   => return Ok(IDENT(word)),
			18   => return Ok(IDENT(word)),
			19   => return Ok(IDENT(word)),
			20   => return Ok(IDENT(word)),
			21   => return Ok(LBRACE(word)),
			22   => return Ok(RBRACE(word)),
			23   => return Ok(IDENT(word)),
			24   => return Ok(IDENT(word)),
			25   => return Ok(TRUE(word)),
			26   => return Ok(IDENT(word)),
			27   => return Ok(IDENT(word)),
			28   => return Ok(IDENT(word)),
			29   => return Ok(IDENT(word)),
			30   => return Ok(STRING(word)),
			31   => return Ok(IF(word)),
			32   => return Ok(IDENT(word)),
			33   => return Ok(INT(word)),
			34   => return Ok(IDENT(word)),
			35   => return Ok(IDENT(word)),
			36   => return Ok(IDENT(word)),
			37   => return Ok(IDENT(word)),
			38   => return Ok(FLOAT(word)),
			39   => return Ok(IDENT(word)),
			40   => return Ok(IDENT(word)),
			41   => return Ok(FALSE(word)),
			42   => return Ok(IDENT(word)),
			43   => return Ok(IDENT(word)),
			44   => return Ok(ELSE(word)),
			45   => return Ok(GREATER_THAN_OR_EQUAL(word)),
			46   => return Ok(EQUALS(word)),
			47   => return Ok(LESS_THAN_OR_EQUAL(word)),
			48   => return Ok(NOT_EQUALS(word)),
			_    => return Err(TokenErr::Err)
		}
	}
}
