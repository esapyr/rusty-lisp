# Rusty Lisp

john mccarthy's lisp in rust using reference counted pointers (Rc) and cons cells.

it's working! albeit very roughly.

##Todo
* Update everything to use sibling call recursion instead of tail call recursion.
* Find a way to replace Rc::new(Pair::Atom("NIL".to_string())) with a static memory location (like 0 or something similar). constant reallocation is killing me.
* Find something that allows me to replace all of those Rc.clone() calls with something cleaner (like just Rc)
* Clean up and document code.
