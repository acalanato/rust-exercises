
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    let mut e = (6..=8).collect();

    v.append(&mut e);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    println!("{:?}", v)
}
