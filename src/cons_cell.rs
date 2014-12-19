#![crate_type = "lib"]
#![crate_name = "ConsCell"]

use std::rc::Rc;
use std::fmt;

/// Lisp lists should be able to share structure
///  The cdr of a list is either a pointer to another list object or nil and must be RC'd 
/// The car of a list is either a pointer to list or an atom and must be RC'd
///
/// Because Pair's are defined recursively we have the have the 'base caseses'
/// of the recursion defined in the Pair. (Atom & NIL)
#[deriving (PartialEq)]
pub enum Pair {
    Cons(Rc<Pair>, Rc<Pair>),
    Atom(String),
    NIL
}

/// Pretty printing
impl fmt::Show for Pair {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pair::Cons(ref head, ref tail) =>{
                match(&**head, &**tail) {
                    (&Pair::Cons(..), &Pair::NIL) => write!(fmt, "({}{})", head, tail),
                    _                             => write!(fmt, "{}{}", head, tail),
                }
            }, 
            Pair::Atom(ref a)              => write!(fmt, " \"{}\" ", a),
            Pair::NIL                      => write!(fmt, ""),
        }
    }
}

// Primative functions on pairs
/// Checks if something is an atom.
/// Assumes all atoms are strings.
///
/// Allocates a new pair for 't and NIL
/// Need to figure out static / globals.
///
/// #Arguments
/// Reference to a Pair.
/// (This means you have to &*pointer when passing.)
///
/// #Returns
/// Rc(Pair::Atom("t")) if x is an atom
/// Rc(Pair::NIL) if x is a cons cell
pub fn atom(x: &Rc<Pair>) -> Rc<Pair> {
    match **x {
        Pair::Cons(..)             => Rc::new( Pair::NIL ),
        Pair::Atom(..) | Pair::NIL => Rc::new( Pair::Atom( "t".to_string() )),
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
pub fn eq(x: &Rc<Pair>, y: &Rc<Pair>) -> Rc<Pair> {
    if **x == **y {
        Rc::new( Pair::Atom( "t".to_string() ))
    } else {
        Rc::new( Pair::NIL )
    }
}

/// Returns the head of the Pair
/// Because cons() clones anything it's passed theres no need
/// to clone the pointer just because you want to borrow it.
///
/// #Arguments
/// The pair you want the head of.
///
/// #Returns
/// A reference to of the head of the Pair.
/// panics! if expr is an atom.
pub fn car(expr: &Rc<Pair>) -> &Rc<Pair> {
    match **expr {
        Pair::Cons(ref head, ref tail) => head,
        _                              => panic!("Cannot get car of an atom"),
    }
}

/// Returns the tail of the Pair
///
/// #Arguments
/// The pair you want the tail of.
///
/// #Returns
/// A reference to the tail of the pair.
/// panics! if expr is an atom.
pub fn cdr(expr: &Rc<Pair>) -> &Rc<Pair> {
    match **expr {
        Pair::Cons(ref head, ref tail) => tail,
        _                              => panic!("Cannot get cdr of an atom"),
    }
}

/// Returns a new pair made up of x and y.
///
/// This is the only cons cell function that adds to the 
/// reference count of a Rc.
pub fn cons(x: &Rc<Pair>, y: &Rc<Pair>) -> Pair {
    Rc::new(Pair::Cons( x.clone(), y.clone() ))
}
