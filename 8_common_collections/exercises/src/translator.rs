pub fn to_pig_latin(str: &str) -> String  {
    // all vowels
    let vowels = "aeiouAEIOU";
    // get first char
    let first = str.as_bytes()[0] as char;
    // get str without first char
    let without_first = &str[1..];

    // if first char is vowel
    if vowels.contains(first) {
        //return `{str}-hay`
        format!("{str}-hay")
    } else {   
        //return `{without_first}-{first}ay`
        format!("{without_first}-{first}ay")
    }

}