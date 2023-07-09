fn main() {
    // creating empty vector
    let v: Vec<i32> = Vec::new();
    // creating vector from list
    let v = vec![1, 2, 3];

    //creating mutable vector
    let mut v = Vec::new();

    // appending elements to vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // creating vector from list
    let v = vec![1, 2, 3, 4, 5];
    // get third element
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // get third element using get
    let third: Option<&i32> = v.get(2);
    
    match third {
        // if there is third element in vector, we get value
        Some(third) => println!("The third element is {third}"),
        // else we dont get any exception but print error
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // that is causing an error
    let does_not_exist = v.get(100); // that is not


    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);
    // that print won`t work because we have mutable and immutable borrows at onse
    // println!("The first element is: {first}");  

    let v = vec![100, 32, 57];
    // iteration on vector
    for i in &v {
        println!("{i}");
    }

    // iterating and changing
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // using a dereference operator `*` to get value
        *i += 50;
    }

    // enum for storing different types in vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // vector with differetnr types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
