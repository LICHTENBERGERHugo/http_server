use std::{thread, sync::mpsc};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<()>,
}
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
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool { workers , sender }
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize,receiver: Arc<Mutex<Receiver<()>>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {
                receiver.lock().unwrap().recv().unwrap();
            }),
        }
    }
}

struct Job;