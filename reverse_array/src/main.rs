// fn d_ch on line 36 was writen by @bronzelle from discord

fn main() {
    let a = 349587;
    let b = 35231;
    let c = 0;
    let d_a = digitize(a);
    let d_b = digitize(b);
    let d_c = digitize(0);
    let s_a = stringer(a);
    let s_b = stringer(b);
    let s_c = stringer(c);
    print!("
{s_a:?} {d_a:?}
{s_b:?} {d_b:?}
{s_c:?} {d_c:?}

Sugestão do colega:
in  {a}
out {:?}
", d_ch(a));
}

fn stringer(n: u64) -> String {
    return n.to_string().chars().rev().collect();
}

fn digitize(n: u64) -> Vec<u8> {
    let s: String = n.to_string().chars().rev().collect();
    return s.into_bytes();
    //    return x;
//    let out: [u8;5] =  s.try_into().unwrap();
//    return out.to_owned()
}

fn d_ch (n: u64) -> Vec<u8> {
    let s: String = n.to_string();
    s.chars().rev().map(|c| (c as u8) - 48).collect::<Vec<_>>()
}


/*

fn reverse(in: Vec){
    let reverse: [i64; 5] = in.iter().rev().collect();
    
}
fn digitize(n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut _last = 0;
    let mut n = n;
    while n > 0 {
	_last = n % 10;
	out.push((n * 10) + _last);
	n = n / 10;
    }
    return Vec::from(out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(349587), vec![7,9,5,8,4,3]);
        assert_eq!(digitize(35231), vec![1,3,2,5,3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
*/
