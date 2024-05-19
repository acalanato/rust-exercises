
fn main() {
    
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
    let b = vec![];
    let c = vec![0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14];
    let d = vec![0,1,2,3,4,5];
    let e = vec![1,2,3,4,5];
    let f = vec![0,-1,-2,-3,-4,-5];
    let g = vec![-1,-2,-3,-4,-5];
    let h = vec![0,0,0,0];
    let i = vec![0];

    assert_eq!(count_positives_sum_negatives(a), [10, -65]);
    assert_eq!(count_positives_sum_negatives(b), []); // fail
    assert_eq!(count_positives_sum_negatives(c), [8, -50]);
    assert_eq!(count_positives_sum_negatives(d), [5, 0]);
    assert_eq!(count_positives_sum_negatives(e), [5, 0]);
    assert_eq!(count_positives_sum_negatives(f), [0, -15]);
    assert_eq!(count_positives_sum_negatives(g), [0, -15]);
    assert_eq!(count_positives_sum_negatives(h), [0, 0]);
    assert_eq!(count_positives_sum_negatives(i), [0, 0]);


    let x = vec![];
    let y = count_positives_sum_negatives(x);
    println!("{:?}", y);
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
    if out[0] + out[1] == 0 && out.len() < 2 {
        return vec![];
    } else {
        return out;
    }
}
