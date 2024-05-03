fn main() {
    let x: i32 = 5; //uninnitialized but used, ERROR!
    let _y: i32; // uninnitialized but also unused, just a warning

    assert_eq!(x, 5);
    println!("Sucess!");
}
