#![feature(globs)]
#![crate_name = "ruisp"]

mod cons_cell;
mod parser;

type s = cons_cell::Pair<_>;

fn main() {
    println!("{}", cons_cell::Pair::NIL)
}
