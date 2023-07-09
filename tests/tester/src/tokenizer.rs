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
	SEMI(String),
	EOF
}
#[derive(Debug, PartialEq, Eq)]
pub struct TokenErr {
   pub error: String
}
pub struct Lexer {
  chars:   Vec<char>,
  pos:     usize,
  begins:  Vec<usize>,
  tabs:    Vec<usize>,
  column:  usize,
  accepts: [usize; 50]
}
impl Lexer {
    pub fn new(fname: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let chars = fs::read_to_string(fname)?
            .chars()
            .collect();
		let accepts = [
			   0, 			   0, 			   0, 			  21, 			  22,
			   8, 			   6, 			   7, 			   9, 			  26,
			  27, 			  14, 			  10, 			  13, 			  25,
			  19, 			  20, 			  25, 			  25, 			  25,
			  25, 			  25, 			  17, 			  18, 			  25,
			  25, 			  23, 			  25, 			  25, 			  25,
			  25, 			   5, 			   1, 			  25, 			   3,
			  25, 			  25, 			  25, 			  25, 			   4,
			  25, 			  25, 			  24, 			  25, 			  25,
			   2, 			  15, 			  11, 			  16, 			  12,
		];
        return Ok(Lexer { 
           chars,
           pos: 0,
           begins: vec![0; 1],
           tabs:   Vec::new(),
           column: 0,
           accepts
        });
    }

