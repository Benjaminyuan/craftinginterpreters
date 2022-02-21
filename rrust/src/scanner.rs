use crate::token::Literal;
use crate::token::Token::{self, *};
use crate::utils::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
pub struct Scanner {
    cur: usize,
    start: usize,
    src: String,
    line: usize,
    tokens: Vec<Token>,
}

lazy_static! {
    static ref keywords: HashMap<&'static str, &'static Token> = {
        let mut map = HashMap::new();
        map.insert("and", And);
        map.insert("class", Class);
        map.insert("else", Else);
        map.insert("false", False);
        map.insert("for", For);
        map.insert("fun", Fun);
        map.insert("if", If);
        map.insert("nil", Nil);
        map.insert("or", Or);
        map.insert("print", Print);
        map.insert("return", Return);
        map.insert("super", Super);
        map.insert("this", This);
        map.insert("true", True);
        map.insert("var", Var);
        map.insert("while", While);
    };
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
        let value = self.src.sub_str(self.start + 1, self.cur - self.start - 2);
        self.add_token(Str(Literal::Str(value)));
    }
    fn number(&self) {
        while !self.is_end() && is_alpha(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
        }
        // float
        while is_digit(self.advance()) {
            self.advance();
        }
        let value = self
            .src
            .sub_str(self.start, self.cur - self.start)
            .parse::<f64>();
        match value {
            Ok(val) => self.add_token(Number(Literal::Double(val))),
            Err(_) => {
                panic!(format!(format!(
                    "fail to parse {} to f64",
                    self.src.sub_str(self.start + 1, self.cur - self.start - 2)
                )));
            }
        }
    }
    fn identifier(&self) {
        while is_alpha(self.peek()) {
            self.advance();
        }
        String text = self.src.sub_str(self.start,self.cur);
        keywords.
    }
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
    fn peek_next(&self) -> char {
        if self.cur + 1 >= self.src.len() {
            return '\0';
        }
        return self.src.chars().nth(self.cur + 1).unwrap();
    }
}
