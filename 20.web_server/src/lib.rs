use std::{error::Error,
    fmt,
    thread,
    sync::{mpsc,
        Arc,
        Mutex},};

//  TODO: Implement a Worker builder that gracefully handles a thread creation
//  failure. 

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id : usize,
    thread : thread::JoinHandle<()>,
}

struct Job;

impl Worker {
    fn new(id : usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {id, thread}
    }
}

#[derive(Debug, Clone)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to create ThreadPool")
    }
}

impl Error for PoolCreationError {}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {return Err(PoolCreationError)}

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok(ThreadPool {workers, sender})
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}