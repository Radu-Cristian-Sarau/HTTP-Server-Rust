use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

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
        self.sender.send(Message::NewJob(job)).unwrap(); // Send the job to the worker.
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap(); // Send the terminate message to all workers.
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap(); // Shut down the worker.
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap(); // Get the job from the receiver.

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }

            }
        });
        Worker { id, thread: Some(thread) } // Return the Worker.
    }

}