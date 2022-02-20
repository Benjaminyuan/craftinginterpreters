use crate::token::TOKEN::{self,*};

pub struct Scanner {
    cur: usize,
    start: usize,
    src: String,
    tokens: Vec<TOKEN>,
}
impl Scanner {
    pub fn scan_tokens(&self, line: &String) -> Vec<TOKEN> {
        while !self.is_end() {

        }
        return std::mem::replace(&mut self.tokens, Vec::new());
    }
    pub fn scan_token(&self)  {
        let c = self.advance();
         match c {
             '('=> { self.add_token(LeftParen)},
             ')'=> { self.add_token(RightParen)},
             '{'=> { self.add_token(LeftBrace)},
             '}'=> { self.add_token(RIGHT_BRACE)},
             ','=> {self.add_token(COMMA)},
             '.'=> {self.add_token(DOT)},
             '-'=> {self.add_token(MINUS)},
             '+'=> {self.add_token(PLUS)},
             ';'=> {self.add_token(SEMICOLON)},
             '*'=> {self.add_token(STAR)}, 
             '!'=> {  
                 if self.is_match('=') { 
                     self.add_token(NOT_EQUAL); 
                }  else { 
                    self.add_token(NOT);
                }
            },
              '='=>{  
                  if self.is_match('=') { 
                    self.add_token(EQUAL_EQUAL)
                    } else { 
                        self.add_token(EQUAL)
                    }
                },
             '<'=> { return if match('=') { LESS_EQUAL} else { LESS}} ,
             '>'=> {return if match('=') { GREATER_EQUAL} else { GREATER} },
 
             '/'=> { 
                if match('/') {
                    // A comment goes until the end of the line.
                    while peek() != '\n' && !isAtEnd() {
                        advance();
                    } 
                } else {
                    addToken(SLASH);
                }
            },
            ' ' | '\r' | '\t' | '\n' | => {}
              // Ignore whitespace.
      
             '\n'=>{ line++}
      //< whitespace
      //> string-start
      
             '"'=> {string()},
      //< string-start
      //> char-error
      
            _ =>{
                if (isDigit(c)) {
                    number();
          //> identifier-start
                  } else if (isAlpha(c)) {
                    identifier();
          //< identifier-start
                  } else {
                    Lox.error(line, "Unexpected character.");
                  }
            }
          

        }
    }
    fn is_end(&self) -> bool {
        return self.cur == self.tokens.len();
    }
    fn advance(&self) -> char {
        let c = self.src.chars().nth(self.cur).unwrap();
        self.cur += 1;
        return c;
    }
    fn peek(&self) -> char {
        return self.src.chars().nth(self.cur).unwrap();
    }
    fn is_match(&self,c : char) -> bool {
        return c == self.peek()
    }
    fn add_token(&self, token TOKEN) {
        self.tokens.push(token);
    }
}
