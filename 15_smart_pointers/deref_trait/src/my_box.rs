use std::ops::Deref;
//Defining a MyBox<T> as analog to Box<T>
pub struct MyBox<T>(pub T); 


impl<T> MyBox<T> {
    // constructor
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// make MyBox dereferable
impl<T> Deref for MyBox<T> {
    type Target = T;
    // when deref return its value
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
