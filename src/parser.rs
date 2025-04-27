use std::rc::Rc;

use color_eyre::eyre::{self, eyre};

use crate::lexer::{self, CanBeEOF, PieToken, PieTokenStream, PieValue, TokenType};


#[derive(Debug)]
pub enum AstNode {
    UnaryOperation { operator: UnaryOperator, operand: Rc<AstNode> },

    IntegerLiteral(i64), FloatLiteral(f64), BooleanLiteral(bool),
    StringLiteral(String), Identifier(String)
}


#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Minus, LogicalNot
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


    fn parse_program(&mut self) -> eyre::Result<AstNode> {
        self.parse_statement()
    }


    fn parse_statement(&mut self) -> eyre::Result<AstNode> {
    }


    fn parse_struct_definition(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_enum_definition(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_struct_member(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_expression(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_lambda(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_operation(&mut self) -> eyre::Result<Rc<AstNode>> {

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

            LeftParen => {
                self.next_token();
                let result = self.parse_operation();
                if result.is_err() {
                    return result;
                }

                // ! Left off here
                match result {
                    Ok(node) => {},
                    Err(e) => Err(e),
                }
            }

            _ => Err(eyre!("Unexpected token: {}", token.lexeme))
        }
    }


    fn parse_operation_tail(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_variable_assignment(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_function_definition(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_function_body(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_if_statement(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_for_in_loop(&mut self) -> eyre::Result<AstNode> {

    }


    fn parse_while_loop(&mut self) -> eyre::Result<AstNode> {

    }
}