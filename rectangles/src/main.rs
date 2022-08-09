#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    
    println!("rect1 is {:?}", rect1);

    dbg!(&rect1);
}
// here fn doesn't take the ownership of dimensions, insead it copy the value in rect1 to
// new tuple dimensions, since dimensions only contain scalar variable
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
