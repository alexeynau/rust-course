mod front_of_house;
// export hosting and bringing it to scope
pub use crate::front_of_house::hosting;

// function that have access to scope
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}