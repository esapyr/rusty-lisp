#![feature(globs)]

use cons_cell::Pair;
use cons_cell;
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

/// Creates the initial s-expression from an iterator over the token list.
///
/// # Arguments
/// A consuming (MoveItems) iterator over the vector of tokens.
///
/// This was done so that we could own the "tokens" when iterating
/// in order to turn each String into an Atom(String).
/// 
/// (+ 1 (+ 1 1) 1) =>
/// (+ (1 ((+ (1 (1 ))) (1 ))))
///
/// #Todo
/// Rust supports sibiling call optimization but not TCO.
/// Implement this a sibling call (mutually) recursive?
pub fn build_sexpr(tokens: &mut Vec<String>) -> Pair {
    if !tokens.is_empty() {
        let token: String = tokens.remove(0).expect("here"); 
    
        match token.as_slice() {
            "("      =>
                        {  
                            //This only works because tokens is mutable, and form_sexpr is blocking. 
                            //So the head (depending on the list) will exhaust a lot of tokens before
                            //moving on to the tail.
                            let head: Rc<Pair> = Rc::new( build_sexpr(tokens) );
                            let tail: Rc<Pair> = Rc::new( build_sexpr(tokens) );
                            Pair::Cons(head, tail)
                        },
            ")"      => Pair::NIL,
            _        => Pair::Cons( Rc::new(Pair::Atom(token)), Rc::new( build_sexpr(tokens) )),
        }
    } else {
        return Pair::NIL
    }
}

/*
pub fn build_sexpr(tokens: &mut Vec<String>) -> Rc<Pair> {
    if tokens.len() == 1 { return Rc::new( Pair::Atom( tokens.pop().expect("") )) }

    match form_sexpr( tokens ) {
        Pair::Cons(ref head, ref tail) => head.clone(),
        _                              => panic!("build sexpr failed"),
    }
}
*/
