fn main() {

    //=============== if {} else {} =================
    let number = 3;

    // example of if {} else {}
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    let number = 3;

    // cant use there non-integer as condition, only boolean can be used
    if number != 0 {
        println!("number was something other than zero");
    }

    
    // handling multiple conditions
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition = true;
    // using if else in let statement
    // types in if {} and else{} must be same
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    //=================== loop {} =======================
    // infinite loop example
    // loop {
    //     println!("again!");
    // }

    // returning value using loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break will assign this expression to result
            break counter * 2;
        }
    };

    // result is 20
    println!("The result is {result}");


    // using label for routing between multiple loops
    let mut count = 0;
    
    // label
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // go to label
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    // example of conditional loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // iterating in array using conditional loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // iterating in array using for loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // iterating in a range
    // rev() reverses an iterator direction
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}