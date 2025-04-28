use core::fmt;
use std::rc::Rc;

use color_eyre::eyre::{self, eyre};

use crate::lexer::{self, CanBeEof, PieToken, PieTokenStream, PieValue, TokenType};


#[derive(Debug)]
pub enum AstNode {
    UnaryOperation { operator: UnaryOperator, operand: Rc<AstNode> },
    BinaryOperation { operator: BinaryOperator, lhs: Rc<AstNode>, rhs: Rc<AstNode> },

    IntegerLiteral(i64), FloatLiteral(f64), BooleanLiteral(bool),
    StringLiteral(String), Identifier(String)
}


impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstNode::UnaryOperation { 
                operator, 
                operand 
            } => write!(f, "({} {})", operator, operand.as_ref()),

            AstNode::BinaryOperation { 
                operator, 
                lhs, 
                rhs 
            } => write!(f, "({}, {}, {})", operator, lhs.as_ref(), rhs.as_ref()),

            _ => write!(f, "{:?}", self)
        }
    }
}


#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Minus, LogicalNot
}


impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus, Minus, Asterisk, Slash,
    PlusEqual, MinusEqual, AsteriskEqual, SlashEqual,
    Equal, EqualEqual, BangEqual, Less, Greater,
    LessEqual, GreaterEqual,
    Or, And
}


impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


struct Parser <'a> {
    token_stream: &'a mut PieTokenStream,
    position: usize
}


impl <'a> Parser <'a> {
    pub fn new(token_stream: &'a mut PieTokenStream) -> Self {
        Self {
            token_stream: token_stream,
            position: 0
        }
    }


    fn has_next(&self) -> bool {
        !self.token_stream.is_eof()
    }


    fn next_token(&mut self) -> Rc<PieToken> {
        let token = self.token_stream.next_token();
        token.unwrap()
    }


    fn peek(&self) -> Rc<PieToken> {
        self.token_stream.peek().unwrap()
    }


    fn parse_program(&mut self) -> eyre::Result<Rc<AstNode>> {
        self.parse_statement()
    }


    fn parse_statement(&mut self) -> eyre::Result<Rc<AstNode>> {
        self.parse_operation()
    }


    // fn parse_struct_definition(&mut self) -> eyre::Result<Rc<AstNode>> {

    // }


    // fn parse_enum_definition(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_struct_member(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_expression(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_lambda(&mut self) -> eyre::Result<AstNode> {

    // }


    fn parse_operation(&mut self) -> eyre::Result<Rc<AstNode>> {
        let mut left = self.parse_unary()?;

        while self.has_next() && TokenType::is_binary_operator(self.peek().as_ref()) {
            let operator = self.next_token();
            let right = self.parse_unary()?;
            left = Rc::new(AstNode::BinaryOperation { operator: BinaryOperator::PlusEqual, lhs: left, rhs: right })
        }

        Ok(left)
    }


    fn parse_literal(&mut self) -> eyre::Result<Rc<AstNode>> {
        let token = self.next_token().clone();

        if token.value.is_none() {
            return Err(eyre!("Expected an operand: "));
        }

        let value = token.as_ref()
                        .value
                        .clone()
                        .unwrap();
        
        match value {
            PieValue::IntegerLiteral(i) => {
                let node = Rc::new(AstNode::IntegerLiteral(i));
                Ok(node)
            },

            PieValue::FloatLiteral(f) => {
                let node = Rc::new(AstNode::FloatLiteral(f));
                Ok(node)
            },

            PieValue::StringLiteral(s) => {
                let node = Rc::new(AstNode::StringLiteral(s.to_string()));
                Ok(node)
            },

            _ => Err(eyre!("Expected an integer, found {}", token.as_ref().lexeme))
        }
    }


    fn parse_unary(&mut self) -> eyre::Result<Rc<AstNode>> {
        use TokenType::*;

        if !self.has_next() {
            return Err(eyre!("Expected an expression."));
        }

        let token = self.peek();

        match token.as_ref().type_ {
            Integer | Float | TokenType::String
                 => self.parse_literal(),

            True => {
                let node = Rc::new(AstNode::BooleanLiteral(true));
                Ok(node)
            }

            False => {
                let node = Rc::new(AstNode::BooleanLiteral(false));
                Ok(node)
            }

            Identifier => {
                let value = token.as_ref().lexeme.to_string();
                let node = Rc::new(AstNode::Identifier(value));
                Ok(node)
            }

            LeftParen => self.parse_parentheses(),

            _ => Err(eyre!("Unexpected token: {}", token.lexeme))
        }
    }


    fn parse_parentheses(&mut self) -> eyre::Result<Rc<AstNode>> {
        self.next_token();
        let expr = self.parse_operation()?;

        if !self.has_next() {
            return Err(eyre!("Expected ')'"));
        }

        let right_paren = self.peek();

        return match right_paren.type_ {
            TokenType::RightParen => Ok(expr),
            _ => Err(eyre!("Expected ')'"))
        };
    }


    // fn parse_operation_tail(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_variable_assignment(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_function_definition(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_function_body(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_if_statement(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_for_in_loop(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_while_loop(&mut self) -> eyre::Result<AstNode> {

    // }
}


pub fn generate_ast(token_stream: &mut PieTokenStream) -> eyre::Result<Rc<AstNode>> {
    let mut parser = Parser::new(token_stream);
    parser.parse_program()
}