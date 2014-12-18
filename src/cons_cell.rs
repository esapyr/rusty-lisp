#![crate_type = "lib"]
#![crate_name = "ConsCell"]

use std::rc::Rc;
use std::fmt;

/// Lisp lists should be able to share structure
/// Therefore the cdr of the list is a reference counted pointer to an immutable List
/// The cdr of a list is either a pointer to another list object or nil and must be RC'd 
/// The car of a list is either a pointer to list or an atom and must be RC'd
///
/// Because Pair's are defined recursively we have the have the 'base caseses' if you will
/// of the recursion defined in the Pair. (Atom & NIL)
#[deriving (PartialEq, Show)]
pub enum Pair {
    Cons(Rc<Pair>, Rc<Pair>),
    Atom(String),
    NIL
}

//Broken
/*
/// Pretty printing
/// leads to (+ (1... and ((+...
/// double cons -> SOL
/// atom nil -> EOL
/// atom cons -> atom
///
/// Cons(Cons(Atom(def), Cons(Atom(r), Cons(Atom(10), NIL))), NIL)
/// ->(def"r"10), NIL)<-
/// Cons(def r 10, NIL)
impl fmt::Show for Pair {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pair::Cons(ref head, ref tail) =>{
              match(&**head, &**tail) {
                  (&Pair::Atom(ref a), &Pair::Cons(ref h, ref t)) => write!(fmt, "{}{}{}", a, h, t),
                  (&Pair::Atom(ref a), &Pair::NIL) => write!(fmt, "{})", a),
                  (&Pair::Cons(ref h, ref t), &Pair::NIL) => write!(fmt, "({}{})", h, t),
                  (&Pair::Cons(ref h, ref t), &Pair::Cons(ref f, ref l)) => write!(fmt, "({}{}{}{}", h, t, f, l),
                  _ => panic!("{}", "flagrent printing error"),
              }
            }, 
            Pair::Atom(ref a)              => write!(fmt, " \"{}\" ", a),
            Pair::NIL                      => write!(fmt, ""),
        }
    }
}
*/

// Primative functions on pairs
/// Takes a pointer to either a list or an atom and returns that pointer
pub fn quote(expr: Rc<Pair>) -> Rc<Pair> {
    expr
}

/// Checks if something is an atom.
/// Assumes all atoms are strings.
///
/// #Arguments
/// Reference to a Pair.
/// (This means you have to &*pointer when passing.)
///
/// #Returns
/// true for anything that's not a pair
pub fn atom(x: &Rc<Pair>) -> bool {
    match **x {
        Pair::Cons(..) => false,
        Pair::Atom(..) => true,
        Pair::NIL      => true,
    }
}

/// Checks if two symbols are equal
/// (have to check in eval if x and y are of the same type)
///
/// #Arguments
/// Two references to either two pairs or two strings.
/// (This means you have to &*pointer when passing)
///
/// #Returns
/// true if x and y are atoms or NILL and are equal.
pub fn eq(x: &Rc<Pair>, y: &Rc<Pair>) -> bool {
    **x == **y
}

/// Returns the head of the Pair
/// To share structure, clones the pointer, not what it points to.
///
/// #Arguments
/// The pair you want the head of.
///
/// #Returns
/// A clone of the head of the Pair.
/// panics! if expr is an atom.
pub fn first(expr: &Rc<Pair>) -> Rc<Pair> {
    match **expr {
        Pair::Cons(ref head, ref tail) => head.clone(),
        _                              => panic!("Cannot get first of an atom"),
    }
}

/// Returns the tail of the Pair
/// To share structure, clones the pointer, not what it points to.
///
/// #Arguments
/// The pair you want the tail of.
///
/// #Returns
/// A clone of the tail of the Pair
/// panics! if expr is an atom.
pub fn rest(expr: &Rc<Pair>) -> Rc<Pair> {
    match **expr {
        Pair::Cons(ref head, ref tail) => tail.clone(),
        _                              => panic!("Cannot get rest of an atom"),
    }
}

/// y must be list
/// (cons 1 nil) -> (1)
/// (cons 1 '(3 2)) -> (1 3 2)
/// (cons '(1) nil) -> ((1))
/// (cons '(1) '(3 2)) -> ((1) 3 2)
/// structural sharing is only for tail.
pub fn cons(x: &Rc<Pair>, y: &Rc<Pair>) -> Pair {
    Pair::Cons( x.clone(), y.clone() )
}

//requires eval
//pub fn cond() 

