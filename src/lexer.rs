use core::fmt;
use std::{collections::HashMap, rc::Rc};

use color_eyre::eyre::{self, Ok, eyre};


#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Plus, PlusEqual, Minus, MinusEqual,
    Asterisk, AsteriskAsterisk, AsteriskEqual, 
    Slash, SlashEqual, Modulus, ModulusEqual, 
    At, 

    Equal, EqualEqual, Bang, BangEqual, And,
    Or, Less, LessEqual, Greater, GreaterEqual,

    Dot, DotDot, Comma, LeftParen, RightParen, LeftBracket, RightBracket,
    LeftBrace, RightBrace, Underscore, Colon, ScopeOperator, RightArrow,

    Float, Integer, True, False,
    String, FormatString,
    Identifier,

    Enum, EndEnum, Struct, EndStruct, Def, EndDef,
    If, EndIf, Else, For, Do, Done, While, Return,
    Break, Continue,

    Eof
}


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl TokenType {
    pub fn is_binary_operator(token: &PieToken) -> bool {
        use TokenType::*;

        vec![
            And, Or, 
            Plus, Minus, Asterisk, AsteriskAsterisk,
            PlusEqual, MinusEqual, AsteriskEqual,
            Modulus, ModulusEqual,
            Slash, SlashEqual, Equal,
            EqualEqual, BangEqual, Less, Greater,
            LessEqual, GreaterEqual
        ].contains(&token.type_)
    }


    pub fn precedence_level(&self) -> i8 {
        match self {
            // P0 - Logical not
            TokenType::Bang => 0,

            // P1 - Equality
            TokenType::EqualEqual
            | TokenType::GreaterEqual
            | TokenType::LessEqual
            | TokenType::BangEqual
            | TokenType::Less
            | TokenType::Greater => 1,

            // P2 - Assignment
            TokenType::PlusEqual
            | TokenType::SlashEqual
            | TokenType::ModulusEqual
            | TokenType::Equal
            | TokenType::AsteriskEqual
            | TokenType::MinusEqual => 2,

            // P3 - Addition, Subtraction, Conjunction, Disjunction
            TokenType::Plus
            | TokenType::And
            | TokenType::Or
            | TokenType::Minus => 3,


            // P4 - Multiplication, Division, Modulo
            TokenType::Asterisk
            | TokenType::Modulus
            | TokenType::Slash => 4,

            // P5 - Exponent
            TokenType::AsteriskAsterisk => 5,
            
            // Anything else == invalid
            _ => -1
        }
    }
}


///
/// A struct for tracking token values, seeing as they can be of
/// a variety of types.
/// 
#[derive(Debug, Clone)]
pub enum PieValue {
    FloatLiteral(f64), IntegerLiteral(i64),
    StringLiteral(String), FormatStringLiteral(String)
}


impl fmt::Display for PieValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Value: {{ ")?;
        match self {
            PieValue::FloatLiteral(v) => write!(f, "Float({})", v)?,

            PieValue::IntegerLiteral(v) => write!(f, "Integer({})", v)?,

            PieValue::StringLiteral(v) 
                | PieValue::FormatStringLiteral(v)
                => write!(f, "\"{}\"", v)?,
        };
        write!(f, " }}")
    }
}


///
/// # Overview
/// `PieToken` represents a singular unit with meaning in
/// this language. For example, the following code:
/// ```
/// for i in 0..10 do
///     print("Hello, World!")
/// done
/// ```
/// Is made of the following tokens:
/// ```
/// [for] [name 'i'] [in] [range 0-10] [do] [name 'print'] [LeftParen] [string 'Hello, World!'] [RightParen] [done]
/// ```
/// 
/// # Members
/// - `type_: TokenType`            - The type of the token.
/// - `lexeme: String`              - The actual text representation of the token in the code.
/// - `value: Option<PieValue>`     - The actual in-memory representation of the token, if applicable.
#[derive(Debug)]
pub struct PieToken {
    pub type_: TokenType,
    pub lexeme: String,
    pub value: Option<PieValue>
}


impl fmt::Display for PieToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token\t|\t{} \"{}\" {}", self.type_, self.lexeme, match &self.value {
            Some(val) => val.to_string(),
            None => String::new(),
        })
    }
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


///
/// # Overview
/// A trait for testing against EOF.
/// 
pub trait CanBeEof {
    fn is_eof(&self) -> bool;
}



///
/// # Overview
/// An iterable stream of tokens. Note that after consuming a token,
/// that token is no longer valid, and will be removed from the iterator.
/// 
/// # Members
/// - `tokens: Vec<Rc<PieToken>>` - The raw list of tokens scanned from the source
/// code.
/// 
pub struct PieTokenStream {
    tokens: Vec<Rc<PieToken>>
}


impl CanBeEof for PieTokenStream {
    fn is_eof(&self) -> bool {
        self.is_eof_internal()
    }
}


