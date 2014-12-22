use std::rc::Rc;
use std::fmt;

/// A naive persistent list implementation that uses reference counted pointers.
/// Deallocation of a particularly long list made up of 0-referenced pointers
/// could incur some overhead.
///
/// #McCarthy's lists
/// Intended to closely resemble McCarthy's orginal list structure where
/// the head of the list was called the address field (which ends up essentially
/// being a pointer), and the tail was called the decrement (which was a pointer
/// to another list structure).
///
/// In McCarthy's lisp, an atom was also a list where a special constant was
/// stored in the first address field. This allowed there to be metadata 
/// about the symbol (such as how to print it). Due to programmer
/// convience and preformance restrictions Atoms are normal rust Strings.
///
/// In addition to that NIL, while called an Atom, was actually the value
/// zero.
///
/// #Implementation
/// A Pair is an Enum containing two reference counted pointers.
/// Both of which could either point to another Pair or to the
/// Atom NIL.
///
#[deriving (PartialEq, Show)]
pub enum Pair {
    Cons(Rc<Pair>, Rc<Pair>),
    Atom(String),
}

// Primative functions on pairs
/// Checks if something is an atom.
/// Assumes all atoms are strings.
///
/// Allocates a new pair for 't and NIL
///
/// #Arguments
/// Reference to a Pair.
/// (This means you have to &*pointer when passing.)
///
/// #Returns
/// The atom T if x is an atom
/// the atom NIL if x is a Pair
pub fn atom(x: Rc<Pair>) -> Rc<Pair> {
    match *x {
        Pair::Cons(..) => Rc::new( Pair::Atom( "NIL".to_string() )),
        Pair::Atom(..) => Rc::new( Pair::Atom( "T".to_string() )),
    }
}

/// Checks if two symbols are equal
/// x and y being different types is undefined behavior in the original
/// paper, and here the type checker will panic.
///
/// #Arguments
/// Two pointers to either two Pairs or two Atoms
///
/// #Returns
/// The Atom T if x and y are atoms and equal (including NIL)
/// The Atom NIL if x and y are atoms and not equal or are collections.
pub fn eq(x: Rc<Pair>, y: Rc<Pair>) -> Rc<Pair> {
    if *x == *y {
        Rc::new( Pair::Atom( "t".to_string() ))
    } else {
        Rc::new( Pair::Atom( "NIL".to_string() ))
    }
}

/// Returns the head of the Pair
///
/// In the original implementation this just moved the address cell into
/// the IBM 704's accumlator register.
///
/// I was conflicted about how to implement this. In the original
/// this function essentially returned a pointer, but in rust returning
/// pointers is an anti-pattern. I went with returning a clone of the
/// head pointer because of the use of reference counted pointers.
///
/// #Arguments
/// The Pair whose head you want.
///
/// #Returns
/// A clone of the pointer in the head. Because car and cdr are 
/// non-destructive we have to share the pointer with the caller.
///
/// Panics! if expr is an atom.
pub fn car(expr: Rc<Pair>) -> Rc<Pair> {
    match *expr {
        Pair::Cons(ref head, _) => head.clone(),
        Pair::Atom(ref a) if a.as_slice() != "NIL"      => panic!("Cannot get car of an Atom"),
        Pair::Atom(..)                                  => panic!("Cannot get car of an NIL"),
    }
}

/// Returns the tail of the Pair
///
/// #Arguments
/// The pair whose tail you want.
///
/// #Returns
/// A clone of the pointer in the tail.
/// panics! if expr is an atom.
pub fn cdr(expr: Rc<Pair>) -> Rc<Pair> {
    match *expr {
        Pair::Cons(_, ref tail) => tail.clone(),
        Pair::Atom(ref a) if a.as_slice() != "NIL" => panic!("Cannot get car of an Atom"),
        Pair::Atom(..)                             => panic!("Cannot get car of an NIL"),
    }
}

/// Returns a new pair made up of x and y.
///
/// In McCarthy's lisp this took a register off of the free-storage list
/// (part of the garbage collection system) and placed the values of x 
/// and y in the address and decrement of the new register respectively.
///
/// In this implementation cons takes two pointers to pairs
/// and places them in the head and tail of new pair and then allocates that 
/// with Rc::new.
///
/// #Arguments
/// Two Pair pointers that will be made into the head and tail of a new Pair
/// respectively. Assumes you're passing cloned pointers i.e because cons now
/// owns x and y it should be free to wrap them in a Pair and allocate that 
/// without needing to clone.
///
/// #Returns
/// A pointer to a newly allocated Pair. (only list function that allocates)
pub fn cons(x: Rc<Pair>, y: Rc<Pair>) -> Rc<Pair> {
    Rc::new( Pair::Cons( x, y ) )
}
