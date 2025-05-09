use crate::ast::{constants, Stmt};
use std::collections::HashMap;

use super::obj::Object;

#[derive(Debug, Clone)]
pub struct Module<'a> {
    pub vtable: HashMap<String, Object>,
    pub stmts: Vec<Stmt>,
    pub parent: Box<Option<&'a Module<'a>>>,
    pub rt: Object,
    pub got_rt: bool,
}

impl<'a> Module<'a> {
    pub fn new() -> Module<'a> {
        Module {
            vtable: HashMap::new(),
            stmts: Vec::new(),
            parent: Box::new(None),
            rt: Object::Const(constants::Const::Undef), /* all functions return undefined by default */
            got_rt: false,
        }
    }

    pub fn get_var(&self, n: &String) -> Option<&Object> {
        if self.vtable.contains_key(n) {
            return self.vtable.get(n);
        }

        if let Some(md) = *self.parent {
            return md.get_var(n);
        }

        return None;
    }

    pub fn add_var(&mut self, n: &String, v: Object) {
        self.vtable.insert(n.clone(), v);
    }
}