impl PieTokenStream {
    ///
    /// Convert a vector of tokens into a `PieTokenStream`. Note that
    /// `tokens` is moved inside of the `PieTokenStream` object here.
    /// 
    pub fn from(tokens: Vec<Rc<PieToken>>) -> Self {
        PieTokenStream { 
            tokens: tokens 
        }
    }


    pub fn is_eof_internal(&self) -> bool {
        self.tokens.is_empty()
    }


    ///
    /// Consume and return the next token in the stream if one exists.
    /// 
    pub fn next_token(&mut self) -> Option<Rc<PieToken>> {
        if self.tokens.is_empty() {
            return None;
        }
        let first = self.tokens.remove(0);
        Some(first)
    }


    ///
    /// Get the next token in the stream without advancing the pointer, 
    /// if a next token exists.
    /// 
    pub fn peek(&self) -> Option<Rc<PieToken>> {
        self.tokens.first().cloned()
    }
}



///
/// # Overview
/// 
/// An abstraction around substrings and views, allowing for more readable
/// code. I.e. Rather than
/// ```
/// let lexeme = &source[start..end];
/// ```
/// we can do:
/// ```
/// let lexeme = cursor.capture();
/// ```
/// 
/// # Members
/// - `source: &'a str`  - a reference to the source code.
/// - `left: u128`       - the left pointer, inclusive.
/// - `right: u128`      - the right pointer, exclusive.
/// 
struct Cursor <'a> {
    source: &'a str,
    left: u128,
    right: u128
}


impl <'a> CanBeEof for Cursor <'a> {
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


///
/// Used to lookup keywords based on lexemes.
/// 
struct KeywordLookup {
    keyword_tokens: HashMap<String, TokenType>
}


impl KeywordLookup {
    pub fn create() -> Self {
        let mut map = HashMap::new();
        
        map.insert(String::from("enum"), TokenType::Enum);
        map.insert(String::from("endenum"), TokenType::EndEnum);
        map.insert(String::from("struct"), TokenType::Struct);
        map.insert(String::from("endstruct"), TokenType::EndStruct);
        map.insert(String::from("if"), TokenType::If);
        map.insert(String::from("endif"), TokenType::EndIf);
        map.insert(String::from("else"), TokenType::Else);
        map.insert(String::from("def"), TokenType::Def);
        map.insert(String::from("enddef"), TokenType::EndDef);
        map.insert(String::from("for"), TokenType::For);
        map.insert(String::from("do"), TokenType::Do);
        map.insert(String::from("done"), TokenType::Done);
        map.insert(String::from("while"), TokenType::While);
        map.insert(String::from("true"), TokenType::True);
        map.insert(String::from("false"), TokenType::False);
        map.insert(String::from("return"), TokenType::Return);
        map.insert(String::from("break"), TokenType::Break);
        map.insert(String::from("continue"), TokenType::Continue);

        Self { keyword_tokens: map }
    }
}


///
/// # Overview
/// Takes a scripts source code as input, and outputs the corresponding
/// sequence of tokens. I.e. The `Lexer` struct is a collection of NFAs 
/// that are used to match substrings to tokens.
/// 
/// # Members
/// - `cursor: Cursor<'a>` - The cursor used to track where in the source code the next token 
///                          will be.
/// 
/// - `line` - The current line of the script.
/// 
/// - `column` - The current column on the current line.
/// 
/// - `token_list: Vec<Rc<PieToken>>` - The list of all tokens that have been scanned 
///                                     thus far.
/// 
struct Lexer <'a> {
    cursor: Cursor<'a>,
    line: u64,
    column: u32,
    token_list: Vec<Rc<PieToken>>
}


impl <'a> Lexer <'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer { 
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
        self.column += 1;
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
            ' ' | '\r' | '\t' => {}

            '\n' => {
                self.line += 1;
                self.column = 1;
            },

            // Digraph operators
            '+' if self.match_peek('=') => {
                self.next();
                self.add_token(TokenType::PlusEqual, None)
            },

            '-' if self.match_peek('=') => {
                self.next();
                self.add_token(TokenType::MinusEqual, None)
            },

            '-' if self.match_peek('>') => {
                self.next();
                self.add_token(TokenType::RightArrow, None)
            },

            '*' if self.match_peek('=') => {
                self.next();
                self.add_token(TokenType::AsteriskEqual, None)
            },

            '*' if self.match_peek('*') => {
                self.next();
                self.add_token(TokenType::AsteriskAsterisk, None)
            },

            '/' if self.match_peek('=') => {
                self.next();
                self.add_token(TokenType::SlashEqual, None)
            },

            '%' if self.match_peek('=') => {
                self.next();
                self.add_token(TokenType::ModulusEqual, None)
            },

