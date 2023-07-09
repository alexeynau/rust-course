fn main() {
    //creating mutable string
    let mut s = String::new();

    let data = "initial contents";

    // converting &str to String
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    // create a String from a string literal
    let s = String::from("initial contents");

    // can include any properly encoded data in String
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // appending to String
    let mut s = String::from("foo");
    s.push_str("bar"); // "foobar"

    // appending to String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    // appending to String one char
    let mut s = String::from("lo");
    s.push('l');

    // concatenation using `+`
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // concatenation using `format!` macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    //using slice instead of indexing
    let hello = "Здравствуйте";
    let s = &hello[0..4]; //use carefully

    // iterating in chars
    for c in "Зд".chars() {
        println!("{}", c);
    }
    // iterating in bytes
    for b in "Зд".bytes() {
        println!("{}", b);
    }

    
}
