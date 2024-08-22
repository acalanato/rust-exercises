fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {"Even"} else {"Odd"}
}

fn main() {

    assert_eq!(even_or_odd(2), "Even");
    assert_eq!(even_or_odd(1),"Odd");
    println!("{}", even_or_odd(1));
    println!("{}", even_or_odd(0));

}
