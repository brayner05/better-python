pub mod ast;

use std::rc::Rc;
use color_eyre::eyre::{self, eyre, OptionExt};
use crate::lexer::{CanBeEof, PieToken, PieTokenStream, PieValue, TokenType};
use ast::*;


struct Parser <'a> {
    token_stream: &'a mut PieTokenStream,
}


impl <'a> Parser <'a> {
    pub fn new(token_stream: &'a mut PieTokenStream) -> Self {
        Self {
            token_stream: token_stream,
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
        while self.has_next() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }


    fn parse_statement(&mut self) -> eyre::Result<Rc<AstNode>> {
        let next = self.peek();

        match next.type_.clone() {
            TokenType::Def => return self.parse_function_definition(),
            TokenType::If => return self.parse_if_statement(),
            _ => {}
        }

        self.parse_equality()
        // self.parse_p1_operation()
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
        let mut args = vec![];

        if self.has_next() && self.peek().type_ == TokenType::RightParen {
            return Ok(vec![]);
        }

        args.push(self.parse_equality()?);

        while self.has_next() && self.peek().type_ == TokenType::Comma {
            self.next_token();

            if self.has_next() && self.peek().type_ != TokenType::Identifier {
                return Err(eyre!("Expected an identifier."));
            }

            args.push(self.parse_unary()?);
        }

        Ok(args)
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


    fn parse_equality(&mut self) -> eyre::Result<Rc<AstNode>> {
        let mut left = self.parse_arithmetic()?;

        while self.has_next() && self.peek().type_ == TokenType::EqualEqual {
            self.next_token();
            let right = self.parse_arithmetic()?;

            let operation = BinaryOperation {
                operator: BinaryOperator::EqualEqual,
                left_child: left,
                right_child: right,
            };

            left = Rc::new(AstNode::BinaryOperation(operation));
        }

        Ok(left)
    }


    fn next_token_precedence(&self) -> i8 {
        let next_token = self.peek();
        next_token.type_.precedence_level()
    }


    fn parse_arithmetic(&mut self) -> eyre::Result<Rc<AstNode>> {
        let mut left = self.parse_term()?;
        let precedence_level = TokenType::Plus.precedence_level();

        while self.has_next() && self.next_token_precedence() == precedence_level {
            let operator_token = self.next_token();
            let right = self.parse_term()?;
            
            let operator = 
                BinaryOperator
                    ::from(operator_token.as_ref())
                    .ok_or_eyre(format!("Unexpected {}", operator_token.lexeme.as_str()));

            let operation = BinaryOperation {
                operator: operator?,
                left_child: left,
                right_child: right,
            };

            left = Rc::new(AstNode::BinaryOperation(operation));
        }

        Ok(left)
    }


    fn parse_term(&mut self) -> eyre::Result<Rc<AstNode>> {
        let mut left = self.parse_exponent()?;
        let precedence_level = TokenType::Asterisk.precedence_level();

        while self.has_next() && self.next_token_precedence() == precedence_level {
            let operator_token = self.next_token();
            let right = self.parse_exponent()?;
            
            let operator = 
                BinaryOperator
                    ::from(operator_token.as_ref())
                    .ok_or_eyre(format!("Unexpected {}", operator_token.lexeme.as_str()));

            let operation = BinaryOperation {
                operator: operator?,
                left_child: left,
                right_child: right,
            };

            left = Rc::new(AstNode::BinaryOperation(operation));
        }

        Ok(left)
    }


    fn parse_exponent(&mut self) -> eyre::Result<Rc<AstNode>> {
        let mut left = self.parse_unary()?;
        let precedence_level = TokenType::AsteriskAsterisk.precedence_level();

        while self.has_next() && self.next_token_precedence() == precedence_level {
            let operator_token = self.next_token();
            let right = self.parse_unary()?;
            
            let operator = 
                BinaryOperator
                    ::from(operator_token.as_ref())
                    .ok_or_eyre(format!("Unexpected {}", operator_token.lexeme.as_str()));

            let operation = BinaryOperation {
                operator: operator?,
                left_child: left,
                right_child: right,
            };

            left = Rc::new(AstNode::BinaryOperation(operation));
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


    fn parse_argument_list(&mut self) -> eyre::Result<Vec<Rc<AstNode>>> {
        let mut args = vec![];

        if self.has_next() && self.peek().type_ == TokenType::RightParen {
            return Ok(vec![]);
        }

        args.push(self.parse_equality()?);

        while self.has_next() && self.peek().type_ == TokenType::Comma {
            self.next_token();
            args.push(self.parse_equality()?);
        }

        Ok(args)
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

                if self.peek().type_ == LeftParen {
                    self.next_token();
                    
                    // Parse the arguments to the function
                    let args = self.parse_argument_list()?;

                    // Skip the trailing ')'
                    self.expect_next(RightParen)?;
                    self.next_token();

                    let call = FunctionCall {
                        function: value,
                        args: args
                    };

                    let node = Rc::new(AstNode::FunctionCall(call));
                    return Ok(node);
                }
                

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

                let operation = UnaryOperation {
                    operator: UnaryOperator::Minus,
                    operand: expr,
                };

                let node = Rc::new(AstNode::UnaryOperation(operation));
                Ok(node)
            }

            _ => Err(eyre!("Unexpected token: {}", token.lexeme))
        }
    }


    fn parse_parentheses(&mut self) -> eyre::Result<Rc<AstNode>> {
        self.next_token();
        let expr = self.parse_equality()?;

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

        self.expect_next(TokenType::LeftParen)?;
        self.next_token();

        let params = self.parse_param_list()?;

        self.expect_next(TokenType::RightParen)?;
        self.next_token();

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
        self.next_token();

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


    fn parse_if_statement(&mut self) -> eyre::Result<Rc<AstNode>> {
        self.expect_next(TokenType::If)?;
        self.next_token();

        let condition = self.parse_equality()?;

        self.expect_next(TokenType::Then)?;
        self.next_token();

        let mut body = vec![];
        
        while self.has_next() && self.peek().type_.clone() != TokenType::EndIf {
            body.push(self.parse_statement()?);
        }

        self.expect_next(TokenType::EndIf)?;
        self.next_token();

        let if_statement = IfStatement {
            condition: condition,
            body: body,
        };

        Ok(Rc::new(AstNode::IfStatement(if_statement)))
    }


    // fn parse_for_in_loop(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_while_loop(&mut self) -> eyre::Result<AstNode> {

    // }
}


pub fn generate_ast(token_stream: &mut PieTokenStream) -> eyre::Result<Vec<Rc<AstNode>>> {
    let mut parser = Parser::new(token_stream);
    parser.parse_program()
}