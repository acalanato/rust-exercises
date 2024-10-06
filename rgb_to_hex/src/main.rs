
fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{:02x?}{:02x?}{:02x?}", r.clamp(0, 255), g.clamp(0, 255), b.clamp(0, 255) ).to_ascii_uppercase()
}

fn _rgb(r: i32, g: i32, b: i32) -> String {

    let to_hex = |c: i32| if c.to_string().len() == 1 {
	format!("{:02x?}", c % 16).to_ascii_uppercase()

    } else { format!("{:x?}", c % 16).to_ascii_uppercase() };

    format!("{}{}{}", to_hex(r), to_hex(g), to_hex(b))

}

 /*
    format!("{:02x?}{:02x?}{:02x?}", r.clamp(0, 255), g.clamp(0, 255), b.clamp(0, 255) ).to_ascii_uppercase()
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