   fn advance(&mut self) -> char {
       let c = self.chars[self.pos];
        match c {
           '\n' => {
               self.column = 0;
               self.begins.push(self.pos + 1);
           },
           '\t' => {
               self.tabs.push(self.column);
               self.column += 4 - (self.column % 4);
           }
           _ => self.column += 1
       }
       self.pos += 1;
       return c;
   }
   fn retract(&mut self) {
       self.pos -= 1;
       let c = self.chars[self.pos];
       match c {
           '\n' => {
               self.begins.pop();
               self.column = self.pos - self.begins[self.begins.len() - 1];
           }
           '\t' => {
               self.column = self.tabs.pop().unwrap();
           }
           _ => self.column -= 1
       }
   }
	pub fn next(&mut self) -> Result<Token, TokenErr> {
		if self.pos == self.chars.len() { return Ok(EOF); }
		let mut stk: Vec<usize> = Vec::new();
		let mut chars: Vec<char> = Vec::new();
		let mut state: usize = 0;
		loop {
			if self.pos == self.chars.len() { break; }
			let c = self.advance();
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
					';' => 10,
					'<' => 11,
					'=' => 12,
					'>' => 13,
					'A'..='Z' => 14,
					'[' => 15,
					']' => 16,
					'a'..='d' => 14,
					'e' => 17,
					'f' => 18,
					'g' | 'h' => 14,
					'i' => 19,
					'j'..='r' => 14,
					's' => 20,
					't' => 21,
					'u'..='z' => 14,
					'{' => 22,
					'}' => 23,
					_ => 1
				},
				1 => {
					stk.push(state);
					chars.push(c);
					break;
				}
				2 => match c {
					'=' => 49,
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
					_ => 1
				},
				11 => match c {
					'=' => 48,
					_ => 1
				},
				12 => match c {
					'=' => 47,
					_ => 1
				},
				13 => match c {
					'=' => 46,
					_ => 1
				},
				14 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='z' => 14,
					_ => 1
				},
				15 => match c {
					_ => 1
				},
				16 => match c {
					_ => 1
				},
				17 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='k' => 14,
					'l' => 43,
					'm'..='z' => 14,
					_ => 1
				},
				18 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a' => 35,
					'b'..='k' => 14,
					'l' => 36,
					'm'..='z' => 14,
					_ => 1
				},
				19 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='e' => 14,
					'f' => 32,
					'g'..='m' => 14,
					'n' => 33,
					'o'..='z' => 14,
					_ => 1
				},
				20 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='s' => 14,
					't' => 27,
					'u'..='z' => 14,
					_ => 1
				},
				21 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='q' => 14,
					'r' => 24,
					's'..='z' => 14,
					_ => 1
				},
				22 => match c {
					_ => 1
				},
				23 => match c {
					_ => 1
				},
				24 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='t' => 14,
					'u' => 25,
					'v'..='z' => 14,
					_ => 1
				},
				25 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='d' => 14,
					'e' => 26,
					'f'..='z' => 14,
					_ => 1
				},
				26 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='z' => 14,
					_ => 1
				},
				27 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='q' => 14,
					'r' => 28,
					's'..='z' => 14,
					_ => 1
				},
				28 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='h' => 14,
					'i' => 29,
					'j'..='z' => 14,
					_ => 1
				},
				29 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='m' => 14,
					'n' => 30,
					'o'..='z' => 14,
					_ => 1
				},
				30 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='f' => 14,
					'g' => 31,
					'h'..='z' => 14,
					_ => 1
				},
				31 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='z' => 14,
					_ => 1
				},
				32 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='z' => 14,
					_ => 1
				},
				33 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='s' => 14,
					't' => 34,
					'u'..='z' => 14,
					_ => 1
				},
				34 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='z' => 14,
					_ => 1
				},
				35 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='k' => 14,
					'l' => 40,
					'm'..='z' => 14,
					_ => 1
				},
				36 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='n' => 14,
					'o' => 37,
					'p'..='z' => 14,
					_ => 1
				},
				37 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a' => 38,
					'b'..='z' => 14,
					_ => 1
				},
				38 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='s' => 14,
					't' => 39,
					'u'..='z' => 14,
					_ => 1
				},
				39 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='z' => 14,
					_ => 1
				},
				40 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='r' => 14,
					's' => 41,
					't'..='z' => 14,
					_ => 1
				},
				41 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='d' => 14,
					'e' => 42,
					'f'..='z' => 14,
					_ => 1
				},
				42 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='z' => 14,
					_ => 1
				},
				43 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='r' => 14,
					's' => 44,
					't'..='z' => 14,
					_ => 1
				},
				44 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='d' => 14,
					'e' => 45,
					'f'..='z' => 14,
					_ => 1
				},
				45 => match c {
					'0'..='9' => 14,
					'A'..='Z' => 14,
					'a'..='z' => 14,
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
				49 => match c {
					_ => 1
				},
				_ => panic!("Invalid State!")
			};
			stk.push(state);
			chars.push(c);
		}
		while stk.len() > 0 &&
		   self.accepts[stk[stk.len() - 1]] == 0 {
		   stk.pop().unwrap();
		   chars.pop().unwrap();
		   self.retract();
		}
		if stk.len() == 0 {
		    let start = self.begins[self.begins.len() - 1];
		    let error_line: String = self.chars[start..]
		        .iter()
		        .take_while(|&&c| c != '\n')
		        .collect();
		    return Err(TokenErr{error: format!(
		        "Failed to lex from: \n{}\n{}^",
		        error_line,
		        " ".repeat(self.column)
		    )});
		}
		let word : String = chars.iter().collect();
		match self.accepts[stk[stk.len() - 1]] {
			1    => return Ok(IF(word)),
			2    => return Ok(ELSE(word)),
			3    => return Ok(INT(word)),
			4    => return Ok(FLOAT(word)),
			5    => return Ok(STRING(word)),
			6    => return Ok(PLUS(word)),
			7    => return Ok(MINUS(word)),
			8    => return Ok(MULTIPLY(word)),
			9    => return Ok(DIVIDE(word)),
			10   => return Ok(ASSIGN(word)),
			11   => return Ok(EQUALS(word)),
			12   => return Ok(NOT_EQUALS(word)),
			13   => return Ok(GREATER_THAN(word)),
			14   => return Ok(LESS_THAN(word)),
			15   => return Ok(GREATER_THAN_OR_EQUAL(word)),
			16   => return Ok(LESS_THAN_OR_EQUAL(word)),
			17   => return Ok(LBRACE(word)),
			18   => return Ok(RBRACE(word)),
			19   => return Ok(LBRACKET(word)),
			20   => return Ok(RBRACKET(word)),
			21   => return Ok(LPAR(word)),
			22   => return Ok(RPAR(word)),
			23   => return Ok(TRUE(word)),
			24   => return Ok(FALSE(word)),
			25   => return Ok(IDENT(word)),
			26   => return Ok(NUMBER(word)),
			27   => return Ok(SEMI(word)),
			_    => panic!("Invalid Accepting State")
		}
	}
}
