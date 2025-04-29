mod ast;

use core::fmt;
use std::rc::Rc;
use color_eyre::eyre::{self, eyre};
use crate::lexer::{self, CanBeEof, PieToken, PieTokenStream, PieValue, TokenType};
use ast::*;


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


    fn parse_program(&mut self) -> eyre::Result<Vec<Rc<AstNode>>> {
        let mut statements = vec![];
        statements.push(self.parse_statement()?);
        Ok(statements)
    }


    fn parse_statement(&mut self) -> eyre::Result<Rc<AstNode>> {
        let next = self.peek();

        match next.type_.clone() {
            TokenType::Def => return self.parse_function_definition(),
            _ => {}
        }

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


    fn expect_next(&self, type_: TokenType) -> eyre::Result<()> {
        if !self.has_next() {
            return Err(eyre!("Reached end of token stream"));
        }

        let next = self.peek().type_.clone();
        if next != type_ {
            return Err(eyre!("Expected {} found {}", type_, next))
        }

        Ok(())
    }


    fn parse_param_list(&mut self) -> eyre::Result<Vec<Rc<AstNode>>> {
        let mut params = vec![];

        self.expect_next(TokenType::LeftParen)?;
        self.next_token();

        while self.has_next() && self.peek().type_.clone() != TokenType::RightParen {
            let param = self.parse_unary()?;
            params.push(param);
        }

        self.expect_next(TokenType::RightParen)?;
        self.next_token();

        Ok(params)
    }


    fn parse_lambda(&mut self) -> eyre::Result<Rc<AstNode>> {
        self.expect_next(TokenType::LeftParen)?;
        self.next_token();

        let params = self.parse_param_list()?;

        self.expect_next(TokenType::RightParen)?;
        self.next_token();

        self.expect_next(TokenType::RightArrow)?;
        let node = AstNode::LambdaFunction { params: params, body: vec![] };
        Ok(Rc::new(node))
    }


    fn parse_operation(&mut self) -> eyre::Result<Rc<AstNode>> {
        let mut left = self.parse_unary()?;

        // If the operation tail is part of a binary expression.
        // ! Precedence not working, needs to be worked on
        while self.has_next() && TokenType::is_binary_operator(self.peek().as_ref()) {
            // Get the operator token
            let operator_token = self.next_token();

            // Ensure the operator is valid for binary expressions.
            let operator = BinaryOperator::from(operator_token.as_ref());
            if operator.is_none() {
                return Err(eyre!("Expected an operator, found {}", operator_token.lexeme.to_string()));
            }

            // Parse the right hand side of the expression.
            let right = self.parse_unary()?;
            left = Rc::new(AstNode::BinaryOperation { operator: operator.unwrap(), lhs: left, rhs: right })
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
                self.next_token();
                let node = Rc::new(AstNode::BooleanLiteral(true));
                Ok(node)
            }

            False => {
                self.next_token();
                let node = Rc::new(AstNode::BooleanLiteral(false));
                Ok(node)
            }

            Identifier => {
                self.next_token();
                let value = token.as_ref().lexeme.to_string();
                let node = Rc::new(AstNode::Identifier(value));
                Ok(node)
            }

            LeftParen => self.parse_parentheses(),

            Minus => {
                self.next_token();
                if !self.has_next() {
                    return Err(eyre!("Expected an expression"));
                }
                let expr = self.parse_unary()?;
                let node = Rc::new(AstNode::UnaryOperation { operator: UnaryOperator::Minus, operand: expr });
                Ok(node)
            }

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
            TokenType::RightParen => {
                self.next_token();
                Ok(expr)
            },
            _ => Err(eyre!("Expected ')'"))
        };
    }


    // fn parse_operation_tail(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_variable_assignment(&mut self) -> eyre::Result<AstNode> {

    // }


    fn parse_function_definition(&mut self) -> eyre::Result<Rc<AstNode>> {
        self.expect_next(TokenType::Def)?;
        self.next_token();

        self.expect_next(TokenType::Identifier)?;
        let identifier = self.next_token().lexeme.to_string();

        let params = self.parse_param_list()?;

        self.expect_next(TokenType::RightArrow)?;
        self.next_token();

        self.expect_next(TokenType::Identifier)?;
        let return_type = self.next_token().type_.clone();

        let mut body = vec![];
        while self.has_next() && self.expect_next(TokenType::EndDef).is_err() {
            let statement = self.parse_statement()?;
            body.push(statement);
        }

        self.expect_next(TokenType::EndDef)?;

        let function = FunctionDefinition {
            name: identifier,
            return_type: return_type,
            param_list: params,
            body: body,
        };

        let node = Rc::new(AstNode::FunctionDefinition(function));
        Ok(node)
    }


    // fn parse_function_body(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_if_statement(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_for_in_loop(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_while_loop(&mut self) -> eyre::Result<AstNode> {

    // }
}


pub fn generate_ast(token_stream: &mut PieTokenStream) -> eyre::Result<Vec<Rc<AstNode>>> {
    let mut parser = Parser::new(token_stream);
    parser.parse_program()
}