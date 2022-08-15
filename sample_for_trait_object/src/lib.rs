#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub trait Draw {
    fn draw(&self);
}

//  struct named Screen that holds a vector named components. 
// This vector is of type Box<dyn Draw>, which is a trait object; 
// it’s a stand-in for any type inside a Box that implements the Draw trait.

// This works differently from defining a struct that uses a generic type 
// parameter with trait bounds. A generic type parameter can only be 
// substituted with one concrete type at a time, whereas trait objects 
// allow for multiple concrete types to fill in for the trait object at 
// runtime. 
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}


impl Screen {
    // On the Screen struct, we’ll define a method named 
    // run that will call the draw method on each of its components
    pub fn run(&self) {
        // iter(), which iterates over &T.
        for component in self.components.iter() {
            component.draw();
        }
    }
}


// add some types that implement the Draw trait
#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("{:?}", self);
    }
}