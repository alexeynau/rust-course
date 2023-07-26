use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};
// manage threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
//type of job that the threads do ut should be a closure send and have endless lifetime
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: u64) -> ThreadPool {
        assert!(size > 0);
        //define tx and rx
        let (sender, receiver) = mpsc::channel();
        //create value to reciver
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size.try_into().unwrap());

        for id in 0..size {
            workers.push(Worker::new(id.try_into().unwrap(), Arc::clone(&receiver)));
        }
        //return new instanse of struct
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    //take 1 thread and give job for it
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        //it is job that recirver will be do
        let job = Box::new(f);
        //send work to any not busy reciver
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    //override drop dor our struct
    fn drop(&mut self) {
        //deop sender of threadpool
        drop(self.sender.take());
        //shut down all recivers in this pool
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                //shut down the thread
                thread.join().unwrap();
            }
        }
    }
}

/// do job iniths thread
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        //create new thread and lock the mutex
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    //do work that the sender send to this thread
                    job();
                }
                // if no message shut down the thread
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// this function simulate work of workers
pub fn model(time_matrix: &Vec<Vec<u64>>, number_of_workers: u64) -> u64 {
    let mut sum_of_job = 0;
    let pool = ThreadPool::new(number_of_workers);

    for row in time_matrix {
        for element in row {
            let time = element.clone();
            sum_of_job += time;
            pool.execute(move || {
                do_hay_harvesting(time);
            });
        }
    }
    println!("sum: {sum_of_job}");
    if (sum_of_job % number_of_workers == 0) {
        sum_of_job / number_of_workers
    } else {
        sum_of_job / number_of_workers + 1
    }
}

// this function simulate a job
fn do_hay_harvesting(time: u64) {
    println!("sleep for {time} seconds");
    thread::sleep(Duration::from_secs(time));
}
