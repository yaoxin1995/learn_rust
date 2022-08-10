
mod front_of_house; // declaration

pub use crate::front_of_house::hosting;

/*
We’ll show two ways to call the add_to_waitlist function from a new 
function eat_at_restaurant defined in the crate root. The eat_at_restaurant 
function is part of our library crate’s public API, so we mark it with the pub keyword
*/

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}