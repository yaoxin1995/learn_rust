#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// When we create a new Post, we set its state field to a Some value 
// that holds a Box. This Box points to a new instance of the Draft struct. 
// This ensures whenever we create a new instance of Post, it will start out 
// as a draft. Because the state field of Post is private, there is no way 
// to create a Post in any other state! 
// In the Post::new function, we set the content field to a new, empty String.
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // The add_text method takes a mutable reference to self, because we’re 
    // changing the Post instance that we’re calling add_text on. We then call 
    // push_str on the String in content and pass the text argument to add to 
    // the saved content. This behavior doesn’t depend on the state the post 
    // is in, so it’s not part of the state pattern. The add_text method 
    // doesn’t interact with the state field at all, but it is part of the 
    // behavior we want to support.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str{
        // self.state -> Option<Box<dyn State>>
        // Option<Box<dyn State>>.as_ref() -> Option<&Box<dyn State>>
        // Option<&Box<dyn State>>.content(post) -> (deref coercions) &state.content

        // We call the as_ref method on the Option because we want a reference to the 
        // value inside the Option rather than ownership of the value. Because state 
        // is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> 
        // is returned. If we didn’t call as_ref, we would get an error because we can’t 
        // move state out of the borrowed &self of the function parameter.

        // We then call the unwrap method, which we know will never panic, because we know 
        // the methods on Post ensure that state will always contain a Some value when those 
        // methods are done. This is one of the cases we talked about in the “Cases In 
        // Which You Have More Information Than the Compiler” section of Chapter 9 when we 
        // know that a None value is never possible, even though the compiler isn’t able to 
        // understand that.

        // At this point, when we call content on the &Box<dyn State>, deref coercion will 
        // take effect on the & and the Box so the content method will ultimately be called 
        // on the type that implements the State trait. 
        self.state.as_ref().unwrap().content(self)
    }

    // To consume the old state, the request_review method needs to take 
    // ownership of the state value. This is where the Option in the state 
    // field of Post comes in: we call the take method to take the Some value
    //  out of the state field and leave a None in its place, because Rust 
    // doesn’t let us have unpopulated fields in structs. This lets us move 
    // the state value out of Post rather than borrowing it. Then we’ll set 
    // the post’s state value to the result of this operation.

    // We need to set state to None temporarily rather than setting it 
    // directly with code like self.state = self.state.request_review(); 
    // to get ownership of the state value. This ensures Post can’t use 
    // the old state value after we’ve transformed it into a new state.
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // changes the post’s state from PendingReview back to Draft.
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// private State trait that will define the behavior 
// that all state objects for a Post must have.
// The State trait defines the behavior shared by different post states. 
// The state objects are Draft, 
// PendingReview, and Published, and they will all implement the State
trait State {
    // all types that implement the trait will now need to implement the 
    // request_review method. Note that rather than having self, &self, or 
    // &mut self as the first parameter of the method, we have self: 
    // Box<Self>. This syntax means the method is only valid when called 
    // on a Box holding the type. This syntax takes ownership of Box<Self>, 
    // invalidating the old state so the state value of the Post can transform
    //  into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    // We’re taking a reference to a post as an argument and returning a reference to part of 
    // that post, so the lifetime of the returned reference is related to the lifetime of the 
    // post argument.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}


// The request_review method on Draft returns a new, boxed instance of a new PendingReview struct, 
// which represents the state when a post is waiting for a review. The PendingReview struct also 
// implements the request_review method but doesn’t do any transformations. Rather, it returns 
// itself, because when we request a review on a post already in the PendingReview state, it should 
// stay in the PendingReview state.
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}


struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}