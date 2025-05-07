use std::vec;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum EOperator {
    Plus,     // +
    Minus,    // -
    Divide,   // /
    Multiply, // *
    Modulus,  // %

    Colon,     // :
    Semicolon, // ;
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    LParen,    // (
    RParen,    // )
    Comma,     // ,

    Eqeq,   // ==
    Eqeqeq, // ===
    Neq,    // !=
    Neqeq,  // !==
    Le,     // <
    Ge,     // >
    Leq,    // <=
    Geq,    // >=
    Eq,     // =
    Not,    // !
    Lshift, // <<
    Rshift, // >>
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Operator(EOperator),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Newline,
    Comment(String),
    Str {
        v: String,
        is_raw: bool,
        is_fmt: bool,
    },
    Undef,
    Eof,
}

impl Token {}

pub fn gen_toks(data: String) -> Vec<Token> {
    let RESERVED_KEYWORDS = vec![
        "let", "const", "var", "if", "else", "for", "while", "do", "function", "return", "class",
        "new",
    ];

    let mut res: Vec<Token> = Vec::new();
    let dred = data.as_bytes();
    let mut i = 0;

    while i < dred.len() {
        let iv = dred[i] as char;
        match iv {
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();

                for j in i..=data.len() - 1 {
                    match dred[j] as char {
                        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                            ident.push(dred[j] as char);
                        }
                        _ => break,
                    }
                }

                i += ident.len() - 1;

                if RESERVED_KEYWORDS.contains(&&ident[..]) {
                    let tk = Token::Keyword(ident);
                    res.push(tk);
                } else if ident == "false" || ident == "true" {
                    let tk = Token::Bool(if ident == "true" { true } else { false });
                    res.push(tk);
                } else if ident == "undefined" {
                    res.push(Token::Undef);
                } else {
                    let tk = Token::Identifier(ident);
                    res.push(tk);
                }
            }

            '+' => {
                res.push(Token::Operator(EOperator::Plus));
            }
            '-' => {
                res.push(Token::Operator(EOperator::Minus));
            }
            '/' => {
                res.push(Token::Operator(EOperator::Divide));
            }
            '*' => {
                res.push(Token::Operator(EOperator::Multiply));
            }
            '%' => {
                res.push(Token::Operator(EOperator::Modulus));
            }
            ':' => {
                res.push(Token::Operator(EOperator::Colon));
            }
            ';' => {
                res.push(Token::Operator(EOperator::Semicolon));
            }
            '{' => {
                res.push(Token::Operator(EOperator::LBrace));
            }
            '}' => {
                res.push(Token::Operator(EOperator::RBrace));
            }
            '[' => {
                res.push(Token::Operator(EOperator::LBracket));
            }
            ']' => {
                res.push(Token::Operator(EOperator::RBracket));
            }
            '(' => {
                res.push(Token::Operator(EOperator::LParen));
            }
            ')' => {
                res.push(Token::Operator(EOperator::RParen));
            }
            ',' => {
                res.push(Token::Operator(EOperator::Comma));
            }
            '=' => {
                if i + 1 < dred.len() && dred[i + 1] as char == '=' {
                    if i + 2 < dred.len() && dred[i + 2] as char == '=' {
                        res.push(Token::Operator(EOperator::Eqeqeq));
                        i += 1;
                    } else {
                        res.push(Token::Operator(EOperator::Eqeq));
                    }
                    i += 1;
                } else {
                    res.push(Token::Operator(EOperator::Eq));
                }
            }
            '!' => {
                if i + 1 < dred.len() && dred[i + 1] as char == '=' {
                    if i + 2 < dred.len() && dred[i + 2] as char == '=' {
                        res.push(Token::Operator(EOperator::Neqeq));
                        i += 1;
                    } else {
                        res.push(Token::Operator(EOperator::Neq));
                    }
                    i += 1;
                } else {
                    res.push(Token::Operator(EOperator::Not));
                }
            }
            '<' => {
                if i + 1 < dred.len() && dred[i + 1] as char == '=' {
                    res.push(Token::Operator(EOperator::Leq));
                    i += 1;
                } else if i + 1 < dred.len() && dred[i + 1] as char == '<' {
                    res.push(Token::Operator(EOperator::Lshift));
                    i += 1;
                } else {
                    res.push(Token::Operator(EOperator::Le));
                }
            }
            '>' => {
                if i + 1 < dred.len() && dred[i + 1] as char == '=' {
                    res.push(Token::Operator(EOperator::Geq));
                    i += 1;
                } else if i + 1 < dred.len() && dred[i + 1] as char == '>' {
                    res.push(Token::Operator(EOperator::Rshift));
                    i += 1;
                } else {
                    res.push(Token::Operator(EOperator::Ge));
                }
            }
            '0'..='9' => {
                let mut num_str = String::new();
                let mut saw_dot = false;

                for j in i..data.len() {
                    let c = dred[j] as char;
                    match c {
                        '0'..='9' => num_str.push(c),
                        '.' if !saw_dot => {
                            saw_dot = true;
                            num_str.push(c);
                        }
                        _ => break,
                    }
                }

                i += num_str.len() - 1;

                if saw_dot {
                    if let Ok(value) = num_str.parse::<f64>() {
                        res.push(Token::Float(value));
                    }
                } else {
                    if let Ok(value) = num_str.parse::<i64>() {
                        res.push(Token::Integer(value));
                    }
                }
            }

            '"' | '\'' | '`' => {
                let q = iv;
                let sp = i;
                let mut is_raw = false;
                let mut is_fmt = q == '`';

                /* if sp > 0 {
                    if dred[sp - 1] as char == 'r' {
                        is_raw = true;
                        i -= 1;
                    } else if dred[sp - 1] as char == 'f' {
                        is_fmt = true;
                        i -= 1;
                    }
                } */

                i += 1;
                let mut scont = String::new();
                let mut saw_bs = false;

                while i < dred.len() {
                    let c = dred[i] as char;

                    if saw_bs && !is_raw {
                        match c {
                            'n' => scont.push('\n'),
                            't' => scont.push('\t'),
                            'r' => scont.push('\r'),
                            '\\' => scont.push('\\'),
                            '\'' => scont.push('\''),
                            '"' => scont.push('"'),
                            _ => scont.push(c),
                        }
                        saw_bs = false;
                    } else if c == '\\' && !is_raw {
                        saw_bs = true;
                    } else if c == q && !saw_bs {
                        break;
                    } else {
                        scont.push(c);
                        saw_bs = false;
                    }

                    i += 1;
                }

                res.push(Token::Str {
                    v: scont,
                    is_raw,
                    is_fmt,
                });
            }

            '\n' => {
                res.push(Token::Newline);
            }

            _ => (),
        }

        i += 1;
    }

    res
}
