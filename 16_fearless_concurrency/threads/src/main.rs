use std::thread;
use std::time::Duration;

fn main() {
    // crate new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // loop in main thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // wait for another thread done
    handle.join().unwrap();


    let v = vec![1, 2, 3];
    // we use `move` to allow closure borrow vector 
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

