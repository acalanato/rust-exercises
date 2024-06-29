
fn nbr_of_laps (x: u16, y: u16) -> (u16, u16) {
    let xv: Vec<u16> = (0..=u16::MAX).step_by(x as usize).collect();
    let yv: Vec<u16> = (0..=u16::MAX).step_by(y as usize).collect();
    let mut i: u16 = 0;
    loop {
        i += 1;
        if xv.contains(&i) & yv.contains(&i) {break}
    }
    (i / x, i / y)
}
/*

fn nbr_of_laps (x: u16, y: u16) -> (u16, u16) {
    let mut i = 1;
    while x * i != y * i {
        i += 1;
        if i > 100 {break}
    }
    (i as u16, 0)
}


fn nbr_of_laps (x: u16, y: u16) -> (u16, u16) {
    let mut mdc: u16 = 1;
    let divide = |a, b| a / b;
    while divide(x, mdc) != divide(y, mdc){
        mdc += 1
    }
    (x.div(mdc), y.div(mdc))
}


fn nbr_of_laps (x: u16, y: u16) -> (u16, u16) {
    let xv: Vec<u16> = (1..=50).step_by(x as usize).collect();
    let yv: Vec<u16> = (1..=50).step_by(y as usize).collect();
    
    (xv[1],xv[3])
}

fn nbr_of_laps (x: u16, y: u16) -> (u16, u16) {
    let s = |s| s + s;
    let mut i = 0;
    let mut out = (x, y);
    loop {
        out.0 = s(out.0);
        out.1 = s(out.1);
        i += 1;
        if i > i {break};
    }
    return (out.0, out.1);
}
*/
fn main() {
    assert_eq!(nbr_of_laps(5, 3), (3, 5));
    assert_eq!(nbr_of_laps(4, 6), (3, 2));
    assert_eq!(nbr_of_laps(5, 5), (1, 1));
    
    println!("Sucess!");
}
