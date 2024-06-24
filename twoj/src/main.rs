
fn nbr_of_laps (x: u16, y: u16) -> (u16, u16) {
    let s = |s| s + s;
//    for x in [..=10] {todo!()};
    let mut out = (x, y);
    loop {
        out.0 = s(out.0);
        out.1 = s(out.1);
        if out.0 < u16::MAX {break};
    }
    return (out.0, out.1);
}

fn main() {
    assert_eq!(nbr_of_laps(5, 3), (3, 5));
    assert_eq!(nbr_of_laps(4, 6), (3, 2));
    assert_eq!(nbr_of_laps(5, 5), (1, 1));
    
    println!("Sucess!");
}
