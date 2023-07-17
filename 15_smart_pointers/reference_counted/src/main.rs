use crate::List::{Cons, Nil};
use std::rc::Rc;
//A definition of List that uses Rc<T>
#[derive(Debug)]
enum List {
    // using Rc<T> to bw able to have some refs
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // that won`t work with Box<t>
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("{:?}", c);
}
