use std::fmt::{self};

#[derive(Debug, Clone)]
pub enum Const {
    Integer(i64),
    Float(f64),
    Str {
        v: String,
        is_raw: bool,
        is_fmt: bool,
    },
    Undef,
    Bool(bool),
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Const::Integer(i) => write!(f, "{i}"),
            Const::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Const::Float(fl) => write!(f, "{fl}"),
            Const::Str {
                v,
                is_raw: _,
                is_fmt: _,
            } => write!(f, "{v}"),
            Const::Undef => write!(f, "undefined"),
        }
    }
}
