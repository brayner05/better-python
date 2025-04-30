use std::{fmt::format, rc::Rc};

use color_eyre::{eyre::{self, Ok}, owo_colors::OwoColorize};

use crate::parser::ast::{AstNode, BinaryOperator, UnaryOperator};

pub fn generate_python(program: Rc<AstNode>) -> eyre::Result<String> {
    match program.as_ref() {
        AstNode::UnaryOperation(unary_operation) => {
                        let operand = generate_python(unary_operation.operand.clone())?;
                        let operator = transpile_unary_operator(&unary_operation.operator);
                        return Ok(format!("{}{}", String::from(operator), operand));
            },
        AstNode::BinaryOperation(binary_operation) => {
                let operator = transpile_binary_operator(&binary_operation.operator);
                let left = generate_python(binary_operation.left_child.clone())?;
                let right = generate_python(binary_operation.right_child.clone())?;
                return Ok(format!("{} {} {}", left, operator, right));
            },
        AstNode::FunctionCall(function_call) => {
                let name = &function_call.function;

                let args = function_call.args
                            .iter()
                            .map(|arg| generate_python(arg.clone()))
                            .collect::<eyre::Result<Vec<String>, _>>()?
                            .join(", ");

                return Ok(format!("{}({})", name, args));
            },
        AstNode::IntegerLiteral(x) => return Ok(x.to_string()),

        AstNode::FloatLiteral(x) => return Ok(x.to_string()),

        AstNode::BooleanLiteral(x) => return Ok(x.to_string()),

        AstNode::StringLiteral(x) =>return Ok(format!("\"{}\"", x)),

        AstNode::Identifier(x) => return Ok(x.to_string()),

        AstNode::LambdaFunction { params, body } => todo!(),

        AstNode::FunctionDefinition(function) => {
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
            },

        AstNode::IfStatement(if_statement) => {
                let condition = generate_python(if_statement.condition.clone())?;

                let body = if_statement.body
                            .iter()
                            .map(|statement| generate_python(statement.clone()))
                            .collect::<eyre::Result<Vec<String>>>()?
                            .join("\n\t");

                return Ok(format!("if {}:\n\t{}", condition, body));
            },

        AstNode::ReturnStatement(return_statement) => {
            let value = generate_python(return_statement.body.clone())?;
            return Ok(format!("return {}", value));
        },
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