use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    // opening file give a Result<T, E>
    let greeting_file_result = File::open("hello.txt");

    // matching a Result
    let greeting_file = match greeting_file_result {
        // File was found
        Ok(file) => file,
        // Problem with file
        Err(error) => match error.kind() {
            // If file wasnt found create it
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                // if there is a problem with creating a file throw error
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // if there is something other problem
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // this code do the same but without match control flow
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // open the file
    let greeting_file = File::open("hello.txt")
        // else this message would be trown
        .expect("hello.txt should be included in this project");
}

// this function read username from file "hello.txt"
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        // Throws a error if file wasnt opened
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    // return a Result<T,E>
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
// this function read username from file "hello.txt"
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username = String::new();
    // operator `?` throws a error in case of Result::Err 
    File::open("hello.txt")?.read_to_string(&mut username)?;
    // return username if no errors
    Ok(username)
}

// The shortest way to read string from file
fn read_username_from_file_3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// find last char jf first line
fn last_char_of_first_line(text: &str) -> Option<char> {
    // return None if no lines
    text.lines().next()?.chars().last()
}