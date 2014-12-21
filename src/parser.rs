#![feature(globs)]

use cons_cell::*;
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

/// Checks if a vector of tokens is balanced. That is that for every
/// left-paren ("(") a right-paren (")") follows at some point.
/// 
/// #Arguments
/// The tokens you want to check are balanced.
///
/// #Returns
/// true if balanced
/// false if not balanced
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
/// #Todo
/// Rust supports sibiling call optimization but not TCO.
/// Implement this a sibling call (mutually) recursive?
///
/// #Arguments
/// A mutable reference to the vector of tokens.
/// We need a mutable reference to allows us to not take ownership of the tokens
/// (to make recursion easier) and so that the list will eventually exhaust.
/// There is probably a better way to do this, and I may look into it later.
///
/// #Returns
/// A pointer to the the newly generated s-expression (Pair).
pub fn build_sexpr(tokens: &mut Vec<String>) -> Rc<Pair> {
    if !tokens.is_empty() {
        let token: String = tokens.remove(0).expect("here"); 
    
        match token.as_slice() {
            "("      =>
                        {  
                            //This only works because tokens is mutable, and form_sexpr is blocking. 
                            //So the head (depending on the list) will exhaust a lot of tokens before
                            //moving on to the tail.
                            let head: Rc<Pair> = build_sexpr(tokens);
                            let tail: Rc<Pair> = build_sexpr(tokens);
                            cons(head, tail)
                        },
            ")"      => Rc::new(Pair::Atom( "NIL".to_string() )),
            _        => cons( Rc::new(Pair::Atom(token)), build_sexpr(tokens) ),
        }
    } else {
        return Rc::new( Pair::Atom( "NIL".to_string() ) )
    }
}
