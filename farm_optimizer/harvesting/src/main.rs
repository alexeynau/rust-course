use harvesting::{model, ThreadPool};
use std::{
    io,
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
    vec, array,
};
fn main() {
    // let number_of_columns = 5;
    // let number_of_rows = 5;
    // let time_matrix= vec![
    //     vec![9, 8, 1, 6, 5],
    //     vec![1, 8, 7, 6, 5],
    //     vec![6, 5, 4, 3, 9],
    //     vec![7, 3, 9, 4, 4],
    //     vec![4, 1, 1, 8, 9],
    // ];
    // let number_of_workers: usize = 10;
    let mut time_matrix = Vec::new();
    let number_of_workers = get_input(&mut time_matrix);
    println!("{number_of_workers}");
    let result = model(&time_matrix, number_of_workers);

    println!("Done! Time: {}", result);
}

/// this function read time matrix
fn get_input( time_matrix : &mut Vec<Vec<u64>>) ->  u64 {
    let number_of_columns = read_int("number_of_columns");
    let number_of_rows = read_int("number_of_rows");
    let number_of_workers = read_int("number_of_workers");

    let mut row_vector = Vec::<u64>::new();

    for i in 0..number_of_rows {
        for j in 0..number_of_columns {
           let element = read_int(&format!("{i}.{j} element")[..]);
           row_vector.push(element);
        }
        time_matrix.push(row_vector.clone());
    }
    number_of_workers
}

/// This function read integer from console
fn read_int(name: &str) -> u64 {
    loop {
        let mut input = String::new();
        println!("Please input {name}.");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read {name}");

        let result: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Failed to parse integer");
                continue;
            }
        };
        return result;
    }
}
