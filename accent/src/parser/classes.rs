use std::collections::HashMap;

use crate::{ast::Stmt, Module, Object};

use super::function::Function;

#[derive(Debug, Clone)]
pub enum ClassD {
    Native {
        name: String,
    },
    Coded {
        name: String,
        vars: HashMap<String, Object>,
    },
}

#[derive(Debug, Clone)]
pub struct ClassO {
    pub name: String,
    pub vars: HashMap<String, Object>,
}
