fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
       
        println!("X equals to {} and Y equals to {}", x, y);
    }
    println!("Outside of scope, X equals to {} and Y equals to {}", x, y);
}
