use crate::ast::{Expr, Stmt};

use super::{module::Module, obj::Object};

#[derive(Debug, Clone)]
pub enum Function {
    Native {
        name: String,
        f: fn(&[Object], &mut Module) -> Object,
    },
    Coded {
        name: String,
        args: Vec<Expr>,
        body: Vec<Stmt>,
    },
}
