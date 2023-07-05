// bringing all elements from collections to the scope
use std::collections::*;
use rand::Rng;
// bringing Ordering and io to the scope
use std::{cmp::Ordering, io};
fn main() {
    // Map initialization
    let mut map = HashMap::new();
    // insert new entry
    map.insert(1, 2);

    // random number in [0..100]
    let secret_number = rand::thread_rng().gen_range(1..=100);
}