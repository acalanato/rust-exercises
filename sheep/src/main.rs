
fn main() {
    assert_eq!(count_sheep(0), "");
    assert_eq!(count_sheep(1), "1 sheep...");
    assert_eq!(count_sheep(2), "1 sheep...2 sheep...");
    assert_eq!(count_sheep(3), "1 sheep...2 sheep...3 sheep...");
 
    println!("Sucess!");

}

fn count_sheep(n: u32) -> String {
    let mut _msg = String::new();
    let mut out = String::new();
    if n == 0 {
        return out;
    } else {
        for mut x in 0..n {
            x += 1;
            _msg = x.to_string() + " sheep...";
            out.push_str(&_msg);
        }
        return out;
    }
}

/*
fn count_sheep(n: u32) -> String {
    (1..=n).map(|x| format!("{} sheep...", x)).collect()
}
*/
