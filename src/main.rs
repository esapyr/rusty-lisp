#![feature(globs)]
#![crate_name = "rusty-lisp"]

use std::io;
use std::rc::Rc;
use cons_cell::{Pair, atom, eq, car, cdr, cons};
use parser::{tokenize, build_sexpr, is_balanced};

mod cons_cell;
mod parser;

//read-eval-print
fn main() {
    loop {
        let mut tokens = tokenize(read());
        
        if is_balanced(&tokens) {
            let sexpr: Rc<Pair> = car(build_sexpr(&mut tokens));
            let value: Rc<Pair> = eval(sexpr, Rc::new(Pair::Atom("NIL".to_string())));
            print_sexpr(value);
            println!("");
        } else { 
            println!("{}", "Expression unbalenced");
        }
    }
}

fn read() -> String {
    print!(">");
    io::stdin()
        .read_line()
        .ok()
        .expect("Couldn't read line")
}

fn print_sexpr(sexpr: Rc<Pair>) {
    let mut pair: Rc<Pair> = sexpr; 

    if !to_bool(atom(pair.clone())) {
        print!("(");
    }
    loop {
        match *pair.clone() {
            Pair::Atom(ref a) if a.as_slice() == "NIL" => { print!("NIL"); break },
            Pair::Atom(ref a)                          => { print!("{}", a); break},
            Pair::Cons(ref h, ref t) => {
                match(&**h, &**t) { 
                    (&Pair::Atom(ref a), &Pair::Cons(..))    => { print!("{} ", a); pair = t.clone(); },
                    (&Pair::Cons(..), &Pair::Atom(ref a))    => { 
                        print_sexpr(h.clone());
                        if a.as_slice() != "NIL" {
                            print!(" . {})", a);
                        } else {
                            print!(")");
                        }
                        break;
                    },
                    (&Pair::Atom(ref a), &Pair::Atom(ref n)) => {
                        if n.as_slice() == "NIL" {
                            print!("{})", a);
                            break;
                        } else {
                            print!("({} . {})", a, n);
                            break;
                        }
                    },
                    (&Pair::Cons(..), &Pair::Cons(..)) => {
                        print_sexpr(h.clone());
                        pair = t.clone();
                    },
                }
            },
        }
    }
}

fn apply(f: Rc<Pair>, args: Rc<Pair>) -> Rc<Pair> {
    eval(cons(f.clone(), appq(args.clone())), Rc::new(Pair::Atom("NIL".to_string())))
}

/// Puts the atom "QUOTE" in front of every argument so eval will regard them
/// as standing for themselves
fn appq(m: Rc<Pair>) -> Rc<Pair>{
    if null(m.clone()) {
        Rc::new(Pair::Atom("NIL".to_string()))
    } else {
        cons(list(Rc::new(Pair::Atom("QUOTE".to_string())), car(m.clone())),
             appq(cdr(m.clone())))
    }

}

fn eval(e: Rc<Pair>, a: Rc<Pair>) -> Rc<Pair> {
    if to_bool(atom(e.clone())) { 
        assoc(e.clone(), a.clone()) 
    } else if to_bool(atom(car(e.clone()))) {
        if to_bool(eq(car(e.clone()),      Rc::new(Pair::Atom("QUOTE".to_string())))) { 
            cadr(e.clone()) 
        }
        else if to_bool(eq(car(e.clone()), Rc::new(Pair::Atom("ATOM".to_string())))) {
            atom(eval(cadr(e.clone()), a.clone()))
        }
        else if to_bool(eq(car(e.clone()), Rc::new(Pair::Atom("EQ".to_string())))) {
            eq(eval(cadr(e.clone()), a.clone()), eval(caddr(e.clone()), a.clone()))
        }
        else if to_bool(eq(car(e.clone()), Rc::new(Pair::Atom("COND".to_string())))) {
            evcon(cdr(e.clone()), a.clone())
        }
        else if to_bool(eq(car(e.clone()), Rc::new(Pair::Atom("CAR".to_string())))) {
            car(eval(cadr(e.clone()), a.clone()))
        }
        else if to_bool(eq(car(e.clone()), Rc::new(Pair::Atom("CDR".to_string())))) {
            cdr(eval(cadr(e.clone()), a.clone()))
        }
        else if to_bool(eq(car(e.clone()), Rc::new(Pair::Atom("CONS".to_string())))) {
            cons(eval(cadr(e.clone()), a.clone()), eval(caddr(e.clone()), a.clone()))
        }
        else {
            eval(cons(assoc(car(e.clone()), a.clone()),
                      evlis(cdr(e.clone()), a.clone())),
                 a.clone())
        }
    } 
    else if to_bool(eq(caar(e.clone()), Rc::new(Pair::Atom("LABEL".to_string())))){
        eval(cons(caddar(e.clone()), cdr(e.clone())),
             cons(list(cadar(e.clone()), car(e.clone())), a.clone()))
    } 
    else if to_bool(eq(caar(e.clone()), Rc::new(Pair::Atom("LAMBDA".to_string())))) {
        eval(caddar(e.clone()), append(pair(cadar(e.clone()), cdr(e.clone())), a.clone()))
    } else {
        println!("{}", "fuck eval");
        Rc::new(Pair::Atom("NIL".to_string()))
    }
}

