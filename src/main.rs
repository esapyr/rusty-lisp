#![feature(globs)]
#![crate_name = "rusty-lisp"]

use std::io;

mod cons_cell;
mod parser;


fn main() {
    print!("{}", ">");
    let input = io::stdin()
                   .read_line()
                   .ok()
                   .expect("Couldn't read line"); 

    let mut tokens = parser::tokenize(input);
    
    if parser::is_balanced(&tokens) {
        let sexpr: cons_cell::Pair = parser::build_sexpr(&mut tokens);
        println!("{}", sexpr);
    } else { 
        println!("{}", "Exper unbalenced");
    }
}
