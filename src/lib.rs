use std::{sync::{mpsc, Arc, Mutex}, thread::{self, Thread}};


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker{
    id:usize,
    // Simple for debugging which worker is doing wrong
    thread:thread::JoinHandle<()>,
    // Adds multiple threads (we also make this so that we dont directly use up all threads)
}

type Job = Box<dyn FnOnce() +Send +'static>;
// Makes use of a trait object to ensure that all types of job can be passed

impl ThreadPool{
    /// Create a new threadpool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// panics if size is 0
    pub fn new(num: usize) -> ThreadPool{
        assert!(num > 0);
        let mut workers = Vec::with_capacity(num);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..num{
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        ThreadPool{workers, sender}
        
    }
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
        // send job down the channel and get a result type

    }
}

impl Worker{
    fn new(id:usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move || loop
            {
                let job = receiver.lock().unwrap().recv().unwrap();
                // mutex -> result -> recieving -> Result
                println!("Worker {} got a job; Executing.",id);
                job();
            });
        Worker {id, thread}
    }
}