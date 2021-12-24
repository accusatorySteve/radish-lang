use std::fmt;
use crate::token::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: ASTNode,
    pub op: Op,
    pub right: ASTNode,
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ left: {:?}, op: {:?}, right: {:?}, }}", self.left, self.op, self.right)
    }
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    BinaryExpr(Box<BinaryExpr>, Span),
    Literal(Literal, Span),
}

impl ASTNode {
    pub fn position(&self) -> Span {
        match self {
            Self::BinaryExpr(_, pos)
            | Self::Literal(_, pos) => *pos, 
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AST {
    pub items: Vec<ASTNode>,
}
