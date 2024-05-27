#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
	self.width * self.height
    }
    fn width(&self) -> bool {
        self.width() > 0
    }
}


fn main() {
    let rect1 = Rectangle {
	width: 30,
	height: 50,
    };

    println!(
	"The area of the rectangle is {} square pixels.",
	rect1.area()
    );

    let test = |rect1: Rectangle| -> u32 {rect1.width * rect1.height};
    println!("Using the anonymous function syntax to calculate: {}", test(rect1));
    
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
