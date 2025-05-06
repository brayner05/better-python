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

    MemberAccess(MemberAccess),

    LambdaFunction { params: Vec<Rc<AstNode>>, body: Vec<Rc<AstNode>> },
    FunctionDefinition(FunctionDefinition),

    IfStatement(IfStatement),
    ReturnStatement(ReturnStatement),
    UseStatement(UseStatement),
    WhileLoop(WhileLoop)
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


#[derive(Debug)]
pub struct MemberAccess {
    pub parent: Rc<AstNode>,
    pub child: Rc<AstNode>
}


impl fmt::Display for MemberAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(MemberAccess {}.{})", self.parent, self.child)
    }
}


#[derive(Debug)]
pub struct UseStatement {
    pub namespace: String
}


impl fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Use {})", &self.namespace)
    }
}


///
/// A structure representing the AST equivalent of a while loop.
/// 
/// # Fields
/// - `condition` - The condition/predicate of the while loop.
/// - `body` - The code to execute while `condition == true`.
/// 
#[derive(Debug)]
pub struct WhileLoop {
    pub condition: Rc<AstNode>,
    pub body: Vec<Rc<AstNode>>
}


impl fmt::Display for WhileLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(If {} ({:?}))", self.condition, self.body)
    }
}


///
/// A structure representing a unary operation on the AST.
/// A unary operation has the highest precedence of an expression, other
/// than expressions in parentheses.
/// 
/// # Fields
/// - `operator` - The operator used in the expression.
/// - `operand` - The operand which the operator will operate on.
/// 
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


///
/// A structure representing a binary operation on the AST.
/// A binary operation has a left-hand side and a right-hand side.
/// 
/// # Fields
/// - `operator` - The operator to apply to the operands.
/// - `left_child` - The left-hand operand. Many operations are left associative.
/// - `right_child` - The right-hand operand.
/// 
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


///
/// A structure representing an if statement on the AST.
/// 
/// # Fields
/// `condition` - The condition of the if statement. The `body` of the if statement
/// will only be executed if `condition` evaluates to `true`.
/// ``
/// 
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



///
/// A structure representing a function definition on the AST.
/// 
/// # Fields
/// - `name` - The name or identifier of the function which will be used to identify 
/// the function.
/// - `return_type` - The type that the function will return. If the function does not return a value
/// it should be marked as follows:
/// ```nadra
/// def hello_world() -> None
///     print("Hello, World!")
/// enddef
/// ```
/// - `param_list` - The parameters of the function.
/// - `body` - A list of statements that will be executed every time the 
/// function is called.
/// 
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


///
/// A structure representing a function-call on the AST.
/// 
/// # Fields
/// - `function` - The name/identifier of the function to call.
/// - `args` - Any arguments that will be passed to the function.
/// 
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


///
/// A unary operator is an operator which is used with only one
/// operand.
/// 
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Minus, LogicalNot
}


impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


///
/// A binary operator is an operator which takes two operands.
/// 
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


///
/// A structure representing a return statement on the AST.
/// 
/// # Fields
/// - `body` - The value to return.
/// 
/// TODO: Change the type of `body` to `Option<Rc<AstNode>>` to permit returning no value.
/// 
#[derive(Debug)]
pub struct ReturnStatement {
    pub body: Rc<AstNode>
}


impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Return {})", self.body)
    }
}