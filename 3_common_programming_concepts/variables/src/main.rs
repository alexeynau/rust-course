fn main() {
    // create mutable variable to be able to change its value later
    let mut x = 5;
    println!("The value of x is: {x}");
    // changing the value of mutable variable
    x = 6;
    println!("The value of x is: {x}");

    // const declaration
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    // example of shadowing
    let x = 5;

    let x = x + 1;

    // shadowing a variable in brackets
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // changing value of variable in brackets didnt affect on value outside of brackets
    println!("The value of x is: {x}");


    let spaces = "   ";
    // can change a type of variable
    let spaces = spaces.len();

}