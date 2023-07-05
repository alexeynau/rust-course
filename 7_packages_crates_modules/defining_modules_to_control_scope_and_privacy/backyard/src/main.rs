// import Struct
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    // create instance of struct
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}