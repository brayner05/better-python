pub mod ast;

use std::rc::Rc;
use color_eyre::eyre::{self, eyre, OptionExt};
use crate::lexer::{CanBeEof, PieToken, PieTokenStream, PieValue, TokenType};
use ast::*;


type ParseResult = eyre::Result<Rc<AstNode>>;


///
/// A structure that tracks the state of a stream of tokens, and
/// via methods outputs an Abstract Syntax Tree (AST) constructed from
/// the stream of tokens.
/// 
/// # Fields
/// `token_stream` - The stream of tokens from which to construct the AST.
/// 
/// # Examples
/// ```rust
/// let mut tokens: PieTokenStream = /* Some code */;
/// let ast = Parser::new(tokens)
///     .parse_program();
/// ```
/// 
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


    /// Get the next token and skip ahead in the stream.
    fn next_token(&mut self) -> Rc<PieToken> {
        let token = self.token_stream.next_token();
        token.unwrap()
    }


    /// Get the next token but do not skip ahead in the stream.
    fn peek(&self) -> Rc<PieToken> {
        self.token_stream.peek().unwrap()
    }


    /// Parse an AST from a program.
    fn parse_program(&mut self) -> eyre::Result<Vec<Rc<AstNode>>> {
        let mut statements = vec![];
        while self.has_next() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }


    ///
    /// Parse an AST from a statement. A statement is really any line of code in Nadra
    /// but there exist several kinds of statements.
    /// 
    fn parse_statement(&mut self) -> eyre::Result<Rc<AstNode>> {
        let next = self.peek();

        match next.type_.clone() {
            TokenType::Def => return self.parse_function_definition(),
            TokenType::TypeAlias => {},
            TokenType::If => return self.parse_if_statement(),
            TokenType::Use => return self.parse_use_statement(),
            TokenType::While => return self.parse_while_loop(),
            // TokenType::Struct => return self.parse_struct_definition(),
            _ => {}
        }

        self.parse_equality()
    }


    // fn parse_struct_definition(&mut self) -> ParseResult {
    //     self.expect_next(TokenType::Struct)?;
    //     self.next_token();
    // }


    fn parse_use_statement(&mut self) -> eyre::Result<Rc<AstNode>> {
        self.expect_next(TokenType::Use)?;
        self.next_token();

        self.expect_next(TokenType::Identifier)?;
        let namespace = self.next_token().lexeme.clone();

        let node = UseStatement {
            namespace: namespace.to_string(),
        };

        Ok(Rc::new(AstNode::UseStatement(node)))
    }


    // fn parse_struct_definition(&mut self) -> eyre::Result<Rc<AstNode>> {

    // }


    // fn parse_enum_definition(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_struct_member(&mut self) -> eyre::Result<AstNode> {

    // }


    // fn parse_expression(&mut self) -> eyre::Result<AstNode> {

    // }


    ///
    /// Check if the next token in the stream is equal to `type_`. If it is,
    /// then `Ok()` is returned, otherwise - if the type of next token is not `type_` then
    /// an error will be returned.
    /// 
    /// # Params
    /// - `type_` - The expected type of the next token in the token stream.
    /// 
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


    ///
    /// Construct an AST node from a while loop. While loops in Nadra look like:
    /// ```nadra
    /// while <condition> do
    ///     <body>
    /// done
    /// ```
    /// 
    fn parse_while_loop(&mut self) -> eyre::Result<Rc<AstNode>> {
        // Ensure the first token is `while`
        self.expect_next(TokenType::While)?;
        self.next_token();

        // Parse the condition of the loop.
        let condition = self.parse_equality()?;

        // Ensure the next token is `do`.
        self.expect_next(TokenType::Do)?;
        self.next_token();

        let mut body = vec![];

        // Parse the body of the loop.
        while self.has_next() && self.peek().type_.clone() != TokenType::Done {
            body.push(self.parse_statement()?);
        }

        // Ensure the loop is ended with a `done` statement.
        self.expect_next(TokenType::Done)?;
        self.next_token();

        let while_loop = WhileLoop {
            condition: condition,
            body: body,
        };

        let node = Rc::new(AstNode::WhileLoop(while_loop));
        Ok(node)
    }


    ///
    /// Parse a list of parameters, separated by commas.
    /// 
    fn parse_param_list(&mut self) -> eyre::Result<Vec<Rc<AstNode>>> {
        let mut args = vec![];

        // If no parameters are required then do not attempt to parse them.
        if self.has_next() && self.peek().type_ == TokenType::RightParen {
            return Ok(vec![]);
        }

        // Parse the first parameter.
        args.push(self.parse_unary()?);

        // Parse the parameters until reaching the end of the param list.
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


    ///
    /// Parse an equality expression. Note that if no equality operations are used,
    /// then the left operand will be the result of the equality.
    /// 
    fn parse_equality(&mut self) -> eyre::Result<Rc<AstNode>> {
        let mut left = self.parse_arithmetic()?;

        let equal_precedence = TokenType::EqualEqual.precedence_level();

        // Parse equalities until reaching the end of the expression.
        while self.has_next() && self.next_token_precedence() == equal_precedence {
            let operator_token = self.next_token();
            let right = self.parse_arithmetic()?;

            let operator = BinaryOperator
                ::from(&operator_token)
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


    ///
    /// Get the operator precedence of the next token if it applies.
    /// 
    fn next_token_precedence(&self) -> i8 {
        let next_token = self.peek();
        next_token.type_.precedence_level()
    }


    ///
    /// Parse an expression.
    /// 
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


    ///
    /// Parse a term. Terms are separated by `+`, `-`, etc.
    /// 
    fn parse_term(&mut self) -> eyre::Result<Rc<AstNode>> {
        let mut left = self.parse_exponent()?;
        let precedence_level = TokenType::Asterisk.precedence_level();

        // Parse until no more terms exist in the expression.
        while self.has_next() && self.next_token_precedence() == precedence_level {
            let operator_token = self.next_token();
            let right = self.parse_exponent()?;
            
            // Get the operator between the two terms.
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


    ///
    /// Parse an exponentiation/power. In Nadra, exponents look like:
    /// ```nadra
    /// x ** n
    /// ```
    /// Which is the same as saying *x^n* in math.
    /// 
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


    ///
    /// Parse a literal such as an integer, float, string, etc.
    /// 
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


    ///
    /// Parse the list of arguments being passed to a function call.
    /// 
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


    ///
    /// Parse a unary expression. That is, parse an expression that takes a single
    /// operand. An example of a unary expression in Nadra is the expression:
    /// ```nadra
    /// return -x + 1
    /// ```
    /// Which roughly equates to
    /// ```lisp
    /// (Return 
    ///     (Plus 
    ///         (Negate (Identifier 'x'))
    ///         1
    ///     )
    /// )
    /// ```
    /// 
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

            Return => {
                self.next_token();
                if !self.has_next() {
                    return Err(eyre!("Expected an expression"));
                }

                let expr = ReturnStatement { 
                    body: self.parse_equality()?
                };

                Ok(Rc::new(AstNode::ReturnStatement(expr)))
            }

            Identifier => self.parse_identifier(),

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
            },

            Bang => {
                self.next_token();
                if !self.has_next() {
                    return Err(eyre!("Expected an expression"));
                }

                let expr = self.parse_unary()?;

                let operation = UnaryOperation {
                    operator: UnaryOperator::LogicalNot,
                    operand: expr,
                };

                let node = Rc::new(AstNode::UnaryOperation(operation));
                Ok(node)
            }

            _ => Err(eyre!("Unexpected token: {}", token.lexeme))
        }
    }


    ///
    /// Parse `parent.child` where `parent` and `child` are identifiers. For example,
    /// ```nadra
    /// my_obj.field
    /// ```
    /// 
    fn parse_member_access(&mut self, parent: Rc<AstNode>) -> ParseResult {
        self.expect_next(TokenType::Dot)?;
        self.next_token();

        self.expect_next(TokenType::Identifier)?;
        let child = self.parse_identifier()?;

        let node = MemberAccess {
            parent: parent,
            child: child,
        };

        Ok(Rc::new(AstNode::MemberAccess(node)))
    }


    fn parse_identifier(&mut self) -> ParseResult {
        use TokenType::{Dot, LeftParen, RightParen, Identifier};

        let token = self.next_token();
        let value = token.as_ref().lexeme.to_string();

        if self.peek().type_ == Dot {
            let node = Rc::new(AstNode::Identifier(value));
            return self.parse_member_access(node);
        }

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


    ///
    /// Parse an expression between parentheses.
    /// 
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


    ///
    /// Parse a function definition. Function definitions in Nadra have the form:
    /// ```nadra
    /// def <name>(<param_1>, <param_2>, ...) -> <return_type>
    ///     <body>
    /// enddef
    /// ```
    /// 
    fn parse_function_definition(&mut self) -> eyre::Result<Rc<AstNode>> {
        // Ensure the first token is `def`
        self.expect_next(TokenType::Def)?;
        self.next_token();

        // Ensure the next token is the name of the function.
        self.expect_next(TokenType::Identifier)?;
        let identifier = self.next_token().lexeme.to_string();

        // Ensure the next token is the beginning of the parameter list.
        self.expect_next(TokenType::LeftParen)?;
        self.next_token();

        // Parse the list of parameters for the function.
        let params = self.parse_param_list()?;

        // Ensure the param list ends with a right parenthesis.
        self.expect_next(TokenType::RightParen)?;
        self.next_token();

        // Ensure the return type is specified after a `->`.
        self.expect_next(TokenType::RightArrow)?;
        self.next_token();

        // Ensure a return type was specified.
        self.expect_next(TokenType::Identifier)?;
        let return_type = self.next_token().type_.clone();

        // Parse the body of the function.
        let mut body = vec![];
        while self.has_next() && self.expect_next(TokenType::EndDef).is_err() {
            let statement = self.parse_statement()?;
            body.push(statement);
        }

        // Ensure the function ends with `enddef`.
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


    ///
    /// Parse an if statement. An if statement takes the following form:
    /// ```nadra
    /// if <condition> then
    ///     <body>
    /// [else]?
    ///     <body>?
    /// endif
    /// ```
    /// 
    fn parse_if_statement(&mut self) -> eyre::Result<Rc<AstNode>> {
        // Ensure the first token is `if`
        self.expect_next(TokenType::If)?;
        self.next_token();

        // Parse the condition of the if statement.
        let condition = self.parse_equality()?;

        // Ensure that after the condition there is a `then`.
        self.expect_next(TokenType::Then)?;
        self.next_token();

        // Parse the body of the if statement.
        let mut body = vec![];
        while self.has_next() && self.peek().type_.clone() != TokenType::EndIf {
            body.push(self.parse_statement()?);
        }

        // Ensure the if statement ends with `endif`
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


///
/// Given a stream of tokens, `token_stream`, construct an Abstract Syntax Tree (AST) from 
/// `token_stream`.
/// 
/// For example,
/// ```nadra
/// 2 * fact(5) == 240
/// ```
/// Will generate something similar to:
/// ```
/// (EqEq 
///     (Mult 2 (Call fact 5)) 
///     240
/// )
/// ```
/// 
/// # Params
/// - `token_stream` - The stream of tokens from which to create the AST.
/// 
/// # Returns
/// - An error if an error occurs while constructing the AST.
/// - A list of top-level statements representing the individual syntax trees
/// of every statement in the source code.
/// 
pub fn generate_ast(token_stream: &mut PieTokenStream) -> eyre::Result<Vec<Rc<AstNode>>> {
    let mut parser = Parser::new(token_stream);
    parser.parse_program()
}