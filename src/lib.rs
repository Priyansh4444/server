use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, Thread},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to All workers");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers!");
        for worker in &mut self.workers {
            println!("Shutting down {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
            // worker is a mutable reference but join takes ownership so we wrap it in Option
            // and take method on thread to take the join handle out of the optional and replace it None
        }
    }
}
struct Worker {
    id: usize,
    // Simple for debugging which worker is doing wrong
    thread: Option<thread::JoinHandle<()>>,
    // Adds multiple threads (we also make this so that we dont directly use up all threads)
}

type Job = Box<dyn FnOnce() + Send + 'static>;
// Makes use of a trait object to ensure that all types of job can be passed

impl ThreadPool {
    /// Create a new threadpool
    ///
    /// The size is the number of threads in the pool
    ///
    /// panics if size is 0
    pub fn new(num: usize) -> ThreadPool {
        assert!(num > 0);
        let mut workers = Vec::with_capacity(num);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..num {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
        // send job down the channel and get a result type
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            // mutex -> result -> recieving -> Result

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; Executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} got stopped job; bye!", id);
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
