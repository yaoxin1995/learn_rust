// we bring the std::env module into scope with a use statement 
use std::env;

use std::process;


use minigrep::Config;


fn main() {
    // env::args() returns an iterator of the command line arguments passed to minigrep
    // iterators produce a series of values, and we can call the collect method on an iterator 
    // to turn it into a collection, such as a vector, that contains all the elements the 
    // iterator produces.
    let args: Vec<String> = env::args().collect(); 
    // Finally, we print the vector using the debug formatter, :?
    println!("{:?}", args);

    // In this listing, we’ve used a method we haven’t covered in detail yet: 
    // unwrap_or_else, which is defined on Result<T, E> by the standard library. 
    // Using unwrap_or_else allows us to define some custom, non-panic! error handling. 
    // If the Result is an Ok value, this method’s behavior is similar to unwrap: 
    // it returns the inner value Ok is wrapping. However, if the value is an Err value, 
    // this method calls the code in the closure, which is an anonymous function we 
    // define and pass as an argument to unwrap_or_else. For now, you just need to 
    // know that unwrap_or_else will pass the inner value of the Err, which in this case 
    // is the static string "not enough arguments" in build func, 
    // to our closure in the argument err that appears between the vertical pipes.
    //  The code in the closure can then use the err value when it runs.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // We use if let rather than unwrap_or_else to check whether run returns an Err 
    // value and call process::exit(1) if it does. The run function doesn’t return a 
    // value that we want to unwrap in the same way that Config::build returns the Config 
    // instance. Because run returns () in the success case, we only care about detecting 
    // an error, so we don’t need unwrap_or_else to return the unwrapped value, which would 
    // only be ().

    // use the prefix minigrep:: to call function in lib.rs
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");

        process::exit(1);
    }
}



