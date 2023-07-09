use std::collections::HashMap;

fn main() {
    // creating a hashmap
    let mut scores = HashMap::new();

    // inserting in hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // getting value by key
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    // iterating in hashmap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }


    // example of dropping ownership after insert
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // printig map
    println!("{:?}", scores);

    //Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // count every letter in text
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}