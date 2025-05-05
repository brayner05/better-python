use std::rc::Rc;
use color_eyre::eyre::{self, Ok};
use crate::parser::ast::*;


struct Transpiler {
    indent: u16
}


impl Transpiler {
    pub fn new() -> Self {
        Self {
            indent: 0
        }
    }


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


    fn increase_indent(&mut self) -> u16 {
        self.indent += 1;
        self.indent
    }


    fn decrease_indent(&mut self) -> u16 {
        self.indent = 0.max(self.indent - 1);
        self.indent
    }


    pub fn generate_python(&mut self, program: Rc<AstNode>) -> eyre::Result<String> {
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


pub fn transpile(program: Rc<AstNode>) -> eyre::Result<String> {
    Transpiler::new()
        .generate_python(program)
}