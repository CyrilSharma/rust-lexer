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
  accepts: [usize; 57]
}
impl Lexer {
    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = fs::read_to_string(fname)?
            .chars()
            .collect();
		let accepts = [
			0, 			0, 			26, 			26, 			26,
			26, 			0, 			21, 			22, 			8,
			6, 			7, 			9, 			26, 			14,
			10, 			13, 			25, 			19, 			20,
			25, 			25, 			25, 			25, 			25,
			25, 			17, 			18, 			25, 			25,
			25, 			25, 			25, 			23, 			25,
			25, 			25, 			25, 			5, 			1,
			25, 			3, 			25, 			25, 			25,
			25, 			4, 			25, 			25, 			24,
			25, 			25, 			2, 			15, 			11,
			16, 			12
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
					'!' => 6,
					'(' => 7,
					')' => 8,
					'*' => 9,
					'+' => 10,
					'-' => 11,
					'/' => 12,
					'0'..='9' => 13,
					'<' => 14,
					'=' => 15,
					'>' => 16,
					'A'..='Z' => 17,
					'[' => 18,
					']' => 19,
					'a'..='d' => 20,
					'e' => 21,
					'f' => 22,
					'g' | 'h' => 20,
					'i' => 23,
					'j'..='r' => 20,
					's' => 24,
					't' => 25,
					'u'..='z' => 20,
					'{' => 26,
					'}' => 27,
					_ => 1
				},
				1 => break,
				2 => match c {
					'\t' => 2,
					'\n' => 3,
					'\r' => 4,
					' ' => 5,
					'!' => 6,
					'(' => 7,
					')' => 8,
					'*' => 9,
					'+' => 10,
					'-' => 11,
					'/' => 12,
					'0'..='9' => 13,
					'<' => 14,
					'=' => 15,
					'>' => 16,
					'A'..='Z' => 17,
					'[' => 18,
					']' => 19,
					'a'..='d' => 20,
					'e' => 21,
					'f' => 22,
					'g' | 'h' => 20,
					'i' => 23,
					'j'..='r' => 20,
					's' => 24,
					't' => 25,
					'u'..='z' => 20,
					'{' => 26,
					'}' => 27,
					_ => 1
				},
				3 => match c {
					'\t' => 2,
					'\n' => 3,
					'\r' => 4,
					' ' => 5,
					'!' => 6,
					'(' => 7,
					')' => 8,
					'*' => 9,
					'+' => 10,
					'-' => 11,
					'/' => 12,
					'0'..='9' => 13,
					'<' => 14,
					'=' => 15,
					'>' => 16,
					'A'..='Z' => 17,
					'[' => 18,
					']' => 19,
					'a'..='d' => 20,
					'e' => 21,
					'f' => 22,
					'g' | 'h' => 20,
					'i' => 23,
					'j'..='r' => 20,
					's' => 24,
					't' => 25,
					'u'..='z' => 20,
					'{' => 26,
					'}' => 27,
					_ => 1
				},
				4 => match c {
					'\t' => 2,
					'\n' => 3,
					'\r' => 4,
					' ' => 5,
					'!' => 6,
					'(' => 7,
					')' => 8,
					'*' => 9,
					'+' => 10,
					'-' => 11,
					'/' => 12,
					'0'..='9' => 13,
					'<' => 14,
					'=' => 15,
					'>' => 16,
					'A'..='Z' => 17,
					'[' => 18,
					']' => 19,
					'a'..='d' => 20,
					'e' => 21,
					'f' => 22,
					'g' | 'h' => 20,
					'i' => 23,
					'j'..='r' => 20,
					's' => 24,
					't' => 25,
					'u'..='z' => 20,
					'{' => 26,
					'}' => 27,
					_ => 1
				},
				5 => match c {
					'\t' => 2,
					'\n' => 3,
					'\r' => 4,
					' ' => 5,
					'!' => 6,
					'(' => 7,
					')' => 8,
					'*' => 9,
					'+' => 10,
					'-' => 11,
					'/' => 12,
					'0'..='9' => 13,
					'<' => 14,
					'=' => 15,
					'>' => 16,
					'A'..='Z' => 17,
					'[' => 18,
					']' => 19,
					'a'..='d' => 20,
					'e' => 21,
					'f' => 22,
					'g' | 'h' => 20,
					'i' => 23,
					'j'..='r' => 20,
					's' => 24,
					't' => 25,
					'u'..='z' => 20,
					'{' => 26,
					'}' => 27,
					_ => 1
				},
				6 => match c {
					'=' => 56,
					_ => 1
				},
				7 => match c {
					_ => 1
				},
				8 => match c {
					_ => 1
				},
				9 => match c {
					_ => 1
				},
				10 => match c {
					_ => 1
				},
				11 => match c {
					_ => 1
				},
				12 => match c {
					_ => 1
				},
				13 => match c {
					'0'..='9' => 13,
					_ => 1
				},
				14 => match c {
					'=' => 55,
					_ => 1
				},
				15 => match c {
					'=' => 54,
					_ => 1
				},
				16 => match c {
					'=' => 53,
					_ => 1
				},
				17 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				18 => match c {
					_ => 1
				},
				19 => match c {
					_ => 1
				},
				20 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				21 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='k' => 30,
					'l' => 50,
					'm'..='z' => 30,
					_ => 1
				},
				22 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a' => 42,
					'b'..='k' => 30,
					'l' => 43,
					'm'..='z' => 30,
					_ => 1
				},
				23 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='e' => 30,
					'f' => 39,
					'g'..='m' => 30,
					'n' => 40,
					'o'..='z' => 30,
					_ => 1
				},
				24 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='s' => 30,
					't' => 34,
					'u'..='z' => 30,
					_ => 1
				},
				25 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='q' => 30,
					'r' => 31,
					's'..='z' => 30,
					_ => 1
				},
				26 => match c {
					_ => 1
				},
				27 => match c {
					_ => 1
				},
				28 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				29 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				30 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				31 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='t' => 30,
					'u' => 32,
					'v'..='z' => 30,
					_ => 1
				},
				32 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='d' => 30,
					'e' => 33,
					'f'..='z' => 30,
					_ => 1
				},
				33 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				34 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='q' => 30,
					'r' => 35,
					's'..='z' => 30,
					_ => 1
				},
				35 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='h' => 30,
					'i' => 36,
					'j'..='z' => 30,
					_ => 1
				},
				36 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='m' => 30,
					'n' => 37,
					'o'..='z' => 30,
					_ => 1
				},
				37 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='f' => 30,
					'g' => 38,
					'h'..='z' => 30,
					_ => 1
				},
				38 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				39 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				40 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='s' => 30,
					't' => 41,
					'u'..='z' => 30,
					_ => 1
				},
				41 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				42 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='k' => 30,
					'l' => 47,
					'm'..='z' => 30,
					_ => 1
				},
				43 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='n' => 30,
					'o' => 44,
					'p'..='z' => 30,
					_ => 1
				},
				44 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a' => 45,
					'b'..='z' => 30,
					_ => 1
				},
				45 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='s' => 30,
					't' => 46,
					'u'..='z' => 30,
					_ => 1
				},
				46 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				47 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='r' => 30,
					's' => 48,
					't'..='z' => 30,
					_ => 1
				},
				48 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='d' => 30,
					'e' => 49,
					'f'..='z' => 30,
					_ => 1
				},
				49 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				50 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='r' => 30,
					's' => 51,
					't'..='z' => 30,
					_ => 1
				},
				51 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='d' => 30,
					'e' => 52,
					'f'..='z' => 30,
					_ => 1
				},
				52 => match c {
					'0'..='9' => 28,
					'A'..='Z' => 29,
					'a'..='z' => 30,
					_ => 1
				},
				53 => match c {
					_ => 1
				},
				54 => match c {
					_ => 1
				},
				55 => match c {
					_ => 1
				},
				56 => match c {
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
			2    => return Ok(NUMBER(word)),
			3    => return Ok(NUMBER(word)),
			4    => return Ok(NUMBER(word)),
			5    => return Ok(NUMBER(word)),
			7    => return Ok(LPAR(word)),
			8    => return Ok(RPAR(word)),
			9    => return Ok(MULTIPLY(word)),
			10   => return Ok(PLUS(word)),
			11   => return Ok(MINUS(word)),
			12   => return Ok(DIVIDE(word)),
			13   => return Ok(NUMBER(word)),
			14   => return Ok(LESS_THAN(word)),
			15   => return Ok(ASSIGN(word)),
			16   => return Ok(GREATER_THAN(word)),
			17   => return Ok(IDENT(word)),
			18   => return Ok(LBRACKET(word)),
			19   => return Ok(RBRACKET(word)),
			20   => return Ok(IDENT(word)),
			21   => return Ok(IDENT(word)),
			22   => return Ok(IDENT(word)),
			23   => return Ok(IDENT(word)),
			24   => return Ok(IDENT(word)),
			25   => return Ok(IDENT(word)),
			26   => return Ok(LBRACE(word)),
			27   => return Ok(RBRACE(word)),
			28   => return Ok(IDENT(word)),
			29   => return Ok(IDENT(word)),
			30   => return Ok(IDENT(word)),
			31   => return Ok(IDENT(word)),
			32   => return Ok(IDENT(word)),
			33   => return Ok(TRUE(word)),
			34   => return Ok(IDENT(word)),
			35   => return Ok(IDENT(word)),
			36   => return Ok(IDENT(word)),
			37   => return Ok(IDENT(word)),
			38   => return Ok(STRING(word)),
			39   => return Ok(IF(word)),
			40   => return Ok(IDENT(word)),
			41   => return Ok(INT(word)),
			42   => return Ok(IDENT(word)),
			43   => return Ok(IDENT(word)),
			44   => return Ok(IDENT(word)),
			45   => return Ok(IDENT(word)),
			46   => return Ok(FLOAT(word)),
			47   => return Ok(IDENT(word)),
			48   => return Ok(IDENT(word)),
			49   => return Ok(FALSE(word)),
			50   => return Ok(IDENT(word)),
			51   => return Ok(IDENT(word)),
			52   => return Ok(ELSE(word)),
			53   => return Ok(GREATER_THAN_OR_EQUAL(word)),
			54   => return Ok(EQUALS(word)),
			55   => return Ok(LESS_THAN_OR_EQUAL(word)),
			56   => return Ok(NOT_EQUALS(word)),
			_    => return Err(TokenErr::Err)
		}
	}
}
