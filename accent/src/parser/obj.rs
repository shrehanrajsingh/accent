use crate::ast::{constants, Arithmetic, EOperator, Expr};
use std::fmt::{self};

use super::{
    classes::{ClassD, ClassO},
    function::Function,
    mod_exec,
    module::Module,
};

#[derive(Debug, Clone)]
pub enum Object {
    Const(constants::Const),
    Funct(Function),
    Class(ClassD),
    ClassObj(ClassO),
}

pub fn obj_eval(e: &Expr, md: &Module) -> Object {
    let mut r: Object = Object::Const(constants::Const::Undef);

    match e {
        Expr::Const(v) => {
            r = Object::Const(v.clone());
        }
        Expr::Var(name) => {
            if md.vtable.contains_key(name) {
                r = md.vtable.get(name).unwrap().clone();
            } else {
                println!("undefined variable '{name}'");
            }
        }
        Expr::FuncCallSimple { name, args } => match md.get_var(name) {
            Some(Object::Funct(id)) => {
                let fref = md.get_var(name).unwrap();
                let args_eval: Vec<Object> = args.iter().map(|arg| obj_eval(arg, md)).collect();

                match fref {
                    Object::Funct(Function::Native { f, .. }) => {
                        let mut mdc = md.clone();
                        f(&args_eval, &mut mdc);
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
                        r = fmd.rt;
                    }
                    _ => unreachable!(),
                }
            }
            Some(Object::Class(cd)) => match cd {
                ClassD::Coded { name, vars } => {
                    r = Object::ClassObj(ClassO {
                        name: name.to_string(),
                        vars: vars.clone(),
                    });

                    let args_eval: Vec<Object> = args.iter().map(|arg| obj_eval(arg, md)).collect();

                    if vars.contains_key("constructor") {
                        match vars.get("constructor").unwrap() {
                            Object::Funct(Function::Coded {
                                name: fname,
                                args: fargs,
                                body: fbody,
                            }) => {
                                let mut cmod = Module::new();
                                cmod.vtable = vars.clone();
                                cmod.stmts = fbody.clone();

                                for (j, jv) in fargs.iter().enumerate() {
                                    match jv {
                                        Expr::Var(vn) => {
                                            cmod.vtable.insert(
                                                vn.to_string(),
                                                args_eval.get(j).unwrap().to_owned(),
                                            );
                                        }
                                        _ => (),
                                    }
                                }

                                cmod.parent = Box::new(Some(md));

                                mod_exec(&mut cmod);
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            },
            Some(_) => println!("'{}' is not a function", name),
            None => println!("Function '{}' does not exist", name),
        },
        Expr::Arith(p) => {
            let mut mp: Vec<Object> = Vec::new();

            for i in p {
                match i {
                    Arithmetic::Node(n) => {
                        mp.push(obj_eval(&n, md));
                    }
                    Arithmetic::Op(o) => {
                        let mut abr = 0.0;

                        match o {
                            EOperator::Plus => {
                                let a = mp.pop().unwrap();
                                let b = mp.pop().unwrap();

                                match (a, b) {
                                    (Object::Const(a_c), Object::Const(b_c)) => match a_c {
                                        constants::Const::Integer(a_c_i) => match b_c {
                                            constants::Const::Integer(b_c_i) => {
                                                abr = b_c_i as f64 + a_c_i as f64;
                                            }
                                            constants::Const::Float(b_c_f) => {
                                                abr = b_c_f + a_c_i as f64;
                                            }
                                            constants::Const::Str { v: b_s_v, .. } => {
                                                let mut ress = String::from(b_s_v);
                                                ress.push_str(&a_c_i.to_string());
                                                mp.push(Object::Const(constants::Const::Str {
                                                    v: ress,
                                                    is_raw: false,
                                                    is_fmt: false,
                                                }));
                                                continue;
                                            }
                                            _ => (),
                                        },
                                        constants::Const::Float(a_c_f) => match b_c {
                                            constants::Const::Integer(b_c_i) => {
                                                abr = b_c_i as f64 + a_c_f;
                                            }
                                            constants::Const::Float(b_c_f) => {
                                                abr = b_c_f + a_c_f;
                                            }
                                            _ => (),
                                        },

                                        constants::Const::Str { v: a_s_v, .. } => match b_c {
                                            constants::Const::Str { v: b_s_v, .. } => {
                                                let mut ress = String::from(b_s_v);
                                                ress.push_str(&a_s_v);
                                                mp.push(Object::Const(constants::Const::Str {
                                                    v: ress,
                                                    is_raw: false,
                                                    is_fmt: false,
                                                }));
                                                continue;
                                            }
                                            constants::Const::Integer(b_i) => {
                                                let mut ress = String::from(b_i.to_string());
                                                ress.push_str(&a_s_v);
                                                mp.push(Object::Const(constants::Const::Str {
                                                    v: ress,
                                                    is_raw: false,
                                                    is_fmt: false,
                                                }));
                                                continue;
                                            }
                                            _ => (),
                                        },
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }

                            EOperator::Minus => {
                                let a = mp.pop().unwrap();
                                let b = mp.pop().unwrap();

                                match (a, b) {
                                    (Object::Const(a_c), Object::Const(b_c)) => match a_c {
                                        constants::Const::Integer(a_c_i) => match b_c {
                                            constants::Const::Integer(b_c_i) => {
                                                abr = b_c_i as f64 - a_c_i as f64;
                                            }
                                            constants::Const::Float(b_c_f) => {
                                                abr = b_c_f - a_c_i as f64;
                                            }
                                            _ => (),
                                        },
                                        constants::Const::Float(a_c_f) => match b_c {
                                            constants::Const::Integer(b_c_i) => {
                                                abr = b_c_i as f64 - a_c_f;
                                            }
                                            constants::Const::Float(b_c_f) => {
                                                abr = b_c_f - a_c_f;
                                            }
                                            _ => (),
                                        },
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }

                            EOperator::Multiply => {
                                let a = mp.pop().unwrap();
                                let b = mp.pop().unwrap();

                                match (a, b) {
                                    (Object::Const(a_c), Object::Const(b_c)) => match a_c {
                                        constants::Const::Integer(a_c_i) => match b_c {
                                            constants::Const::Integer(b_c_i) => {
                                                abr = b_c_i as f64 * a_c_i as f64;
                                            }
                                            constants::Const::Float(b_c_f) => {
                                                abr = b_c_f * a_c_i as f64;
                                            }
                                            _ => (),
                                        },
                                        constants::Const::Float(a_c_f) => match b_c {
                                            constants::Const::Integer(b_c_i) => {
                                                abr = b_c_i as f64 * a_c_f;
                                            }
                                            constants::Const::Float(b_c_f) => {
                                                abr = b_c_f * a_c_f;
                                            }
                                            _ => (),
                                        },
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }

                            EOperator::Divide => {
                                let a = mp.pop().unwrap();
                                let b = mp.pop().unwrap();

                                match (a, b) {
                                    (Object::Const(a_c), Object::Const(b_c)) => match a_c {
                                        constants::Const::Integer(a_c_i) => match b_c {
                                            constants::Const::Integer(b_c_i) => {
                                                if a_c_i == 0 {
                                                    panic!("err: division by 0");
                                                }
                                                abr = b_c_i as f64 / a_c_i as f64;
                                            }
                                            constants::Const::Float(b_c_f) => {
                                                if a_c_i == 0 {
                                                    panic!("err: division by 0");
                                                }
                                                abr = b_c_f / a_c_i as f64;
                                            }
                                            _ => (),
                                        },
                                        constants::Const::Float(a_c_f) => match b_c {
                                            constants::Const::Integer(b_c_i) => {
                                                if a_c_f == 0.0 {
                                                    panic!("err: division by 0");
                                                }
                                                abr = b_c_i as f64 / a_c_f;
                                            }
                                            constants::Const::Float(b_c_f) => {
                                                if a_c_f == 0.0 {
                                                    panic!("err: division by 0");
                                                }
                                                abr = b_c_f / a_c_f;
                                            }
                                            _ => (),
                                        },
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }
                            _ => (),
                        }

                        // println!("{}", abr);
                        mp.push(Object::Const(constants::Const::Float(abr)));
                    }
                    _ => (),
                }
            }

            r = mp.pop().unwrap();
        }
        Expr::NewConstruct(e) => {
            r = obj_eval(e, md);
        }
        _ => (),
    }

    r
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Const(v) => write!(f, "{}", v),
            Object::Funct(u) => write!(f, "<function>"),
            _ => unreachable!(),
        }
    }
}
