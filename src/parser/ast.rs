use core::fmt;
use std::{fmt::write, rc::Rc};

use crate::lexer::{PieToken, TokenType};

#[derive(Debug)]
pub enum AstNode {
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    FunctionCall(FunctionCall),

    IntegerLiteral(i64), FloatLiteral(f64), BooleanLiteral(bool),
    StringLiteral(String), Identifier(String),

    LambdaFunction { params: Vec<Rc<AstNode>>, body: Vec<Rc<AstNode>> },
    FunctionDefinition(FunctionDefinition),

    IfStatement(IfStatement),
    ReturnStatement(ReturnStatement)
}


impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnaryOperation(op) => write!(f, "{}", op),

            Self::BinaryOperation(op) => write!(f, "{}", op),
            
            Self::FunctionDefinition(func) => write!(f, "{}", func),

            Self::IfStatement(stmt) => write!(f, "{}", stmt),

            Self::FunctionCall(func) => write!(f, "{}", func),

            _ => write!(f, "{:?}", self)
        }
    }
}


impl AstNode {
    pub fn is_equality_operator(&self) -> bool {
        let operation: &BinaryOperation;

        match self {
            AstNode::BinaryOperation(op) => operation = op,
            _ => return false,
        };

        match operation.operator {
            BinaryOperator::EqualEqual
            | BinaryOperator::BangEqual
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual => true,

            _ => false
        }
    }


    pub fn is_assignment_operator(&self) -> bool {
        let operation: &BinaryOperation;

        match self {
            AstNode::BinaryOperation(op) => operation = op,
            _ => return false,
        };

        match operation.operator {
            BinaryOperator::PlusEqual
            | BinaryOperator::MinusEqual
            | BinaryOperator::AsteriskEqual
            | BinaryOperator::SlashEqual
            | BinaryOperator::ModulusEqual => true,

            _ => false
        }
    }
}

#[derive(Debug)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Rc<AstNode>
}


impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.operator, self.operand)
    }
}


#[derive(Debug)]
pub struct BinaryOperation {
    pub operator: BinaryOperator,
    pub left_child: Rc<AstNode>,
    pub right_child: Rc<AstNode>
}


impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left_child, self.right_child)
    }
}


#[derive(Debug)]
pub struct IfStatement {
    pub condition: Rc<AstNode>,
    pub body: Vec<Rc<AstNode>>
}


impl fmt::Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(If {} ({:?}))", self.condition, self.body)
    }
}


#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub return_type: TokenType,
    pub param_list: Vec<Rc<AstNode>>,
    pub body: Vec<Rc<AstNode>>
}


impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Def {} ({:?}) ({:?})", self.name, self.param_list, self.body)
    }
}


#[derive(Debug)]
pub struct FunctionCall {
    pub function: String,
    pub args: Vec<Rc<AstNode>>
}


impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Call {} ({:?}))", self.function.as_str(), &self.args)
    }
}


#[derive(Debug)]
pub struct LambdaFunction {
    pub params: Vec<Rc<AstNode>>, 
    pub body: Vec<Rc<AstNode>>
}


impl fmt::Display for LambdaFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(λ ({:?}) ({:?}))", self.params, self.body)
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
    PlusEqual, MinusEqual, AsteriskEqual, Exponent,
    SlashEqual, Modulus, ModulusEqual,
    Equal, EqualEqual, BangEqual, Less, Greater,
    LessEqual, GreaterEqual,
    Or, And
}


impl BinaryOperator {
    pub fn from(token: &PieToken) -> Option<Self> {
        match token.type_ {
            TokenType::Plus => Some(Self::Plus),
            TokenType::PlusEqual => Some(Self::PlusEqual),
            TokenType::Minus => Some(Self::Minus),
            TokenType::MinusEqual => Some(Self::MinusEqual),
            TokenType::Asterisk => Some(Self::Asterisk),
            TokenType::AsteriskAsterisk => Some(Self::Exponent),
            TokenType::AsteriskEqual => Some(Self::AsteriskEqual),
            TokenType::Slash => Some(Self::Slash),
            TokenType::SlashEqual => Some(Self::SlashEqual),
            TokenType::Modulus => Some(Self::Modulus),
            TokenType::ModulusEqual => Some(Self::ModulusEqual),
            TokenType::Equal => Some(Self::Equal),
            TokenType::EqualEqual => Some(Self::EqualEqual),
            TokenType::BangEqual => Some(Self::BangEqual),
            TokenType::And => Some(Self::And),
            TokenType::Or => Some(Self::Or),
            TokenType::Less => Some(Self::Less),
            TokenType::LessEqual => Some(Self::LessEqual),
            TokenType::Greater => Some(Self::Greater),
            TokenType::GreaterEqual => Some(Self::GreaterEqual),
            _ => None
        }
    }
}


impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
pub struct ReturnStatement {
    pub body: Rc<AstNode>
}


impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Return {})", self.body)
    }
}