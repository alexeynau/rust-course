pub mod point;
pub mod largest;

fn main() {
    // first integer list to find largest
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest::largest(&number_list);
    println!("The largest number is {}", result);

    // first char list to find largest
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest::largest(&char_list);
    println!("The largest char is {}", result);

    // miximg points
    let p1 = point::Point { x: 5, y: 10.4 };
    let p2 = point::Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

