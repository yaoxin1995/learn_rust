use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

        // loop forever, asking the receiving end of the channel for a 
        // job and running the job when it gets one
        let thread = thread::spawn(move || loop {
            // Here, we first call lock on the receiver to acquire the mutex, and 
            // then we call unwrap to panic on any errors. Acquiring a lock might fail if 
            // the mutex is in a poisoned state, which can happen 
            // if some other thread panicked while holding the lock rather than releasing the lock.
            // The call to recv blocks, so if there is no job yet, the current thread will wait until 
            // a job becomes available. The Mutex<T> ensures that only one Worker thread at a time is trying to request a job.
            
            // let, any temporary values used in the expression on the right hand side of the equals sign are 
            // immediately dropped when the let statement ends, so  Mutex will be released by its drop method
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}

// a type alias for a trait object that holds the type of closure that execute receives
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // to share ownership across multiple threads and allow 
        // the threads to mutate the value, we need to use Arc<Mutex<T>>. 
        // The Arc type will let multiple workers own the receiver, and Mutex 
        // will ensure that only one worker gets a job from the receiver at a time. 
        let receiver = Arc::new(Mutex::new(receiver));

        //  The with_capacity function performs the same task as Vec::new 
        // but with an important difference: it preallocates space in the vector. 
        // Because we know we need to store size elements in the vector,
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }


        ThreadPool { workers, sender}
    }

    // The F type parameter also has the trait bound Send and the lifetime 
    // bound 'static, which are useful in our situation: we need Send to 
    // transfer the closure from one thread to another and 
    // 'static because we don’t know how long the thread will take to execute.
    
    // We still use the () after FnOnce because this FnOnce represents a 
    // closure that takes no parameters and returns the unit type (). 
    // Just like function definitions, the return type can be omitted from 
    // the signature, but even if we have no parameters, we still need the parentheses.
    
    
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job : Job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}