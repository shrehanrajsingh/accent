use crate::ast::{constants, Arithmetic, EOperator, Expr};
use std::fmt::{self};

use super::{function::Function, module::Module};

#[derive(Debug, Clone)]
pub enum Object {
    Const(constants::Const),
    Funct(Function),
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
