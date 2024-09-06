
fn accum(s:&str)->String {
    let words = s.chars().map(|x| x as u8).collect::<Vec<u8>>();
    let mut out: Vec<u8> = Vec::new();
    for (i, val) in s.as_bytes().into_iter().enumerate() {
	
	for _x in 0..=i {out.push(*val);
			 out.push(b'-');
	};
	
	//if val != words.last().unwrap() {words.push(b'-')}; this is dumb

    }
    String::from_utf8(words).unwrap()
}
    
/*

}

fn accum(s:&str)->String {
    let mut out = Vec::new();
    s.chars().map(|c| out.extend(c) ).collect()
}

 */

fn main() {

    println!("{}", accum("ZpglnRxqenU"));
    
    println!("Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuu");
}



#[test]
fn basic_tests() {
  assert_eq!(accum("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
  assert_eq!(accum("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb");
  assert_eq!(accum("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu");
  assert_eq!(accum("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm");
  assert_eq!(accum("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc");
}
