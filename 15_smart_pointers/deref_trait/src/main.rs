mod my_box;
use my_box::MyBox;
fn main() {
    // test with reference
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // using the dereference operator to follow a reference to an i32 value

    // test with smart pointer
    let x = 5;
    let y = Box::new(x); //Using the dereference operator on a Box<i32>

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // test with our smart pointer
    let x = 5; //Attempting to use MyBox<T> in the same way we used references and Box<T>
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

   
    let m = MyBox::new(String::from("Rust")); //The code we would have to write if Rust didn’t have deref coercion
    hello(&(*m)[..]); // same as &m
}

fn hello(name: &str) {
    //A hello function that has the parameter name of type &str
    println!("Hello, {name}!");
}