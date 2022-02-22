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
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
        "and" =>  And,
        "class"=>  Class,
        "else"=>  Else,
        "false"=>  False,
        "for"=>  For,
        "fun"=>  Fun,
        "if"=>  If,
        "nil"=>  Nil,
        "or"=>  Or,
        "print"=>  Print,
        "return"=>  Return,
        "super"=>  Super,
        "this"=>  This,
        "true"=>  True,
        "var"=>  Var,
        "while"=>  While,
};
impl Scanner {
    pub fn new(src_str: String) -> Self {
        Scanner {
            cur: 0,
            start: 0,
            line: 0,
            src: src_str,
            tokens: Vec::new(),
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        println!("{},{:?}", self.src, self.is_end());
        while !self.is_end() {
            self.start = self.cur;
            self.scan_token();
        }
        return std::mem::replace(&mut self.tokens, Vec::new());
    }
    pub fn scan_token(&mut self) {
        let c = self.advance();
        println!("cur char: {}", c);
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
            ' ' | '\r' | '\t' => {}
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
    fn string(&mut self) {
        while !self.is_end() && !self.is_match('"') {
            // 换行
            if self.is_match('\n') {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_end() {
            panic!("line: {} Unterminated string.", self.line);
        }
        // "1"a
        // closed '"'
        self.advance();
        let value = self.src.sub_str(self.start + 1, self.cur - self.start - 2);
        self.add_token(Str(Literal::Str(value)));
    }
    fn number(&mut self) {
        while !self.is_end() && is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
        }
        // float
        while is_digit(self.peek()) {
            self.advance();
        }
        let value = self
            .src
            .sub_str(self.start, self.cur - self.start)
            .parse::<f64>();
        println!("number: {:?}", value);
        match value {
            Ok(val) => self.add_token(Number(Literal::Double(val))),
            Err(_) => {
                panic!(
                    "fail to parse {} to f64",
                    self.src.sub_str(self.start + 1, self.cur - self.start - 2)
                );
            }
        }
    }
    fn identifier(&mut self) {
        while is_alpha(self.peek()) {
            self.advance();
        }
        let text: String = self.src.sub_str(self.start, self.cur);
        match KEYWORDS.get(text.as_str()) {
            Some(v) => self.add_token(v.clone()),
            None => self.add_token(Identifier(Literal::Str(text))),
        }
    }
    fn is_end(&self) -> bool {
        return self.cur >= self.src.len();
    }
    fn advance(&mut self) -> char {
        if self.is_end() {
            return '\0';
        }
        let c = self.src.chars().nth(self.cur).unwrap();
        self.cur += 1;
        return c;
    }
    fn peek(&self) -> char {
        if self.is_end() {
            return '\0';
        }
        return self.src.chars().nth(self.cur).unwrap();
    }
    fn is_match(&self, c: char) -> bool {
        return c == self.peek();
    }
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
    fn peek_next(&self) -> char {
        if self.cur + 1 >= self.src.len() {
            return '\0';
        }
        return self.src.chars().nth(self.cur + 1).unwrap();
    }
}
