pub mod ast;
pub mod parser;
pub mod token;

use std::fs;

pub use parser::{module::Module, obj::Object};

pub fn native_print(args: &[Object], md: &mut Module) -> Object {
    for i in args {
        print!("{i} ");
    }
    println!();
    Object::Const(ast::constants::Const::Undef)
}

#[cfg(test)]
mod tests {
    use crate::parser::{function::Function, mod_exec, module::Module};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn tok_test() {
        let data =
            String::from(fs::read_to_string("src/tests/test.js").expect("File does not exist"));

        let mut toks = token::gen_toks(data);

        for i in toks {
            println!("{:?}", i);
        }
    }

    #[test]
    fn stmt_test() {
        let data =
            String::from(fs::read_to_string("src/tests/test.js").expect("File does not exist"));

        let mut toks = token::gen_toks(data);
        let mut sts = ast::stmt_gen(&toks);

        for (i, iv) in sts.iter().enumerate() {
            println!("{} {:?}", i, iv);
        }
    }

    #[test]
    fn eg_test() {
        let data = String::from("a = 20;");

        let mut eg = ast::expr_gen(&token::gen_toks(data));
    }

    #[test]
    fn mod_test() {
        let data =
            String::from(fs::read_to_string("src/tests/test.js").expect("File does not exist"));

        let mut toks = token::gen_toks(data);
        let mut sts = ast::stmt_gen(&toks);

        let mut md = Module::new();
        md.stmts = sts;

        let fref = Function::Native {
            name: String::from("print"),
            f: native_print,
        };

        md.vtable.insert(String::from("print"), Object::Funct(fref));

        mod_exec(&mut md);

        // for (i, iv) in md.vtable {
        //     println!("{i}: {iv:?}");
        // }
        println!("Program ended.");
    }
}
