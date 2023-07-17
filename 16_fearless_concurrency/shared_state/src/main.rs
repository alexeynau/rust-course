use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    //create mutex  
    let m = Mutex::new(5); 

    //pick the value from mutex
    {
        let mut num = m.lock().unwrap();
        *num = 6;//change value in mutex
    }
    //print value in mutex
    println!("m = {:?}", m);
    //create mutex
    let counter = Arc::new(Mutex::new(0));
    //create vector of threads
    let mut handles = vec![];

    for _ in 0..10 {
        //clone our counter
        let counter = Arc::clone(&counter);
        //in every iteration we spawn new thread
        let handle = thread::spawn(move || {
            //pick the calue from mutex
            let mut num = counter.lock().unwrap();
            //change it and then lock it
            *num += 1;
        });
        //push the thread in vector
        handles.push(handle);
    }




    for handle in handles {
        handle.join().unwrap();//wait when all threads do their work
    }

    println!("Result: {}", *counter.lock().unwrap());

}
