#![crate_type = "lib"]
#![crate_name = "ConsCell"]

use std::rc::Rc;

///Lisp lists should be able to share structure
///Therefore the cdr of the list is a reference counted pointer to an immutable List
///The cdr of a list is either a pointer to another list object or nil and must be RC'd 
///The car of a list is either a pointer to list or an atom (string|number) and must be RC'd
#[deriving (PartialEq, Show, Clone)]
enum Pair<T: Expression> {
    Cons(Rc<T>, Rc<Pair<T>>),
    NIL
}

// :'(
enum Either<T> {
    Atom(String),
    List(Pair<T>),
}

/// In lisp an expression is either a list or an atom.
/// A list is a Pair, and an atom is a String.
/// In order to implement eq (and Pair for that matter)
/// we have to to have a way of talking about both
/// Atoms (strings) and Lists (Pairs) interchangably.
trait Expression {
    //An expression can tell you if it's an atom.
    fn atom(&self) -> bool;
}

impl<T: Expression> Expression for Pair<T> {
    fn atom(&self) -> bool {
        false
    }
}

impl Expression for String {
    fn atom(&self) -> bool {
        true
    }
}

// Primative functions on pairs
/// Takes a pointer to either a list or an atom and returns that pointer
pub fn quote<T: Expression>(expr: Rc<T>) -> Rc<T> {
    expr
}

/// Checks if something is an atom.
/// Assumes all atoms are strings.
///
/// #Arguments
/// Reference to either a pair or a string.
/// (This means you have to &*pointer when passing.)
///
/// #Returns
/// true for anything that's not a pair
pub fn atom<T: Expression>(x: &T) -> bool {
    x.atom()
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
pub fn eq<T: Expression + PartialEq>(x: &T, y: &T) -> bool {
    x == y
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
pub fn first<T: Expression>(expr: &Pair<T>) -> Rc<T> {
    match expr {
        &Pair::Cons(ref head, ref tail) => head.clone(),
        _                               => panic!("Cannot get first of an atom"),
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
pub fn rest<T: Expression>(expr: &Pair<T>) -> Rc<Pair<T>> {
    match expr {
        &Pair::Cons(ref head, ref tail) => tail.clone(),
        _                               => panic!("Cannot get rest of an atom"),
    }
}

/// y must be list
/// (cons 1 nil) -> (1)
/// (cons 1 '(3 2)) -> (1 3 2)
/// (cons '(1) nil) -> ((1))
/// (cons '(1) '(3 2)) -> ((1) 3 2)
/// structural sharing is only for tail.
pub fn cons<T: Expression>(x: &Rc<T>, y: &Rc<Pair<T>>) -> Pair<T> {
    Pair::Cons( x.clone(), y.clone() )
}

//requires eval
//pub fn cond() 

