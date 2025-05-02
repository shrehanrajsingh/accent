use std::collections::HashMap;

pub use crate::token::{EOperator, Token};
pub use arithmetic::Arithmetic;
pub use expr::{constants, Expr};
pub use stmt::{Stmt, VarQualifier};

mod arithmetic;
mod expr;
mod stmt;

pub fn expr_gen(toks: &[Token]) -> expr::Expr {
    let mut res: Expr = Expr::Const(constants::Const::Undef);
    let mut i = 0;

    while i < toks.len() {
        if let Some(t) = toks.get(i) {
            match t {
                Token::Identifier(id) => {
                    res = Expr::Var(id.clone());
                }
                Token::Integer(v) => {
                    res = Expr::Const(constants::Const::Integer(*v));
                }
                Token::Float(f) => {
                    res = Expr::Const(constants::Const::Float(*f));
                }
                Token::Str { v, is_raw, is_fmt } => {
                    res = Expr::Const(constants::Const::Str {
                        v: v.to_string(),
                        is_raw: *is_raw,
                        is_fmt: *is_fmt,
                    })
                }
                Token::Operator(
                    EOperator::Plus | EOperator::Minus | EOperator::Multiply | EOperator::Divide,
                ) => {
                    let mut v: Vec<Arithmetic> = Vec::new();
                    v.push(Arithmetic::Node(Box::new(res.clone())));

                    let mut precedence_map: HashMap<EOperator, i8> = HashMap::new();

                    precedence_map.insert(EOperator::Plus, 10);
                    precedence_map.insert(EOperator::Minus, 10);
                    precedence_map.insert(EOperator::Multiply, 20);
                    precedence_map.insert(EOperator::Divide, 20);

                    let op = match t {
                        Token::Operator(p) => p,
                        _ => unreachable!(),
                    };
                    v.push(Arithmetic::Op(op.to_owned()));

                    let mut gb = 0;
                    let mut last_arg_idx = i + 1;
                    let mut j = i + 1;

                    while j < toks.len() {
                        if let Some(jv) = toks.get(j) {
                            match jv {
                                Token::Operator(
                                    EOperator::Plus
                                    | EOperator::Minus
                                    | EOperator::Multiply
                                    | EOperator::Divide,
                                ) if gb == 0 => {
                                    v.push(Arithmetic::Node(Box::new(expr_gen(
                                        &toks[last_arg_idx..j],
                                    ))));

                                    if let Token::Operator(o) = jv {
                                        v.push(Arithmetic::Op(o.to_owned()));
                                    }

                                    last_arg_idx = j + 1;
                                }
                                Token::Operator(
                                    EOperator::LParen | EOperator::LBrace | EOperator::LBracket,
                                ) => gb += 1,
                                Token::Operator(
                                    EOperator::RParen | EOperator::RBrace | EOperator::RBracket,
                                ) => gb -= 1,
                                _ => (),
                            }
                        }
                        j += 1;
                    }

                    v.push(Arithmetic::Node(Box::new(expr_gen(&toks[last_arg_idx..j]))));

                    let mut v_pf: Vec<Arithmetic> = Vec::new();
                    let mut stack: Vec<Arithmetic> = Vec::new();

                    for (j, jv) in v.iter().enumerate() {
                        match jv {
                            Arithmetic::Op(o) => {
                                while stack.len() > 0 {
                                    match stack.last().unwrap() {
                                        Arithmetic::Op(e) => {
                                            if precedence_map.get(e) >= precedence_map.get(o) {
                                                v_pf.push(stack.pop().unwrap());
                                            } else {
                                                break;
                                            }
                                        }
                                        _ => break,
                                    }
                                }

                                stack.push(jv.to_owned());
                            }

                            Arithmetic::Node(n) => {
                                v_pf.push(Arithmetic::Node(n.to_owned()));
                            }
                            _ => (),
                        }
                    }

                    while stack.len() > 0 {
                        v_pf.push(stack.pop().unwrap());
                    }

                    // println!("{}", v.len());
                    // for j in &v {
                    //     println!("> {j:?}");
                    // }

                    // println!("{}", v_pf.len());
                    // for j in &v_pf {
                    //     println!(": {j:?}");
                    // }

                    res = Expr::Arith(v_pf);
                    i = toks.len();
                }
                Token::Bool(b) => res = Expr::Const(constants::Const::Bool(*b)),
                _ => (),
            }
        }

        i += 1;
    }

    res
}

