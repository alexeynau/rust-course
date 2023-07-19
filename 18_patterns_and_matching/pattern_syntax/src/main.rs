struct Point {
    x: i32,
    y: i32,
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Text {
    Hello { id: i32 },
}
fn main() {
    // matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching n=named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // got x value here (5)
        _ => println!("Default case, x = {:?}", x),
    }
    // x and y didnt changed
    println!("at the end: x = {:?}, y = {y}", x);

    // multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // matching ranges of values with char
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructing structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // got two variables here a and b
    assert_eq!(0, a);
    assert_eq!(7, b);
    // little bit shorter but the same
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // pattern matching with destructing
    let p = Point { x: 0, y: 7 };
    // find a point type
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // destructing an enumeration
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    // use enum destructing in pattern matching
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        // destructing nested structs
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
    }
    // destructing tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ignoring an entire value with `_` wildcard
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // example of ignoring values in tuple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // ignores second and fourth value
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
    // variable name starts with _ not to get a warning about unused
    let _x = 5;
    let y = 10;

    // ignoring with `..`
    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // igores middle values
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // using match guard in match pattern
    let num = Some(4);

    match num {
        // if x is even
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        // if x is odd
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // example of resolving a shadowing problem with a match guard
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);


    let x = 4;
    let y = false;

    match x {
        // works if x is 4 or 5 or 6 and y is true
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

  
    // example of "@ binding"
    let msg = Text::Hello { id: 5 };

    match msg {
        Text::Hello {
            //create a variable that holds a value at the same time 
            // as we are testing that value for a pattern match
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Text::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Text::Hello { id } => println!("Found some other id: {}", id),
    }
}
