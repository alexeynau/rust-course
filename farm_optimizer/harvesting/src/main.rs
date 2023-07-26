use harvesting::ThreadPool;
use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};
fn main() {
    let number_of_columns = 5;
    let number_of_rows = 5;
    let time_matrix = [
        [9, 8, 1, 6, 5],
        [1, 8, 7, 6, 5],
        [6, 5, 4, 3, 9],
        [7, 3, 9, 4, 4],
        [4, 1, 1, 8, 9],
    ];
    let number_of_workers = 10;
    // let counter = Arc::new(Mutex::new(0));
    // let counter = Arc::new(RwLock::new(0));
    let pool = ThreadPool::new(number_of_workers);

    for row in time_matrix {
        for element in row {
            let time = element.clone();
            pool.execute( move || {
                do_hay_harvesting(time);
            });
        }
    }

    println!("done!");
}

fn do_hay_harvesting(time: u64) {
    println!("sleep for {time} seconds");
    thread::sleep(Duration::from_secs(time));
    // for i in 0..time {
    //     let mut counter = counter.write().unwrap();
    //     *counter +=1;
    //     thread::sleep(Duration::from_secs(1));
    // }
}