pub fn stmt_gen(toks: &[Token]) -> Vec<stmt::Stmt> {
    let mut res: Vec<stmt::Stmt> = Vec::new();
    let mut i = 0;

    while i < toks.len() {
        if let Some(t) = toks.get(i) {
            match t {
                Token::Keyword(kw) => {
                    if kw == "let" || kw == "var" || kw == "const" {
                        let qual = match kw.as_str() {
                            "let" => VarQualifier::Let,
                            "const" => VarQualifier::Const,
                            "var" => VarQualifier::Var,
                            _ => unreachable!(),
                        };

                        if let Some(Token::Identifier(id)) = toks.get(i + 1) {
                            /* simple definition */
                            assert!(matches!(
                                toks.get(i + 2).unwrap(),
                                Token::Operator(EOperator::Eq)
                            ));

                            let mut j = i + 3;

                            /* find semicolon */
                            while let Some(tj) = toks.get(j) {
                                match tj {
                                    Token::Operator(EOperator::Semicolon) => {
                                        break;
                                    }
                                    _ => j += 1,
                                }
                            }

                            let st = Stmt::VardeclSimple {
                                name: String::from(id),
                                val: expr_gen(&toks[i + 3..j]),
                                qual,
                            };
                            res.push(st);
                        }
                    } else if kw == "function" {
                        /* get name */
                        let name = toks.get(i + 1).unwrap();

                        /* check '(' */
                        assert!(matches!(
                            toks.get(i + 2).unwrap(),
                            Token::Operator(EOperator::LParen)
                        ));

                        let mut gb = 0;
                        let mut args: Vec<Expr> = Vec::new();
                        let mut last_idx = i + 3;

                        for j in i + 3..toks.len() {
                            let t = toks.get(j).unwrap();

                            match t {
                                Token::Operator(
                                    EOperator::LParen | EOperator::LBrace | EOperator::LBracket,
                                ) => gb += 1,
                                Token::Operator(
                                    EOperator::RParen | EOperator::RBrace | EOperator::RBracket,
                                ) => {
                                    if matches!(t, Token::Operator(EOperator::RParen)) {
                                        args.push(expr_gen(&toks[last_idx..j]));
                                        last_idx = j + 1;
                                        break;
                                    }

                                    gb -= 1;
                                }

                                Token::Operator(EOperator::Comma) => {
                                    if gb == 0 {
                                        args.push(expr_gen(&toks[last_idx..j]));
                                        last_idx = j + 1;
                                    }
                                }
                                _ => (),
                            }
                        }

                        gb = 0;
                        let mut block_end_idx = last_idx;

                        for j in last_idx + 1..toks.len() {
                            let t = toks.get(j).unwrap();

                            match t {
                                Token::Operator(
                                    EOperator::LParen | EOperator::LBrace | EOperator::LBracket,
                                ) => gb += 1,
                                Token::Operator(
                                    EOperator::RParen | EOperator::RBrace | EOperator::RBracket,
                                ) => {
                                    if matches!(t, Token::Operator(EOperator::RBrace)) {
                                        block_end_idx = j;
                                        break;
                                    }
                                    gb -= 1;
                                }
                                _ => (),
                            }
                        }

                        let stmt_tree = stmt_gen(&toks[last_idx..block_end_idx]);

                        let st = Stmt::Funcdecl {
                            name: match name {
                                Token::Identifier(n) => n.to_string(),
                                _ => panic!("Invalid syntax for function declaration"),
                            },
                            args,
                            body: stmt_tree,
                        };

                        res.push(st);
                        i = block_end_idx - 1;
                    } else if kw == "return" {
                        let mut end_idx = 0;
                        let mut gb = 0;

                        for j in i + 1..toks.len() {
                            let t = toks.get(j).unwrap();

                            match t {
                                Token::Operator(
                                    EOperator::LParen | EOperator::LBrace | EOperator::LBracket,
                                ) => gb += 1,
                                Token::Operator(
                                    EOperator::RParen | EOperator::RBrace | EOperator::RBracket,
                                ) => gb -= 1,

                                Token::Operator(EOperator::Semicolon) => {
                                    if gb == 0 {
                                        end_idx = j;
                                        break;
                                    }
                                }
                                _ => (),
                            }
                        }

                        let st = Stmt::ReturnStmt(expr_gen(&toks[i + 1..end_idx]));

                        res.push(st);
                        i = end_idx;
                    } else if kw == "function" {
                        let mut name: String = String::from("func_undefined");

                        match toks.get(i + 1) {
                            Some(Token::Identifier(n)) => {
                                name = n.clone();
                            }
                            Some(_) => println!("syntax error."),
                            None => println!("unexpected EOF."),
                        }

                        assert!(matches!(
                            toks.get(i + 2).unwrap(),
                            Token::Operator(EOperator::LParen)
                        )); // check '('

                        let mut gb = 0;
                        let mut last_arg_idx = i + 3;
                        let mut args: Vec<Expr> = Vec::new();

                        for j in i + 3..toks.len() {
                            match toks.get(j).unwrap() {
                                Token::Operator(
                                    EOperator::LBrace | EOperator::LBracket | EOperator::LParen,
                                ) => gb += 1,
                                Token::Operator(EOperator::RParen) => {
                                    if gb == 0 {
                                        if last_arg_idx != i + 3 {
                                            args.push(expr_gen(&toks[last_arg_idx..j]));
                                        }
                                        last_arg_idx = j + 1;
                                        break;
                                    }

                                    gb -= 1;
                                }
                                Token::Operator(EOperator::RBrace | EOperator::RBracket) => gb -= 1,
                                Token::Operator(EOperator::Comma) if gb == 0 => {
                                    args.push(expr_gen(&toks[last_arg_idx..j]));
                                    last_arg_idx = j + 1;
                                }
                                _ => (),
                            }
                        }

                        gb = 0;
                        let mut stmts: Vec<Stmt> = Vec::new();
                        let mut block_end_idx = last_arg_idx;

                        for j in last_arg_idx + 1..toks.len() {
                            match toks.get(j).unwrap() {
                                Token::Operator(
                                    EOperator::LBrace | EOperator::LBracket | EOperator::LParen,
                                ) => gb += 1,
                                Token::Operator(EOperator::RBrace) => {
                                    if gb == 0 {
                                        block_end_idx = j;
                                        break;
                                    }

                                    gb -= 1;
                                }
                                Token::Operator(EOperator::RParen | EOperator::RBracket) => gb -= 1,
                                _ => (),
                            }
                        }

                        stmts = stmt_gen(&toks[last_arg_idx + 1..block_end_idx]);

                        let st: Stmt = Stmt::Funcdecl {
                            name,
                            args,
                            body: stmts,
                        };

                        res.push(st);
                        i = block_end_idx;
                    }
                }
                Token::Operator(EOperator::LParen) => {
                    if i > 0 {
                        if let Token::Identifier(func_name) = &toks[i - 1] {
                            let mut gb = 0;
                            let mut args = Vec::new();
                            let mut last_arg_start = i + 1;
                            let mut end_idx = i;

                            for j in (i + 1)..toks.len() {
                                match &toks[j] {
                                    Token::Operator(
                                        EOperator::LParen | EOperator::LBrace | EOperator::LBracket,
                                    ) => gb += 1,
                                    Token::Operator(EOperator::RParen) => {
                                        if gb == 0 {
                                            if last_arg_start < j {
                                                args.push(expr_gen(&toks[last_arg_start..j]));
                                            }
                                            end_idx = j;
                                            break;
                                        }
                                        gb -= 1;
                                    }
                                    Token::Operator(EOperator::RBrace | EOperator::RBracket) => {
                                        gb -= 1
                                    }
                                    Token::Operator(EOperator::Comma) if gb == 0 => {
                                        args.push(expr_gen(&toks[last_arg_start..j]));
                                        last_arg_start = j + 1;
                                    }
                                    _ => {}
                                }
                            }

                            /* for j in &args {
                                println!("{j:?}");
                            } */

                            let mut semicolon_idx = end_idx;
                            for j in (end_idx + 1)..toks.len() {
                                if let Token::Operator(EOperator::Semicolon) = &toks[j] {
                                    semicolon_idx = j;
                                    break;
                                }
                            }

                            let st = Stmt::FunctCallSimple {
                                name: func_name.clone(),
                                args,
                            };
                            res.push(st);

                            i = semicolon_idx;
                        }
                    }
                }
                _ => (),
            }
        }

        i += 1;
    }

    res
}
