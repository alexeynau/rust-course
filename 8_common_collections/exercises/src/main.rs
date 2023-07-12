use crate::math::data_analyse;
pub mod translator;
pub mod math;
fn main() {

    // test median
    println!("Hello, world!");
    let v = vec![4, 3, 3, 5, 7, 3, 7, 3, 9, 4, 3, 9, 1, 2, 5, 7, 8, 9, 10, 23];
    let median = data_analyse::median(&v);
    println!("{median}");
    // test mode
    let mode = data_analyse::mode(&v);
    println!("{mode}");

    
    // test pig latin
    let str = "pig";
    let pig_latin = translator::to_pig_latin(str);
    println!("{pig_latin}");
    let str = "apig";
    let pig_latin = translator::to_pig_latin(str);
    println!("{pig_latin}");
}
