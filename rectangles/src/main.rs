
fn main() {
    let rect1 = (30, 50);

    println! (
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

}

// here fn doesn't take the ownership of dimensions, insead it copy the value in rect1 to
// new tuple dimensions, since dimensions only contain scalar variable
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
