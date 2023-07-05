// defining a rectangle struct
// this outer attribute implemets Debug method
//that help us to print the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // init rectangle with two values
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // init rectangle with one value (tuple)
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect1)
    );

    // init rectangle as struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    // print struct with debug format `{:?}`
    println!("rect1 is {:?}", rect1);

    // print struct with debug format `{:#?}` easier tor read
    println!("rect1 is {:#?}", rect1);


    let scale = 2;
    let rect1 = Rectangle {
        // prints debug info of this expression
        width: dbg!(30 * scale), // and give value and ownership back
        height: 50,
    };
    // prints debug info of struct
    dbg!(&rect1);
}

// This function calculate area of rectangle
// width and height are given in params
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// This function calculate area of rectangle
// width and height are given in params as a tuple
fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// This function calculate area of rectangle
// given in params 
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}