use std::sync::mpsc;
use std::thread;

fn main() {
    // We create a new channel using the mpsc::channel function; 
    // mpsc stands for multiple producer, single consumer
    // The mpsc::channel function returns a tuple, the first element of which is the sending end--
    // the transmitter--and the second element is the receiving end--the receiver
    let (tx, rx) = mpsc::channel();

    //  move the transmitting end into a spawned thread and have it send 
    // one string so the spawned thread is communicating with the main thread
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // The receiver has two useful methods: recv and try_recv. 
    // We’re using recv, short for receive, which will block the main 
    // thread’s execution and wait until a value is sent down the channel. 
    // Once a value is sent, recv will return it in a Result<T, E>. When 
    // the transmitter closes, recv will return an error to signal that 
    // no more values will be coming

    // The try_recv method doesn’t block, but will instead return a 
    // Result<T, E> immediately: an Ok value holding a message if one 
    // is available and an Err value if there aren’t any messages this time.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}