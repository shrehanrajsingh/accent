/**
 * THIS MODULE IS DEPRECATED.  
 * AVOID USING IT AT ALL COSTS.  
 * `Module` now manages functions in `Objects`
 */
use super::{function::Function, module::Module, obj::Object};

pub struct Ctx<'a> {
    pub mods: Vec<&'a mut Module<'a>>,
    pub funcs: Vec<&'a mut Function>,
}

impl<'a> Ctx<'a> {
    pub fn new() -> Ctx<'a> {
        Ctx {
            mods: Vec::new(),
            funcs: Vec::new(),
        }
    }
}