fn evcon(c: Rc<Pair>, a: Rc<Pair>) -> Rc<Pair> {
    if to_bool(eval(caar(c.clone()), a.clone())) {
        eval(cadar(c.clone()), a.clone())
    } else {
        evcon(cdr(c.clone()), a.clone())
    }
}

fn evlis(m: Rc<Pair>, a: Rc<Pair>) -> Rc<Pair> {
    if null(m.clone()) {
        m //just have to return nil
    } else {
        cons(eval(car(m.clone()), a.clone()), evlis(cdr(m.clone()), a.clone()))
    }
}

fn pair(x: Rc<Pair>, y: Rc<Pair>) -> Rc<Pair> {
    if null(x.clone()) && null(y.clone()) {
        x //just have to return nil
    } else if to_bool(atom(x.clone())) && to_bool(atom(y.clone())) {
       cons(list(car(x.clone()), car(y.clone())),
            pair(cdr(x.clone()), cdr(y.clone())))
    } else {
        panic!("fn pair failed")
    }
}

fn append(x: Rc<Pair>, y: Rc<Pair>) -> Rc<Pair> {
    if null(y.clone()) {
        y //just have to return nul
    } else {
        cons(car(x.clone()), append(cdr(x.clone()), y.clone()))
    }
}

fn assoc(x: Rc<Pair>, y: Rc<Pair>) -> Rc<Pair> {
    if null(y.clone()) {
        print_sexpr(x);
        panic!("Symbol not in environment");
    } else {
        if to_bool( eq(caar(y.clone()), x.clone()) ) {
            cadar(y.clone())
        } else {
            assoc(x.clone(), cdr(y.clone()))
        }
    }
}

fn list(x: Rc<Pair>, y: Rc<Pair>) -> Rc<Pair> {
    cons(x.clone(), 
         cons(y.clone(), 
              Rc::new(Pair::Atom("NIL".to_string()))))
}

fn null(e: Rc<Pair>) -> bool {
    match *e {
        Pair::Atom(ref a) if a.as_slice() == "NIL" => true,
        _                                          => false,
    }
}

/// Helper fuction to unrap t and nil
fn to_bool(e: Rc<Pair>) -> bool {
    match *e {
        Pair::Atom(ref a) if a.as_slice() == "NIL"  => false,
        _ => true,
    }
}

fn cadr(e: Rc<Pair>) -> Rc<Pair> {
    car(cdr(e.clone()))
}

fn caddr(e: Rc<Pair>) -> Rc<Pair> {
    car(cdr(cdr(e.clone())))
}

fn caar(e: Rc<Pair>) -> Rc<Pair> {
    car(car(e.clone()))
}

fn caddar(e: Rc<Pair>) -> Rc<Pair> {
    car(cdr(cdr(car(e.clone()))))
}

fn cadar(e: Rc<Pair>) -> Rc<Pair> {
    car(cdr(car(e.clone())))
}
