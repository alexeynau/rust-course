mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// bringing path to hosting into scope
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //calling a function from hosting
    hosting::add_to_waitlist();
}

// importing libs
use std::fmt;
// smth like typedef
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}
// using IoResult<()> instead of io::Result<()>
fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}