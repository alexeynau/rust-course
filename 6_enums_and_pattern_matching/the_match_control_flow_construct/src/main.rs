#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
//defining a enum that describes a type of coin
enum Coin {
    Penny,
    Nickel,
    Dime,
    // UsState describes origins of this coin
    Quarter(UsState),
}


fn main() {
    // This function takes an option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        // we have to cover all variants of Option
        match x {
            // if it is some it will increament a argument
            Some(i) => Some(i + 1),
            // else it return None
            None => None,
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // that code describes a logic of board game
    let dice_roll = 9;
    // different options depend on `dice_roll`
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //in case it is not 3 or 7
        other => move_player(other),
    }

    
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

// convert type of coin to it value
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // we can add more code with brackets
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // print state if coin is quarter
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}