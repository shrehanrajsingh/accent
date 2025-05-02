use accent::ast;
use accent::parser;
use accent::token;
use clap::{Arg, Command};
use std::fs;
use std::path::Path;

fn run_file(data: String) {
    let mut toks = accent::token::gen_toks(data);
    let mut sts = accent::ast::stmt_gen(&toks);

    let mut md = accent::Module::new();
    md.stmts = sts;

    let fref = accent::parser::function::Function::Native {
        name: String::from("print"),
        f: accent::native_print,
    };

    md.add_var(&String::from("print"), accent::Object::Funct(fref));

    accent::parser::mod_exec(&mut md);
}

fn main() {
    let matches = Command::new("accent-js")
        .version("0.1.0")
        .about("A JavaScript interpreter written in Rust")
        .arg(Arg::new("file").help("JavaScript file to execute").index(1))
        .get_matches();

    // println!("Arguments received:");
    // for arg in std::env::args() {
    //     println!("  {}", arg);
    // }

    if let Some(file_path) = matches.get_one::<String>("file") {
        let path = Path::new(file_path);
        if path.exists() {
            println!("File to execute: {}", file_path);

            let data = fs::read_to_string(path).unwrap();
            run_file(data);
        } else {
            eprintln!("Error: File '{}' does not exist", file_path);
            std::process::exit(1);
        }
    } else {
        println!("No file specified. Use --help for usage information.");
    }
}
