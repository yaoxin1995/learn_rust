use std::error::Error;
use std::fs;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // Note that the backslash after the opening double quote tells 
        // Rust not to put a newline character at the beginning of the 
        // contents of this string literal)
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

// we tell Rust that the data returned by the search function will 
// live as long as the data passed into the search function in the contents argument
//  by defining an explicit lifetime 'a in the signature of search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let mut results = Vec::new();

    //  helpful method to handle line-by-line iteration of strings, conveniently named lines
    //for line in contents.lines() {
        // do something with line
    //    if line.contains(query) {
    //        results.push(line);
    //    }

    //}

    //results


    //  Similar to the filter example in Listing 13-16, this code uses the filter adaptor 
    // to keep only the lines that line.contains(query) returns true for. We then collect 
    // the matching lines into another vector with collect. Much simpler! Feel free to make 
    // the same change to use iterator methods in the search_case_insensitive function as 
    // well
    contents
        .lines()
            .filter(|line| line.contains(query))
                .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // Note that query is now a String rather than a string slice, 
    // because calling to_lowercase creates new data rather than 
    // referencing existing data
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        //  When we pass query as an argument to the contains 
        // method now, we need to add an ampersand because the 
        // signature of contains is defined to take a string slice.
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /**
     *  We can create instances of types in the standard library, 
     *  such as String, by calling String::new. Similarly, by changing 
     *  parse_config into a new function associated with Config, we’ll be 
     *  able to create instances of Config by calling Config::new. Listing 12-7 
     *  shows the changes we need to make.
     */

     // The standard library documentation for the env::args 
     // function shows that the type of the iterator it returns is 
     // std::env::Args, and that type implements the Iterator trait 
     // and returns String values. We’ve updated the signature of the Config::build 
     // function so the parameter args has a generic type with the trait bounds impl
     // Iterator<Item = String> instead of &[String]. This usage of the impl
     //  Trait syntax we discussed in the “Traits as Parameters” section of
     // Chapter 10 means that args can be any type that implements the Iterator
     //  type and returns String items
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        args.next(); //  ignore the first value in the return value of env::args
                     // which is the name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
            
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
            

        
        // Here, we create a new variable ignore_case. To set its value, 
        // we call the env::var function and pass it the name of the IGNORE_CASE 
        // environment variable. The env::var function returns a Result that will 
        // be the successful Ok variant that contains the value of the environment 
        // variable if the environment variable is set to any value. It will return 
        // the Err variant if the environment variable is not set.

        // We’re using the is_ok method on the Result to check whether the environment 
        // variable is set, which means the program should do a case-insensitive search. 
        // If the IGNORE_CASE environment variable isn’t set to anything, is_ok will 
        // return false and the program will perform a case-sensitive search. We 
        // don’t care about the value of the environment variable, just whether it’s 
        // set or unset, so we’re checking is_ok rather than using unwrap, expect, 
        // or any of the other methods we’ve seen on Result.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case})
    }
}



// For the error type, we used the trait object Box<dyn Error> 
// (and we’ve brought std::error::Error into scope with a use statement at the top). 
// We’ll cover trait objects in Chapter 17. For now, just know that 
// Box<dyn Error> means the function will return a type that implements 
// the Error trait, but we don’t have to specify what particular type the 
// return value will be. This gives us flexibility to return error values 
// that may be of different types in different error cases. The dyn keyword is 
// short for “dynamic.”
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // fs::read_to_string takes the filename, opens that file, 
    // and returns a Result<String> of the file’s contents

    // Second, we’ve removed the call to expect in favor of the ? operator, 
    // as we talked about in Chapter 9. Rather than panic! on an error, 
    // ? will return the error value from the current function for the 
    // caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // Third, the run function now returns an Ok value in 
    // the success case. We’ve declared the run function’s 
    // success type as () in the signature, which means we need 
    // to wrap the unit type value in the Ok value. This Ok(()) 
    // syntax might look a bit strange at first, but using () 
    // like this is the idiomatic way to indicate that we’re 
    // calling run for its side effects only; it doesn’t return a value we need.
    Ok(())
}
