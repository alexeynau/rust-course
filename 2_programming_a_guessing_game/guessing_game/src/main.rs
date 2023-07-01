// import input/out library from standard library
use std::io;

fn main() {
    // Guess the number!
    println!("Угадай число!");
    // println!("Please input your guess.");
    println!("Ввведите свое число.");

    // creates mutable variable of String type (empty instance of String)
    let mut guess = String::new();
    // get stdin instance that used for handling input from user
    io::stdin()
    // read input from user and store it "guess" using a mutable reference (&mut)
        .read_line(&mut guess)
    // handling an exception that read_line may cause
        .expect("Failed to read line");
    // using a placeholder in string
    // println!("You guessed {guess}");
    println!("Вы назвали число: {guess}");
}
