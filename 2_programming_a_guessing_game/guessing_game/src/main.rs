// import input/out library from standard library
use std::io;
// import library for random
use rand::Rng;
// import library with comparator
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    // generate random in number [1, 100] using generator (thread_rng) and its method (gen_range)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");
    // println!("Ввведите свое число.");

    // creates mutable variable of String type (empty instance of String)
    let mut guess = String::new();
    // get stdin instance that used for handling input from user
    io::stdin()
        // read input from user and store it "guess" using a mutable reference (&mut)
        .read_line(&mut guess)
        // handling an exception that read_line may cause
        .expect("Failed to read line");

    // parsing guess to unsigned integer
    let guess: u32 = guess
    // using trim() for removing any whitespaces and \n at the beginning and the end
    .trim().
    // parse method returns
    parse().

    expect("Please type a number!");
    // using a placeholder in string
    println!("You guessed {guess}");
    // println!("Вы назвали число: {guess}");

    // match is a flow control statement like switch case
    // cmp method returns a enum of Ordering, that includes Less, Greater and Equal
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
