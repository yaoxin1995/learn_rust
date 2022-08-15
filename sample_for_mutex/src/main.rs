use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //  create a Mutex<T> using the associated function new
    // The type of m is Mutex<i32>, not i32, so we must call lock to be 
    // able to use the i32 value. We can’t forget; the type system won’t 
    // let us access the inner i32 otherwise.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
        // To access the data inside the mutex, 
        // we use the lock method to acquire the lock
        // This call will block the current thread so 
        // it can’t do any work until it’s our turn to have the lock.

        // After we’ve acquired the lock, we can treat the return value, 
        // named num in this case, as a mutable reference to the data inside

        // As you might suspect, Mutex<T> is a smart pointer. More accurately, 
        // the call to lock returns a smart pointer called MutexGuard, wrapped 
        // in a LockResult that we handled with the call to unwrap. The MutexGuard 
        // smart pointer implements Deref to point at our inner data; the smart 
        // pointer also has a Drop implementation that releases the lock 
        // automatically when a MutexGuard goes out of scope, which happens at 
        // the end of the inner scope. As a result, we don’t risk forgetting to 
        // release the lock and blocking the mutex from being used by other 
        // threads, because the lock release happens automatically.
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}