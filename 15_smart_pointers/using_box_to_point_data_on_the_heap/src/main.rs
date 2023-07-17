use crate::List::{Cons, Nil};

// recursive struct
#[derive(Debug)]
enum List {
    // cant use Cons(i32, List) because in that case rust doesnt know structure size
    Cons(i32, Box<List>),
    Nil,
}

fn main() {

    // storing an i32 value on the heap using a box as a pointer
    let b = Box::new(5);
    println!("b = {}", b);

    // init recursive struct
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}",list);
}