fn main() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
    println!("Pass");
}

fn square_sum(vec: Vec<i32>) -> i32 {
    let mut out: i32 = 0;
    for x in vec.iter() {
        out = out + x.pow(2)
    };
    return out;
}

/*
fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|s| s * s).sum()
}
*/
