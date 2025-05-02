use function::Function;
use module::Module;
use obj::{obj_eval, Object};

use crate::ast::{constants, Expr, Stmt};

pub mod ctx;
pub mod function;
pub mod module;
pub mod obj;

pub fn mod_exec(md: &mut Module) {
    let stmts = md.stmts.clone();
    for st in &stmts {
        match st {
            Stmt::FunctCallSimple { name, args } => match md.get_var(name) {
                Some(Object::Funct(id)) => {
                    let fref = md.get_var(name).unwrap();
                    let args_eval: Vec<Object> = args.iter().map(|arg| obj_eval(arg, md)).collect();

                    match fref {
                        Object::Funct(Function::Native { f, .. }) => {
                            f(&args_eval, md);
                        }
                        Object::Funct(Function::Coded { name, args, body }) => {
                            let mut fmd = Module::new();

                            for (j, jv) in args.iter().enumerate() {
                                match jv {
                                    Expr::Var(vn) => {
                                        fmd.vtable.insert(
                                            vn.to_string(),
                                            args_eval.get(j).unwrap().to_owned(),
                                        );
                                    }
                                    _ => (),
                                }
                            }

                            fmd.parent = Box::new(Some(md));
                            fmd.stmts = body.to_vec().clone();

                            mod_exec(&mut fmd);
                        }
                        _ => unreachable!(),
                    }
                }
                Some(_) => println!("'{}' is not a function", name),
                None => println!("Function '{}' does not exist", name),
            },
            Stmt::VardeclSimple { name, val, qual } => {
                let evaluated_val = obj_eval(val, md);
                md.vtable.insert(name.clone(), evaluated_val);
            }
            Stmt::Funcdecl { name, args, body } => {
                md.vtable.insert(
                    name.clone(),
                    Object::Funct(Function::Coded {
                        name: name.clone(),
                        args: args.to_vec(),
                        body: body.to_vec(),
                    }),
                );
            }
            _ => (),
        }
    }
}
