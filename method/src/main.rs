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
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
	Self {
	    width: size,
	    height: size,
	}
    }
}

fn main() {
    let rect1 = Rectangle {
	width: 30,
	height: 50,
    };
    let rect2 = Rectangle {
	width: 10,
	height: 40,
    };
    let rect3 = Rectangle {
	width: 60,
	height: 45,
    };

    println!(
	"The area of the rectangle is {} square pixels.",
	rect1.area()
    );

    let test = |rect1: &Rectangle| -> u32 {rect1.width * rect1.height};
    println!("Using the anonymous function syntax to calculate: {}", test(&rect1));
    
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));
    let sq = &Rectangle::square(15);
    println!("Area of a square is {}",sq.area());

}
