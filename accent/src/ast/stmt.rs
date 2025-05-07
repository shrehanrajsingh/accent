use crate::ast::expr;

use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum VarQualifier {
    Const,
    Let,
    Var,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /**
     * Simple variable declaration statement
     * ```let a = b;```
     */
    VardeclSimple {
        name: String,
        val: expr::Expr,
        qual: VarQualifier,
    },

    /**
     * Basic JS function struct
     * ```js
     * function name(arg1, arg2, arg3) {
     *      ...body
     * }
     * ``````
     */
    Funcdecl {
        name: String,
        args: Vec<Expr>,
        body: Vec<Stmt>,
    },

    /**
     * Simple function call statement
     * ```name(arg1, arg2, ...);```
     */
    FunctCallSimple { name: String, args: Vec<expr::Expr> },

    /**
     * Return statement
     * ```return <expr>;```
     */
    ReturnStmt(Expr),

    ClassDecl {
        name: String,
        vars: Vec<Stmt>,
        fns: Vec<Stmt>,
    },
}
