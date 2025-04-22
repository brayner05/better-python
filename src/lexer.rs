use core::fmt;
use std::{fmt::write, ops::Range, rc::Rc};

use color_eyre::eyre::{self, Error, Ok, eyre};



pub enum TokenType {
    Plus, PlusEqual, Minus, MinusEqual,
    Asterisk, AsteriskAsterisk, AsteriskEqual, 
    Slash, SlashEqual, Modulus, ModulusEqual, 
    At, 

    Equal, EqualEqual, Bang, BangEqual, And,
    Or, Less, LessEqual, Greater, GreaterEqual,

    Dot, Comma, LeftParen, RightParen, LeftBracket, RightBracket,
    LeftBrace, RightBrace, Underscore, Colon, ScopeOperator, RightArrow,

    Float(f64), Integer(f64), Boolean(bool),
    String(String), FormatString(String),
    Identifier,

    Enum, EndEnum, Struct, EndStruct, Def, EndDef,
    If, EndIf, Else, For, Do, Done, While, Return,
    Break, Continue,

    Eof
}


#[derive(Debug, Clone)]
pub enum PieValue {
    FloatLiteral(f32), IntegerLiteral(i32),
    BoolLiteral(bool), StringLiteral(String),
    FormatStringLiteral(String)
}


impl fmt::Display for PieValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Value: {{ ")?;
        match self {
            PieValue::FloatLiteral(v) => write!(f, "Float({})", v)?,

            PieValue::IntegerLiteral(v) => write!(f, "Integer({})", v)?,

            PieValue::BoolLiteral(v) => write!(f, "{}", v)?,

            PieValue::StringLiteral(v) 
                | PieValue::FormatStringLiteral(v)
                => write!(f, "\"{}\"", v)?,
        };
        write!(f, " }}")
    }
}


pub struct PieToken {
    type_: TokenType,
    lexeme: String,
    value: Option<PieValue>
}


impl PieToken {
    pub fn new(type_: TokenType, lexeme: &str, value: Option<PieValue>) -> Self {
        PieToken { 
            type_: type_, 
            lexeme: lexeme.to_string(), 
            value: value
        }
    }
}


pub trait CanBeEOF {
    fn is_eof(&self) -> bool;
}


pub struct PieTokenStream {
    tokens: Vec<Rc<PieToken>>
}


impl CanBeEOF for PieTokenStream {
    fn is_eof(&self) -> bool {
        self.is_eof_internal()
    }
}


impl PieTokenStream {
    pub fn from(tokens: Vec<Rc<PieToken>>) -> Self {
        PieTokenStream { 
            tokens: tokens 
        }
    }


    pub fn is_eof_internal(&self) -> bool {
        self.tokens.is_empty()
    }


    pub fn next_token(&mut self) -> Option<Rc<PieToken>> {
        if self.tokens.is_empty() {
            return None;
        }
        let first = self.tokens.remove(0);
        Some(first)
    }
}


struct Cursor <'a> {
    source: &'a str,
    left: u128,
    right: u128
}


impl <'a> CanBeEOF for Cursor <'a> {
    fn is_eof(&self) -> bool {
        self.is_eof_internal()
    }
}


impl <'a> Cursor <'a> {
    pub fn new(source: &'a str) -> Self {
        Cursor { 
            source: source, 
            left: 0, 
            right: 0 
        }
    }


    pub fn advance_left(&mut self) {
        self.left += 1
    }


    pub fn advance_left_to_right(&mut self) {
        self.left = self.right
    }


    pub fn advance_right(&mut self) {
        self.right += 1
    }


    pub fn current(&self) -> Option<char> {
        self.source.chars().nth(self.right as usize)
    }


    pub fn is_eof_internal(&self) -> bool {
        (self.right as usize) >= self.source.len()
    }


    pub fn capture(&self) -> String {
        let (begin, end) = (self.left as usize, self.right as usize);
        let lexeme = &self.source[begin..end];
        lexeme.to_string()
    }
}


// Private
struct Lexer <'a> {
    source: &'a str,
    cursor: Cursor<'a>,
    line: u64,
    column: u32,
    token_list: Vec<Rc<PieToken>>
}


impl <'a> Lexer <'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer { 
            source: source, 
            cursor: Cursor::new(source), 
            line: 0, 
            column: 0,
            token_list: vec![]
        }
    }


    fn has_next(&self) -> bool {
        !self.cursor.is_eof()
    }


    fn peek(&self) -> Option<char> {
        self.cursor.current()
    }


    fn match_peek(&self, to_match: char) -> bool {
        match self.peek() {
            Some(ch) => to_match == ch,
            None => false,
        }
    }


    fn next(&mut self) -> Option<char> {
        let next_char = self.cursor.current();
        self.cursor.advance_right();
        next_char
    }


    fn add_token(&mut self, type_: TokenType, value: Option<PieValue>) {
        let lexeme = self.cursor.capture();
        let token = PieToken::new(type_, &lexeme, value);
        self.token_list.push(Rc::new(token));
    }


    fn scan_token(&mut self) -> eyre::Result<()> {
        let ch = self.next();
        if ch.is_none() {
            return Err(eyre!("Reached the end of the input file."))
        }
        match ch.unwrap() {
            // Digraph operators
            '+' if self.match_peek('=') 
                => self.add_token(TokenType::PlusEqual, None),

            '-' if self.match_peek('=')
                => self.add_token(TokenType::MinusEqual, None),

            '*' if self.match_peek('=')
                => self.add_token(TokenType::AsteriskEqual, None),

            '*' if self.match_peek('*')
                => self.add_token(TokenType::AsteriskAsterisk, None),

            '/' if self.match_peek('=')
                => self.add_token(TokenType::SlashEqual, None),

            '%' if self.match_peek('=')
                => self.add_token(TokenType::ModulusEqual, None),

            '=' if self.match_peek('=')
                => self.add_token(TokenType::EqualEqual, None),

            '!' if self.match_peek('=')
                => self.add_token(TokenType::BangEqual, None),

            // Single Grapheme Operators
            '+' => self.add_token(TokenType::Plus, None),

            '-' => self.add_token(TokenType::Minus, None),

            _ => return Err(eyre!("Unexpected: {}", ch.unwrap()))
        };

        Ok(())
    }


    pub fn tokenize(&mut self) -> eyre::Result<()> {
        while let Some(ch) = self.peek() {
            self.scan_token()?;
            self.cursor.advance_left_to_right();
        }
        Ok(())
    }
}


pub fn scan_all_tokens(source: &str) -> eyre::Result<PieTokenStream> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()?;
    let token_stream = PieTokenStream::from(lexer.token_list);
    Ok(token_stream)
}