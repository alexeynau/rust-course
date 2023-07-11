// generic function, type T has comparator
// This function find the largest element in list of type T
pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // first element is maxat this moment
    let mut largest = &list[0];

    // search in list
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    //result
    largest
}
