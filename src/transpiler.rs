use std::{fmt::format, rc::Rc};

use color_eyre::{eyre::{self, Ok}, owo_colors::OwoColorize};

use crate::parser::ast::{AstNode, BinaryOperator, UnaryOperator};

macro_rules! pythonize {
    ($indent:expr, $(arg:tt)*) => {
        let indent = "\t".repeat($indent);
        format!("{}{}", indent, format!($($arg)*))
    };
}

pub fn generate_python(program: Rc<AstNode>, indentation_depth: u16) -> eyre::Result<String> {
    let indent = "\t".repeat(indentation_depth as usize);

    match program.as_ref() {
        AstNode::UnaryOperation(unary_operation) => {
                        let operand = generate_python(unary_operation.operand.clone(), indentation_depth)?;
                        let operator = transpile_unary_operator(&unary_operation.operator);
                        return Ok(format!("{}{}{}", indent, String::from(operator), operand));
        },

        AstNode::BinaryOperation(binary_operation) => {
                let operator = transpile_binary_operator(&binary_operation.operator);
                let left = generate_python(binary_operation.left_child.clone(), indentation_depth)?;
                let right = generate_python(binary_operation.right_child.clone(), indentation_depth)?;
                return Ok(format!("{}{} {} {}", indent, left, operator, right));
        },

        AstNode::FunctionCall(function_call) => {
                let name = &function_call.function;

                let args = function_call.args
                            .iter()
                            .map(|arg| generate_python(arg.clone(), indentation_depth))
                            .collect::<eyre::Result<Vec<String>, _>>()?
                            .join(", ");

                return Ok(format!("{}{}({})", indent, name, args));
        },

        AstNode::IntegerLiteral(x) => return Ok(format!("{}{}", indent, x)),

        AstNode::FloatLiteral(x) => return Ok(format!("{}{}", indent, x)),

        AstNode::BooleanLiteral(x) => Ok(format!("{}{}", indent, x)),

        AstNode::StringLiteral(x) =>return Ok(format!("{}\"{}\"", indent, x)),

        AstNode::Identifier(x) => Ok(format!("{}{}", indent, x)),

        AstNode::LambdaFunction { params, body } => todo!(),

        AstNode::FunctionDefinition(function) => {
                let name = function.name.clone();

                let params = function.param_list
                            .iter()
                            .map(|param| generate_python(param.clone(), indentation_depth))
                            .collect::<eyre::Result<Vec<String>>>()?
                            .join(", ");

                let body = function.body
                            .iter()
                            .map(|statement| generate_python(statement.clone(), indentation_depth + 1))
                            .collect::<eyre::Result<Vec<String>>>()?
                            .join("\n\t");

                let return_type = function.return_type.clone();

                return Ok(format!("{}def {}({}):\n\t{}", indent, name, params, body));
    },

        AstNode::IfStatement(if_statement) => {
                let condition = generate_python(if_statement.condition.clone(), indentation_depth)?;

                let body = if_statement.body
                            .iter()
                            .map(|statement| generate_python(statement.clone(), indentation_depth))
                            .collect::<eyre::Result<Vec<String>>>()?
                            .join("\n\t");

                return Ok(format!("{}if {}:\n\t{}", indent, condition, body));
            },

        AstNode::ReturnStatement(return_statement) => {
            let value = generate_python(return_statement.body.clone(), indentation_depth)?;
            return Ok(format!("{}return {}", indent, value));
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