use crate::token::Literal;
use crate::token::Token::{self, *};
use crate::utils::*;
pub struct Scanner {
    cur: usize,
    start: usize,
    src: String,
    line: usize,
    tokens: Vec<Token>,
}
impl Scanner {
    pub fn scan_tokens(&self, line: &String) -> Vec<Token> {
        while !self.is_end() {
            self.start = self.cur;
            self.scan_token();
        }
        return std::mem::replace(&mut self.tokens, Vec::new());
    }
    pub fn scan_token(&self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                if self.is_match('=') {
                    self.add_token(NotEqual);
                } else {
                    self.add_token(Not);
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(EqualEqual)
                } else {
                    self.add_token(Equal)
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(LessEqual)
                } else {
                    self.add_token(Less)
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(GreaterEqual);
                } else {
                    self.add_token(Greater);
                }
            }
            '/' => {
                if self.is_match('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' | '\n' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    panic!("unexpected line error");
                }
            }
        }
    }
    fn string(&self) {
        let mut s: String = String::new();
        while !self.is_end() && !self.is_match('"') {
            // 换行
            if self.is_match('\n') {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_end() {
            panic!(format!("line: {} Unterminated string.", self.line));
        }
        // "1"a
        // closed '"'
        self.advance();
        let value = self
            .src
            .chars()
            .skip(self.start + 1)
            .take(self.cur - self.start - 2)
            .collect();
        self.add_token(Str(Literal::Str(value)));
    }
    fn number(&self) {
        let mut s: String = String::new();
        while !self.is_end() && is_alpha(self.peek()) {
            s.push(self.peek());
        }
        self.advance();
    }
    fn identifier(&self) {}
    fn is_end(&self) -> bool {
        return self.cur >= self.tokens.len();
    }
    fn advance(&self) -> char {
        let c = self.src.chars().nth(self.cur).unwrap();
        self.cur += 1;
        return c;
    }
    fn peek(&self) -> char {
        return self.src.chars().nth(self.cur).unwrap();
    }
    fn is_match(&self, c: char) -> bool {
        return c == self.peek();
    }
    fn add_token(&self, token: Token) {
        self.tokens.push(token);
    }
    fn peek_next(&self) -> char {}
}
