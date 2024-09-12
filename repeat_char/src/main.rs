
fn accum(s:&str)->String {
    //let words = s.chars().map(|x| x as u8).collect::<Vec<u8>>();


    let mut out: Vec<&u8> = Vec::new();
    for (i, val) in s.as_bytes().into_iter().enumerate() {
	out = std::iter::repeat(val).take(i).collect::<Vec<_>>();

	//for _x in 0..=i {out.push(*val);
	//println!();
	//};
	//out.push(b'-');
    }
    String::from_utf8(out).unwrap()
}
    


fn _accum(s:&str)->String {
    let mut out: Vec<String> = Vec::new();
    let list = s.chars().map(|x| x as u8).collect::<Vec<u8>>();
    let repeat = |x: char, y: usize| std::iter::repeat(x).take(y);
    
    //String::from_utf8(out).unwrap()
    
    println!("{:?}", out);
    "".to_string()
}


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
