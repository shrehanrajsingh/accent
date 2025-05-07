use crate::{ast::Stmt, Module};

use super::function::Function;

#[derive(Debug, Clone)]
pub enum ClassD<'a> {
    Native { name: String },
    Coded { name: String, md: Box<Module<'a>> },
}
