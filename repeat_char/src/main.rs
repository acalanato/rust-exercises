
fn accum(s:&str)->String {
    let mut out: Vec<u8> = Vec::new();
    for (i, val) in s.as_bytes().into_iter().enumerate() {
	out.push(val.to_ascii_uppercase());
	for _x in 1..=i {
	    out.push(val.to_ascii_lowercase())
	}
	out.push(b'-');
    }
    out.pop();
    String::from_utf8(out).unwrap()
}

#[allow(unused)]
fn _accum(s:&str)->String {
    let words = s.chars().map(|x| (x.to_string())).collect::<Vec<String>>();
    enum Words {
	Letter
    }
    let mut out: Vec<Vec<String>> = Vec::new();
    for x in 0..s.len() {
	for y in 0..x {
	    out.push([words[x].clone()].to_vec())
	};
    }
    
    println!("{:?}", out);
    "".to_string()
}

fn _accum2(s:&str)->String {
    s.chars().enumerate()
        .map(|(i,c)|c.to_string().to_uppercase() + c.to_string().to_lowercase().repeat(i).as_str())
        .collect::<Vec<String>>()
        .join("-")
}

fn main() {

    //println!("{}", _accum("ZpglnRxqenU"));
    
    println!("{}", accum("ZpglnRxqenU"));
    
    println!("Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
}

#[test]
fn basic_tests() {
  assert_eq!(accum("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
  assert_eq!(accum("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb");
  assert_eq!(accum("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu");
  assert_eq!(accum("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm");
  assert_eq!(accum("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc");
}
