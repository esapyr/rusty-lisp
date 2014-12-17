#![feature(globs)]

use cons_cell::{Expression, Pair};
use std::rc::Rc;

pub fn tokenize(raw_input: String) -> Vec<String> {
    raw_input.trim()
             .replace("(","( ")
             .replace(")", " )") //should find better way
             .replace("(  )","()")
             .replace("'","' ")
             .split(' ')
             .map(|s| s.to_string()) //have to re-allocate the charsplits b/c replace() currently own
             .collect::<Vec<String>>()
}

pub fn is_balanced(tokens: &Vec<String>) -> bool {
    let mut count = 0i;

    for token in tokens.iter() {
        if token.as_slice() == "(" { count += 1 }
        else if token.as_slice() == ")" { 
            count -= 1;
            if count < 0 { return false }
        }
    }

    count == 0
}

pub fn form_sexpr<T: Expression, E: Iterator<String>>(tokens: &mut E) -> Pair<T> {
    let token: &str = tokens.next().expect("").as_slice();

    match token {
        "("      => Pair::Cons( Rc::new(form_sexpr(tokens)), Rc::new(Pair::NIL)),         //returns Pair<Pair<_>>
        ")"      => Pair::NIL,
        _        => Pair::Cons( Rc::new(token.to_string()), Rc::new(form_sexpr(tokens))), //returns Pair<String>
    }
}
