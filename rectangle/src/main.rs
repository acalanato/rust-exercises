//example program using structs

#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

fn main() {
    //using simple values
    let width1 = 30;
    let height1 = 50;
    println!(
	"The area of the rectangle is {} square pixels.",
	area(width1, height1)
    );
    
    //using tuple
    let rect1 = (30, 50);
    println!(
	"The area of the rectangle is {} square pixels.",
	area_t(rect1)
    );

    //using struct
    let rect2 = Rectangle {
	w: 30,
	h: 50,
    };
        println!(
	"The area of the rectangle is {} square pixels.",
	area_s(&rect2)
    );

    let rect3 = Rectangle {
	w: 30,
	h:50,
    };

    println!("rect3 is {:#?}", rect3);
    // another method to debug stuff
    dbg!(&rect1);
    
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_t(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_s(rectangle: &Rectangle) -> u32{
    rectangle.w * rectangle.h
}
