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
        let sexpr: Pair = parser::build_sexpr(&mut tokens);
        println!("{}", sexpr);
    } else { 
        println!("{}", "Expression unbalenced");
    }
}

fn eval(expr: &Rc<Pair>, env: &Rc<Pair>) -> &Rc<Pair> {
    match expr {
        Pair::Atom(ref a) => assoc(expr, env),
        Pair::Cons(ref head, ref tail) => {
            match head {
                //(quote .. (car ...
                Pair::Atom(ref a) => {
                    match a.as_slice() {
                        "quote" => cadr(&expr),
                        "atom"  => atom(eval(cadr(&expr), &env)),
                        "eq"    => eq(  eval(cadr(&expr), &env), eval(caddr(&expr), &env)),
                        "car"   => car( eval(cadr(&expr), &env)),
                        "cdr"   => cdr( eval(cadr(&expr), &env)),
                        "cons"  => cons(eval(cadr(&expr), &env), eval(caddr(&expr), &env)),
                        "cond"  => evcon(cdr(&expr), &env),
                        _       => eval(cons(assoc(car(&expr), &env), cdr(&expr)), &env),
                    }
                },
                //((lambda... and ((label
                Pair::Cons(ref h, ref t) => {
                    match h {
                        Pair::Atom(ref a) => {
                            match a.as_slice() {
                                "label"   => eval(cons(caddar(&expr), cdr(&expr)), 
                                                  cons(list(cadar(&expr), car(&expr)), &env)), 

                                "lambda"  => eval(caddar(&expr), 
                                                  append(pair(cadar(&expr), 
                                                              evlis(cdr(&expr), &env)), &env)),
                                _         => panic!("Undefined function {}", a)
                            }
                        },
                        _ => panic!("Undefined function {}", h),
                    }
                },
                Pair::NIL => panic!("Undefined function NIL"),
            }
        },
        Pair::NIL => Rc::new(Pair::NIL),
    }
}

fn evcon(expr: &Rc<Pair>, env: &Rc<Pair>) -> &Rc<Pair> {
    let pred: &Rc<Pair> = eval(caar(&expr), &env);

    if pred == Rc::new(Pair::Atom("t".to_string())) {
        eval(cadar(&expr), &env)
    } else {
        evcon(cdr(&expr), &env)
    }
}

fn evlis(expr: &Rc<Pair>, env: &Rc<Pair>) -> &Rc<Pair> {
    if null(expr) { 
        Rc::new(Pair::NIL) 
    } else {
        cons(eval(car(&expr), &env), evlis(cdr(&expr), &env))
    }
}

pub fn caar(expr: &Rc<Pair>) -> &Rc<Pair> {
    car(car(expr))
}

pub fn caddar(expr: &Rc<Pair>) -> &Rc<Pair> {
    //(car (cdr (cdr (car
    car(cdr(cdr(car(expr))))
}

//second (car (cdr ))
pub fn cadr(expr: &Rc<Pair>) -> &Rc<Pair> {
    car(cdr(expr))    
}

pub fn cadar(exper: &Rc<Pair>) -> &Rc<Pair> {
    car(cdr(car(exper)))
}

pub fn caddr(expr: &Rc<Pair>) -> &Rc<Pair> {
    car(cdr(cdr(expr)))
}

pub fn cdar(expr: &Rc<Pair>) -> &Rc<Pair> {
    cdr(car(expr))
}

pub fn null(expr: &Rc<Pair>) -> bool {
    match expr {
        Pair::NIL => Rc::new(Pair::Atom("t".to_string())),
        _         => Rc::new(Pair::NIL),
    }
}

// has to be a macro?
// should be multivardic 
pub fn list(x: &Rc<Pair>, y: &Rc<Pair>) -> &Rc<Pair> {
    cons(x, cons(y, Pair::NIL))
}

//(append '(a b) '(c d)) -> '(a b c d)
pub fn append(x: &Rc<Pair>, y: &Rc<Pair>) -> &Rc<Pair> {
    if !null(x) {
        y
    } else {
        cons(car(x), append(cdr(x), y))
    }
}

//creates env: ((k v) (k v) (k v))
pub fn pair(x: &Rc<Pair>, y: &Rc<Pair>) -> &Rc<Pair> {
    if null(x) && null(y) { Rc::new(Pair::NIL) }
    else if !atom(x) && !atom(y){
        cons(list(car(x), car(y)), pair(cdr(x), cdr(y)))
    }
}

//finds k in env and returns v
pub fn assoc(x: &Rc<Pair>, y: &Rc<Pair>) -> &Rc<Pair> {
    if caar(y) == x { cadar(y) } else { assoc(x, cdr(y)) }
}
