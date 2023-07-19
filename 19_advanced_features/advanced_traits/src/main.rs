use std::fmt;

// example of using traits with the same methods` names
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    // use Pilot::fly(&human_instance) to call
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    // use Wizzard::fly(&human_instance) to call
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    // this method can be called usind dot notation
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// the same thing but with associated functions
trait Animal {    
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    // to call use Dog::baby_name()
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    // to call use <Dog as Animal>::baby_name()
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// example of newtype pattern
struct Wrapper(Vec<String>);
// use wrapper to add functional for standart type
impl fmt::Display for Wrapper {
    // make vector of string printable
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn main() {
    // example of calling function with the same name
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // example of using newtype patterns
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}