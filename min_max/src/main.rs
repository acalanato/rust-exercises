fn main() {
    let a_min = minimum(&[-52, 56, 30, 29, -54, 0, -110]);
    let a_max = maximum(&[-52, 56, 30, 29, -54, 0, -110]);
    let b_min = minimum(&[42, 54, 65, 87, 0]);
    let b_max = maximum(&[42, 54, 65, 87, 0]);
    let c_min = minimum(&[1, 2, 3, 4, 5, 10]); // 1, 10);
    let c_max = maximum(&[1, 2, 3, 4, 5, 10]); // 1, 10);
    let d_min = minimum(&[-1, -2, -3, -4, -5, -10, 534, 43, 2, 1, 3, 4, 5, 5, 443, 443, 555, 555]); // -10, 555);
    let d_max = maximum(&[-1, -2, -3, -4, -5, -10, 534, 43, 2, 1, 3, 4, 5, 5, 443, 443, 555, 555]); // -10, 555);
    let e_min = minimum(&[9]); // 9, 9);
    let e_max = maximum(&[9]); // 9, 9);
    let f_min = minimum(&[4,6,2,1,9,63,-134,566]); // -134, 566);
    let f_max = maximum(&[4,6,2,1,9,63,-134,566]); // -134, 566);
    print!("
min:\t max
{a_min}\t {a_max}
{b_min}\t {b_max}
{c_min}\t {c_max}
{d_min}\t {d_max}
{e_min}\t {e_max}
{f_min}\t {f_max}
");
}

fn minimum(arr: &[i32]) -> i32 {
    return *arr.iter().min().unwrap();
}

fn maximum(arr: &[i32]) -> i32 {
    return *arr.iter().max().unwrap();
}
