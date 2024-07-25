use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of the htreads in the pool.
    /// 
    /// # Panics
    /// 
    /// The 'new' function will panic id the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel(); // Create a channel to send the jobs to the workers.

        let receiver = Arc::new(Mutex::new(receiver)); // Wrap the receiver in a Mutex and an Arc to make it thread safe.

        let mut workers = Vec::with_capacity(size); // Create a vector to hold the threads.

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); // Create the threads and store them in the vector.
        }
        
        ThreadPool{ workers, sender } // Return the ThreadPool.
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f); // Create a new job.
        self.sender.send(job).unwrap(); // Send the job to the worker.
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap(); // Get the job from the receiver.

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread } // Return the Worker.
    }
}