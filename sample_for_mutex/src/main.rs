use std::sync::Mutex;
use std::thread;

fn main() {
    //  create a Mutex<T> using the associated function new
    // The type of m is Mutex<i32>, not i32, so we must call lock to be 
    // able to use the i32 value. We can’t forget; the type system won’t 
    // let us access the inner i32 otherwise.
    let m = Mutex::new(5);

    {
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
        let mut num = m.lock().unwrap();

        
        *num = 6;
    }

    println!("m = {:?}", m);
}