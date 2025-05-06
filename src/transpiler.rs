use std::rc::Rc;
use color_eyre::eyre::{self, Ok};
use crate::parser::ast::*;



///
/// A structure that holds the state of the transpiler whilst
/// converting an AST to Python.
/// 
/// # Fields
/// - `indent` - The current indentation of the code. Crucial for generating Python code
/// as it relies of indentation for scope.
/// 
struct PythonTranspiler {
    indent: u16
}


impl PythonTranspiler {
    pub fn new() -> Self {
        Self {
            indent: 0
        }
    }


    ///
    /// 'Indent' the code by returning the *n* tabs where
    /// *n* = `self.indent`
    /// 
    fn indent_code(&self) -> String {
        "\t".repeat(self.indent as usize)
    }


    fn transpile_unary_operation(&mut self, operation: &UnaryOperation) -> eyre::Result<String> {
        let operand = self.generate_python(operation.operand.clone())?;
        let operator = self.transpile_unary_operator(&operation.operator);
        return Ok(format!("{}{}", String::from(operator), operand));
    }


    fn transpile_binary_operation(&mut self, operation: &BinaryOperation) -> eyre::Result<String> {
        let operator = self.transpile_binary_operator(&operation.operator);
        let left = self.generate_python(operation.left_child.clone())?;
        let right = self.generate_python(operation.right_child.clone())?;
        return Ok(format!("{} {} {}", left, operator, right));
    }


    fn transpile_function_call(&mut self, call: &FunctionCall) -> eyre::Result<String> {
        let name = &call.function;

        let args = call.args
                .iter()
                .map(|arg| self.generate_python(arg.clone()))
                .collect::<eyre::Result<Vec<String>, _>>()?
                .join(", ");

        return Ok(format!("{}({})", name, args));
    }


    ///
    /// Transpile a code-block with the appropriate indentation.
    /// 
    /// # Params
    /// - `block` - The statements in the 'block' which will each be indented.
    /// 
    fn transpile_block(&mut self, block: &Vec<Rc<AstNode>>) -> eyre::Result<String> {
        block.iter()
        .map(|stmt| {
            let line = self.generate_python(stmt.clone())?;
            Ok(format!("{}{}\n", self.indent_code(), line))
        })
        .collect::<eyre::Result<String>>()
    }


    fn transpile_function_definition(&mut self, function: &FunctionDefinition) -> eyre::Result<String> {
        let name = function.name.clone();

        let params = function.param_list
                .iter()
                .map(|param| self.generate_python(param.clone()))
                .collect::<eyre::Result<Vec<String>>>()?
                .join(", ");

        self.increase_indent();
        let body = self.transpile_block(&function.body)?;
        self.decrease_indent();

        return Ok(format!("def {}({}):\n{}", name, params, body));
    }


    fn transpile_if_statement(&mut self, statement: &IfStatement) -> eyre::Result<String> {
        let condition = self.generate_python(statement.condition.clone())?;

        self.increase_indent();
        let body = self.transpile_block(&statement.body)?;
        self.decrease_indent();

        return Ok(format!("if {}:\n{}", condition, body));
    }


    fn transpile_return_statement(&mut self, statement: &ReturnStatement) -> eyre::Result<String> {
        let value = self.generate_python(statement.body.clone())?;
        return Ok(format!("return {}", value));
    }


    fn transpile_while_loop(&mut self, while_loop: &WhileLoop) -> eyre::Result<String> {
        let condition = self.generate_python(while_loop.condition.clone())?;

        self.increase_indent();

        let body = while_loop.body
                .iter()
                .map(|statement| self.generate_python(statement.clone()))
                .collect::<eyre::Result<Vec<String>>>()?
                .join(&format!("\n"));

        self.decrease_indent();

        return Ok(format!("while {}:\n{}", condition, body));
    }


    ///
    /// Increase the indentation level of the Python code by 1 tab.
    /// 
    fn increase_indent(&mut self) -> u16 {
        self.indent += 1;
        self.indent
    }


    ///
    /// Decrease the indentation of the Python code by 1 tab, unless
    /// the indentation level is `0`, in which case nothing will happen.
    /// 
    fn decrease_indent(&mut self) -> u16 {
        self.indent = 0.max(self.indent - 1);
        self.indent
    }


    ///
    /// Transpile a Nadra program to Python via it's Abstract Syntax Tree (AST).
    /// 
    /// # Params
    /// - `program` - The root node of the AST to convert to Python.
    /// 
    fn generate_python(&mut self, program: Rc<AstNode>) -> eyre::Result<String> {
        let python_code = match program.as_ref() {
            AstNode::UnaryOperation(operation) 
                        => self.transpile_unary_operation(operation)?,

            AstNode::BinaryOperation(operation) 
                        => self.transpile_binary_operation(operation)?,

            AstNode::FunctionCall(call) 
                        => self.transpile_function_call(call)?,

            AstNode::IntegerLiteral(x) => x.to_string(),

            AstNode::FloatLiteral(x) => x.to_string(),

            AstNode::BooleanLiteral(x) => String::from(if *x { "True" } else { "False" }),

            AstNode::StringLiteral(x) => format!("\"{}\"", x),

            AstNode::Identifier(x) => x.to_string(),

            AstNode::LambdaFunction { params, body } => todo!(),

            AstNode::FunctionDefinition(function)
                        => self.transpile_function_definition(function)?,
                        
            AstNode::IfStatement(statement) 
                        => self.transpile_if_statement(statement)?,

            AstNode::ReturnStatement(statement)
                        => self.transpile_return_statement(statement)?,

            AstNode::WhileLoop(while_loop) 
                        => self.transpile_while_loop(while_loop)?,

            AstNode::UseStatement(use_statement) 
                => format!("import {}", &use_statement.namespace),
        };

        Ok(python_code)
    }


    fn transpile_unary_operator(&self, operator: &UnaryOperator) -> &'static str {
        match operator {
            UnaryOperator::Minus => "-",
            UnaryOperator::LogicalNot => "not ",
        }
    }


    fn transpile_binary_operator(&self, operator: &BinaryOperator) -> &'static str {
        match operator {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Asterisk => "*",
            BinaryOperator::Slash => "/",
            BinaryOperator::PlusEqual => "+=",
            BinaryOperator::MinusEqual => "-=",
            BinaryOperator::AsteriskEqual => "*=",
            BinaryOperator::Exponent => "**",
            BinaryOperator::SlashEqual => "/=",
            BinaryOperator::Modulus => "%",
            BinaryOperator::ModulusEqual => "%=",
            BinaryOperator::Equal => "=",
            BinaryOperator::EqualEqual => "==",
            BinaryOperator::BangEqual => "!=",
            BinaryOperator::Less => "<",
            BinaryOperator::Greater => ">",
            BinaryOperator::LessEqual => "<=",
            BinaryOperator::GreaterEqual => ">=",
            BinaryOperator::Or => "or",
            BinaryOperator::And => "and",
        }
    }
}


///
/// Convert an Abstract Syntax Tree (AST) to Python code. For example, the AST denoted by
/// ```
///  (EqEq 
///     (Mult 2 (Call fact 5)) 
///     240
/// )
/// ```
/// Generates the following python code:
/// ```py
/// 2 * fact(5) == 240
/// ```
/// 
/// # Params
/// - `program` - The root node of the AST to transpile. Note that a Nadra program may
/// contain several ASTs, one for each top-level statement.
/// 
pub fn transpile(program: Rc<AstNode>) -> eyre::Result<String> {
    // Transpile the Nadra AST to Python
    PythonTranspiler::new()
        .generate_python(program)
}