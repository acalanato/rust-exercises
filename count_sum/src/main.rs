//use std::iter;
use std::fmt::Debug;

fn main() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
    let y = count_positives_sum_negatives(x);
    writeln!("{:?}", y);
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut out = vec![0, 0];
    let input: Vec<i32> = Vec::from(input);
    for x in input.iter() {
        if x > &0 {
            out[0] += 1;
        } else {
            out[1] += x;
        };
    }
    if out[0] == 0 && out[1] == 0 && out.len() > 2 {
        return vec![];
    } else {
        return out;
    }
}

