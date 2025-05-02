use std::{fmt::format, rc::Rc};

use color_eyre::{eyre::{self, Ok}, owo_colors::OwoColorize};

use crate::parser::ast::{AstNode, BinaryOperation, BinaryOperator, FunctionCall, FunctionDefinition, IfStatement, ReturnStatement, UnaryOperation, UnaryOperator, WhileLoop};


fn transpile_unary_operation(operation: &UnaryOperation) -> eyre::Result<String> {
    let operand = generate_python(operation.operand.clone())?;
    let operator = transpile_unary_operator(&operation.operator);
    return Ok(format!("{}{}", String::from(operator), operand));
}


fn transpile_binary_operation(operation: &BinaryOperation) -> eyre::Result<String> {
    let operator = transpile_binary_operator(&operation.operator);
    let left = generate_python(operation.left_child.clone())?;
    let right = generate_python(operation.right_child.clone())?;
    return Ok(format!("{} {} {}", left, operator, right));
}


fn transpile_function_call(call: &FunctionCall) -> eyre::Result<String> {
    let name = &call.function;

    let args = call.args
            .iter()
            .map(|arg| generate_python(arg.clone()))
            .collect::<eyre::Result<Vec<String>, _>>()?
            .join(", ");

    return Ok(format!("{}({})", name, args));
}


fn transpile_function_definition(function: &FunctionDefinition) -> eyre::Result<String> {
    let name = function.name.clone();

    let params = function.param_list
            .iter()
            .map(|param| generate_python(param.clone()))
            .collect::<eyre::Result<Vec<String>>>()?
            .join(", ");

    let body = function.body
            .iter()
            .map(|statement| generate_python(statement.clone()))
            .collect::<eyre::Result<Vec<String>>>()?
            .join("\n\t");

    let return_type = function.return_type.clone();

    return Ok(format!("def {}({}):\n\t{}", name, params, body));
}


fn transpile_if_statement(statement: &IfStatement) -> eyre::Result<String> {
    let condition = generate_python(statement.condition.clone())?;

    let body = statement.body
            .iter()
            .map(|statement| generate_python(statement.clone()))
            .collect::<eyre::Result<Vec<String>>>()?
            .join("\n\t");

    return Ok(format!("if {}:\n\t{}", condition, body));
}


fn transpile_return_statement(statement: &ReturnStatement) -> eyre::Result<String> {
    let value = generate_python(statement.body.clone())?;
    return Ok(format!("return {}", value));
}


fn transpile_while_loop(while_loop: &WhileLoop) -> eyre::Result<String> {
    let condition = generate_python(while_loop.condition.clone())?;

    let body = while_loop.body
            .iter()
            .map(|statement| generate_python(statement.clone()))
            .collect::<eyre::Result<Vec<String>>>()?
            .join("\n\t");

    return Ok(format!("while {}:\n\t{}", condition, body));
}


pub fn generate_python(program: Rc<AstNode>) -> eyre::Result<String> {
    match program.as_ref() {
        AstNode::UnaryOperation(operation) => transpile_unary_operation(operation),

        AstNode::BinaryOperation(operation) => transpile_binary_operation(operation),

        AstNode::FunctionCall(call) => transpile_function_call(call),

        AstNode::IntegerLiteral(x) => return Ok(x.to_string()),

        AstNode::FloatLiteral(x) => return Ok(x.to_string()),

        AstNode::BooleanLiteral(x) => return Ok(x.to_string()),

        AstNode::StringLiteral(x) =>return Ok(format!("\"{}\"", x)),

        AstNode::Identifier(x) => return Ok(x.to_string()),

        AstNode::LambdaFunction { params, body } => todo!(),

        AstNode::FunctionDefinition(function) => transpile_function_definition(function),

        AstNode::IfStatement(statement) => transpile_if_statement(statement),

        AstNode::ReturnStatement(statement) => transpile_return_statement(statement),
        
        AstNode::WhileLoop(while_loop) => transpile_while_loop(while_loop),
    }
}


fn transpile_unary_operator(operator: &UnaryOperator) -> &'static str {
    match operator {
        UnaryOperator::Minus => "-",
        UnaryOperator::LogicalNot => "not",
    }
}


fn transpile_binary_operator(operator: &BinaryOperator) -> &'static str {
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