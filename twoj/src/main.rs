fn nbr_of_laps (x: u16, y: u16) -> (u16, u16) {
    let _is_prime = |a: u16| if (a > 1) & (a < 4) {true;} else if a % 2 == 0 {false;};
    //let n = |m| for n in 0..=10 {m * n};
    //let n = (0..=10).map(|n| n * m).collect();
    //let mut m: Vec<u16> = (0..=10).collect();
    //for n in m.iter() {m * n};
    (x, y)
}


/*

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

*/
fn main() {
    /*
    assert_eq!(nbr_of_laps(5, 3), (3, 5));
    assert_eq!(nbr_of_laps(4, 6), (3, 2));
    assert_eq!(nbr_of_laps(5, 5), (1, 1));
    let  m: Vec<u16> = (0..=10).collect();
    let o = 3;
    let n: Vec<_> = m.iter().map(|x| x * o).collect();
     */
    let n = |m: u16| (0..=10).map(move |n| m * n).collect::<Box<[_]>>();
    
    println!("Sucess!{:?}", n(3)[5]);
}
