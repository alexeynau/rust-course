mod custom;
use custom::CustomSmartPointer;

fn main() {
    // testing custom smart pointer
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c);
    println!("CustomSmartPointers created.");
}