use statement::{self, *};
use token::Token::{self, *};
pub struct Parser {
    tokens: Vec<Token>,
    cur: i32,
}
#[derive(Debug)]
enum Error {
    ParseErr,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cur: 0 }
    }
    pub fn is_end(&self) -> bool {
        self.cur >= self.tokens.len()
    }
    pub fn peek(&self) -> &Token {
        &self.tokens[self.cur]
    }
    pub fn parse(&self) -> Vec<Stmt> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.is_end() {
            stmts.push(self.declaration());
        }
        return stmts;
    }
    // todo: err handing 
    fn declaration(&self) -> Stmt {
        
        if self.is_match(&[&Token::Class]) {
            return self.classDeclaration()?;
        } 
        if self.is_match(&[&Token::Fun]) {
            return self.funDeclaration()?;
            
        }
        if self.is_match(&[&Token::Var]) {
            return self.varDeclaration()?;
        }
        return self.statement()?;
        
    }
    fn classDeclaration(&self) -> Result<Stmt,Error> {
        return Ok()
    }
    pub fn is_match(&mut self, tokens: &[&Token]) -> bool {
        for t in tokens {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    pub fn check(&self, token_type: &Token) -> bool {
        self.peek() == token_type
    }
    pub fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.cur += 1;
        }
        return self.previous();
    }
    pub fn consume(&mut self, t: &Token, msg: &str) {
        if self.check(t) {
            self.advance();
            return;
        }
        panic!("{}", msg);
    }

    pub fn previous(&mut self) -> &Token {
        &self.tokens[self.cur - 1]
    }
}
