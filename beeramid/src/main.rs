fn beeramid (bonus: i32, price: f32) -> usize {

    let mut nbr_cans = 0.0;
    let mut i = 0.0;

    
    if bonus > 0 && price > 0.0 {
        nbr_cans = (bonus as f32 / price).abs();
    } else { return 0 }

    loop {
	nbr_cans = nbr_cans - i*i;
	if nbr_cans < 0.0 {break}
	i += 1.0;
    }
    i as usize - 1_usize
}

/*
fn beeramid(bonus: i32, price: f32) -> usize {
    let max_count = (bonus as f32 / price) as i32;
    (1..)
        .scan(0, |acc, x| {
            *acc += x * x;
            Some(*acc)
        })
        .take_while(|&x| x <= max_count)
        .count()
}
*/

fn main() {
    let bee = beeramid(5000, 0.0);
    println!("{}", bee);
    println!("Sucess!");
}


#[cfg(test)]
mod tests {
    use super::beeramid;

    #[test]
    fn sample_tests() {
        assert_eq!(beeramid(9, 2.0), 1);
        assert_eq!(beeramid(10, 2.0), 2);
        assert_eq!(beeramid(11, 2.0), 2);
        assert_eq!(beeramid(21, 1.5), 3);
        assert_eq!(beeramid(454, 5.0), 5);
        assert_eq!(beeramid(455, 5.0), 6);
        assert_eq!(beeramid(4, 4.0), 1);
        assert_eq!(beeramid(3, 4.0), 0);
        assert_eq!(beeramid(0, 4.0), 0);
        assert_eq!(beeramid(-1, 4.0), 0);
    }
}
