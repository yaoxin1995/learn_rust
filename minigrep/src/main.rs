// we bring the std::env module into scope with a use statement 
use std::env;
// handle files.
use std::fs;

fn main() {
    // env::args() returns an iterator of the command line arguments passed to minigrep
    // iterators produce a series of values, and we can call the collect method on an iterator 
    // to turn it into a collection, such as a vector, that contains all the elements the 
    // iterator produces.
    let args: Vec<String> = env::args().collect(); 
    // Finally, we print the vector using the debug formatter, :?
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // fs::read_to_string takes the filename, opens that file, 
    // and returns a Result<String> of the file’s contents
    let contents = fs::read_to_string(filename)
                    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}