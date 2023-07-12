// This function calculate a median of sequence
pub fn median(vec: &Vec<i32>) -> f32 {
    // clone vector
    let mut vec_to_sort = vec.clone();
    // sorting vector
    vec_to_sort.sort();
    // vector lentgh
    let length = vec_to_sort.len();

    // if length is even
    if length % 2 == 0 {
        // find left and right median index
        let left = vec_to_sort.get(length / 2).copied().unwrap_or(0);
        let right = vec_to_sort.get(length / 2 + 1).copied().unwrap_or(0);
        // return mean of left and right
        return ((left + right) as f32) / 2.0;
        // if length is odd
    } else {
        // return middle index
        return vec_to_sort.get(length / 2 + 1).copied().unwrap_or(0) as f32;
    }
}


// This function calculate a mode of sequence
// by finding the longest subsequence of indentical elements in sorted vector
pub fn mode(vec: &Vec<i32>) -> i32 {
    // clone vector
    let mut vec_to_sort = vec.clone();
    // sort it
    vec_to_sort.sort();
    // mode, inmitialized with first value
    let mut mode = vec_to_sort.first().copied().unwrap();
    // count a number of same numbers in sequence
    let mut count = 1;
    // max counter
    let mut max = 1;
    // previous element, inmitialized with first value
    let mut prev: i32 = vec_to_sort.first().copied().unwrap();
    // iterating on vector
    for el in &vec_to_sort[1..] {
        // if current element equal to previous
        if *el == prev {
            // increase counter
            count += 1;
        } else {
            // counter to zero
            count = 0;
        }
        // if counter is bigger than max counter
        if count > max {
            // new value of max counter
            max = count;
            // new mode
            mode = *el;
        }
        // save previous
        prev = *el;
    }
    //return mode
    return mode;
}
