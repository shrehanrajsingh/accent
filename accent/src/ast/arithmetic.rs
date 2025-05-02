use super::{EOperator, Expr};

#[derive(Debug, Clone)]
pub enum Arithmetic {
    Node(Box<Expr>),
    Op(EOperator),
}
