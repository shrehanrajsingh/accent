use function::Function;
use module::Module;
use obj::{obj_eval, Object};

use crate::ast::{constants, Expr, Stmt};

pub mod classes;
pub mod ctx;
pub mod function;
pub mod module;
pub mod obj;

pub fn mod_exec(md: &mut Module) {
    let stmts = md.stmts.clone();
    for st in &stmts {
        if md.got_rt {
            break;
        }

        match st {
            Stmt::FunctCallSimple { name, args } => match md.get_var(name) {
                Some(Object::Funct(id)) => {
                    let fref = md.get_var(name).unwrap();
                    let args_eval: Vec<Object> = args.iter().map(|arg| obj_eval(arg, md)).collect();

                    match fref {
                        Object::Funct(Function::Native { f, .. }) => {
                            let mut fmd = md.clone();
                            f(&args_eval, &mut fmd);
                        }
                        Object::Funct(Function::Coded { name, args, body }) => {
                            let mut fmd = Module::new();

                            for (j, jv) in args.iter().enumerate() {
                                match jv {
                                    Expr::Var(vn) => {
                                        fmd.add_var(vn, args_eval.get(j).unwrap().to_owned());
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
                let evaluated_val = obj_eval(val, &md);
                md.add_var(name, evaluated_val);
            }
            Stmt::Funcdecl { name, args, body } => {
                md.add_var(
                    name,
                    Object::Funct(Function::Coded {
                        name: name.clone(),
                        args: args.to_vec(),
                        body: body.to_vec(),
                    }),
                );
            }
            Stmt::ClassDecl { name, vars, fns } => {
                let mut cmd = Module::new();

                cmd.stmts = vars.clone();
                mod_exec(&mut cmd);

                cmd.stmts = fns.clone();
                mod_exec(&mut cmd);
            }
            Stmt::ReturnStmt(e) => {
                md.rt = obj_eval(e, md);
                md.got_rt = true;
            }
            _ => (),
        }
    }
}
