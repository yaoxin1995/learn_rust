use std::ops::{Deref, DerefMut};

// The Box<T> type is ultimately defined as a tuple struct with one element, 
// so Listing 15-8 defines a MyBox<T> type in the same way. We’ll also define
//  a new function to match the new function defined on Box<T>.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {

    fn deref_mut(&mut self) -> &mut Self::Target{
        &mut self.0
    }
}


fn hello(name: &mut str) {
    println!("Hello, {name}!");
}

fn hello1(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);   // *(y.deref())

    let mut m = MyBox::new(String::from("Rust"));
    

    hello1(&m);   // Deref Coercion with Deref

    hello(&mut m);   // Deref Coercion with DerefMut

    hello1(&mut m);   // Deref Coercion with Deref









}