            '=' if self.match_peek('=') => {
                self.next();
                self.add_token(TokenType::EqualEqual, None)
            },

            '!' if self.match_peek('=') => {
                self.next();
                self.add_token(TokenType::BangEqual, None)
            },

            '<' if self.match_peek('=') => {
                self.next();
                self.add_token(TokenType::LessEqual, None)
            },

            '>' if self.match_peek('=') => {
                self.next();
                self.add_token(TokenType::GreaterEqual, None)
            },

            '&' if self.match_peek('&') => {
                self.next();
                self.add_token(TokenType::And, None)
            },

            '|' if self.match_peek('|') => {
                self.next();
                self.add_token(TokenType::Or, None)
            },

            ':' if self.match_peek(':') => {
                self.next();
                self.add_token(TokenType::ScopeOperator, None)
            },

            '.' if self.match_peek('.') => {
                self.next();
                self.add_token(TokenType::DotDot, None)
            },


            // Single Grapheme Operators
            '+' => self.add_token(TokenType::Plus, None),

            '-' => self.add_token(TokenType::Minus, None),

            '*' => self.add_token(TokenType::Asterisk, None),

            '/' => self.add_token(TokenType::Slash, None),

            '%' => self.add_token(TokenType::Modulus, None),

            /**************************************************
             * Other single character tokens
             **************************************************/
            '@' => self.add_token(TokenType::At, None),

            '!' => self.add_token(TokenType::Bang, None),

            '=' => self.add_token(TokenType::Equal, None),

            '<' => self.add_token(TokenType::Less, None),

            '>' => self.add_token(TokenType::Greater, None),

            '.' => self.add_token(TokenType::Dot, None),

            ',' => self.add_token(TokenType::Comma, None),

            '(' => self.add_token(TokenType::LeftParen, None),

            ')' => self.add_token(TokenType::RightParen, None),

            '[' => self.add_token(TokenType::LeftBracket, None),

            ']' => self.add_token(TokenType::RightBracket, None),

            '{' => self.add_token(TokenType::LeftBrace, None),

            '}' => self.add_token(TokenType::RightBrace, None),

            '_' => self.add_token(TokenType::Underscore, None),

            ':' => self.add_token(TokenType::Colon, None),

            alpha if alpha.is_alphabetic() => self.scan_keyword(),

            digit if digit.is_digit(10) => self.scan_numeric(),

            '\"' => self.scan_string()?,

            _ => return Err(eyre!("Unexpected: {}", ch.unwrap()))
        };

        Ok(())
    }


    fn scan_string(&mut self) -> eyre::Result<()> {
        while self.has_next() && self.peek().unwrap() != '\"' {
            self.next();
        }

        if !self.has_next() {
            return Err(eyre!("Unterminated string at: {}:{}", self.line, self.column));
        }

        self.next(); // Skip terminating quote
        let lexeme = self.cursor.capture();
        let value = PieValue::StringLiteral(lexeme[1..lexeme.len() - 1].to_string());

        let token = PieToken::new(TokenType::String, &lexeme, Some(value));
        self.token_list.push(Rc::new(token));

        Ok(())
    }


    fn scan_numeric(&mut self) {
        while self.has_next() && self.peek().unwrap().is_digit(10) {
            self.next();
        }

        if let Some('.') = self.peek() {
            self.next();
            while self.has_next() && self.peek().unwrap().is_digit(10) {
                self.next();
            }
            let lexeme = self.cursor.capture();
            let float_value = PieValue::FloatLiteral(lexeme.parse().unwrap());
            let token = PieToken::new(TokenType::Float, &lexeme, Some(float_value));
            self.token_list.push(Rc::new(token));
            return;
        }

        let lexeme = self.cursor.capture();
        let int_value = PieValue::IntegerLiteral(lexeme.parse().unwrap());
        let token = PieToken::new(TokenType::Integer, &lexeme, Some(int_value));
        self.token_list.push(Rc::new(token));
    }


    fn scan_keyword(&mut self) {
        while self.has_next() && self.peek().unwrap().is_alphanumeric() {
            self.next();
        }

        let lexeme = self.cursor.capture();
        let keywords = KeywordLookup::create();

        if let Some(kw) = keywords.keyword_tokens.get(&lexeme) {
            self.add_token(kw.clone(), None);
            return;
        }

        self.add_token(TokenType::Identifier, None);
    }


    pub fn tokenize(&mut self) -> eyre::Result<()> {
        while self.peek().is_some() {
            self.scan_token()?;
            self.cursor.advance_left_to_right();
        }

        self.token_list.push(Rc::new(PieToken::new(TokenType::Eof, "", None)));
        Ok(())
    }
}


pub fn scan_all_tokens(source: &str) -> eyre::Result<PieTokenStream> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()?;
    let token_stream = PieTokenStream::from(lexer.token_list);
    Ok(token_stream)
}