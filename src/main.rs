#![feature(globs)]
#![crate_name = "rusty-lisp"]

use std::io;
use std::rc::Rc;
use cons_cell::*;

mod cons_cell;
mod parser;


fn main() {
    print!("{}", ">");
    let input = io::stdin()
                   .read_line()
                   .ok()
                   .expect("Couldn't read line"); 

    let mut tokens = parser::tokenize(input);
    
    if parser::is_balanced(&tokens) {
        let sexpr: Rc<Pair> = parser::build_sexpr(&mut tokens);
        println!("{}", *sexpr);
    } else { 
        println!("{}", "Expression unbalenced");
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
        cons(list2(Rc::new(Pair::Atom("QUOTE".to_string())), car(m.clone())),
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
             cons(list2(cadar(e.clone()), car(e.clone())), a.clone()))
    } 
    else if to_bool(eq(caar(e.clone()), Rc::new(Pair::Atom("LAMBDA".to_string())))) {
        eval(caddar(e.clone()), append(pair(cadar(e.clone()), cdr(e.clone())), a.clone()))
    } else {
        panic!("Eval failed")
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
       cons(list2(car(x.clone()), car(y.clone())),
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
    if to_bool(eq(caar(y.clone()), x.clone())) {
        caddar(y.clone())
    } else {
        assoc(x.clone(), cdr(y.clone()))
    }
}

fn list2(x: Rc<Pair>, y: Rc<Pair>) -> Rc<Pair> {
    cons(x.clone(), 
         cons(y.clone(), 
              Rc::new(Pair::Atom("NIL".to_string()))))
}

fn list3(x: Rc<Pair>, y: Rc<Pair>, z: Rc<Pair>) -> Rc<Pair> {
    cons( x.clone(), 
          cons( y.clone(), 
                cons( z.clone(), 
                      Rc::new(Pair::Atom("NIL".to_string())))))
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
        Pair::Atom(ref a) => {
            if a.as_slice() == "t" { true } else { false }
        },
        _ => false,
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
