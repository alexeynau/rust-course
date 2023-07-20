use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
// that struct give jobs its thread, manage threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
// type name of job that thread do. Should be a clousure implements Send trait
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // get channel reciver and sender
        let (sender, receiver) = mpsc::channel();
        // to share ownership across multiple threads and allow the threads to mutate the value
        // we need to use Arc<Mutex<T>>
        let receiver = Arc::new(Mutex::new(receiver));
        // init workers with size given in params
        let mut workers = Vec::with_capacity(size);
        // push new worker in workers with clonned reciever
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        // return new thread pool
        ThreadPool { workers, sender }
    }
    /// Take a job given in params and send it to thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // takes clousure
    {
        // create a job from clousure
        let job = Box::new(f);
        // send a job through a sender
        self.sender.send(job).unwrap();
    }
}
/// Take a job and do it
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}


impl Worker {
    /// Create a new `Worker` with id and reciver given in params
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        /// create a thread
        let thread = thread::spawn(move || loop { // infinite loop
            // wait for getting a new job
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");
            // do job
            job();
        });
        // return worker
        Worker { id, thread }
    }
}
