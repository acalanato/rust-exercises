
fn _rgb(r: i32, g: i32, b: i32) -> String {
    let mut out = String::new();
    for x in vec!(r,g,b) {
	match x {
	    1..=10 => out.push_str(&(format!("{:02?}", x))),
	    11 => out.push_str("a"),
	    12 => out.push_str("b"),
	    13 => out.push_str("c"),
	    14 => out.push_str("d"),
	    15 => out.push_str("e"),
	    16 => out.push_str("f"),
	    _ => out.push_str("00"),
	};
    }
    out
}

fn rgb(r: i32, g: i32, b: i32) -> String {

    format!("{:X?}{:X?}{:X?}", r % 16, g % 16, b % 16)

}

/*
 let s = format!("{:x}", n.round() as i32);
    if s.len() == 1 {
      "0".to_string() + &s
    } else {
      s
    }
  }
  let (r, g, b) = *rgb;
*/

fn main() {
    println!("{}", rgb(1, 2, 3));
}


macro_rules! compare {
  ( $got : expr, $expected : expr ) => {
    if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
  };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
