
fn _points(games: &[String]) -> u32 {
    let mut score: u32 = 0;    
    for x in games {
	let (a, b) = x.split_at(1);
	if a.cmp(b) == std::cmp::Ordering::Greater {
	    score += 3
	} else if a.eq(b) {
	    score += 1
	} else { continue }
    }
    score
}

fn points(games: &[String]) -> u32 {
    use std::cmp::Ordering;
    let mut score: u32 = 0;
    
    for x in games {
	let (a, b) = x.split_at(1);
	match a.cmp(b) {
	    Ordering::Greater => score += 3_u32,
	    Ordering::Equal => score += 1_u32,
	    _ => continue,
	}
    }
    score
}

fn main() {
    let score = points(&["1:0".to_string(),
			 "2:0".to_string(),
			 "3:0".to_string(),
			 "4:0".to_string(),
			 "2:1".to_string(),
			 "3:1".to_string(),
			 "4:1".to_string(),
			 "3:2".to_string(),
			 "4:2".to_string(),
			 "4:3".to_string(),
    ]);

    println!("{:?}", "3".cmp("2"));
    println!("{}", score);
    //println!("\x1B[1;34m{}", score);
}


#[cfg(test)]
mod tests {
    use super::points;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn do_fixed_test(e: &[&str], expected: u32) {
        let a = &e.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(points(a), expected, "{ERR_MSG} with games = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        do_fixed_test(&["1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3"], 30);
        do_fixed_test(&["1:1", "2:2", "3:3", "4:4", "2:2", "3:3", "4:4", "3:3", "4:4", "4:4"], 10);
        do_fixed_test(&["0:1", "0:2", "0:3", "0:4", "1:2", "1:3", "1:4", "2:3", "2:4", "3:4"], 0);
        do_fixed_test(&["1:0", "2:0", "3:0", "4:0", "2:1", "1:3", "1:4", "2:3", "2:4", "3:4"], 15);
        do_fixed_test(&["1:0", "2:0", "3:0", "4:4", "2:2", "3:3", "1:4", "2:3", "2:4", "3:4"], 12);
    }
}
