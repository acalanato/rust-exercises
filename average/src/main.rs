
fn main() {
    find_average([
        17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0
    ]);
    println!("{}", x)
}

fn find_average(slice: &mut [f64; usize]) -> f64 {
    let i = 0;
    for x in slice {
        x = x + slice[i];
        i += 1;
    }
    return slice;
}

