
fn main() {
    let x = find_average(&[
        17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
    ]);
    println!("{}", x)
}

fn find_average(slice: &[f64]) -> f64 {
    let slice_i = slice.iter();
    for x in slice_i {
        x = &(x + slice[x]);
    }
    return slice;
}

