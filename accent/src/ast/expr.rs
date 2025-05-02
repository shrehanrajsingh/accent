use super::arithmetic::Arithmetic;

pub mod constants;

#[derive(Debug, Clone)]
pub enum Expr {
    Const(constants::Const),
    Var(String),
    Arith(Vec<Arithmetic> /* postfix form */),
}
