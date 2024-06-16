
fn main() {
    let mut _a: [char;10] = ['0','1','2','3','4','5','6','7','8','9'];
    let _b: &str = "\u{0460}";
    let _c: char = '\u{41}';
    let _d: String = "Test to string".to_string();
    let mut _e = "Infer this".to_string();
    _e.push_str(", add this");
    _e.push('.');
    let _f = String::from("Plus");
    let _ff = String::from("Sign");
    let _fff = _f + &_ff; //deref is a bitch
    let g = format!("{_b} {_c} {_d} {_e}");
    //println!("Sucess!");
    for c in g.chars() {
        print!("{}", c)
    };
    println!();
        for c in g.bytes() {
        print!("{}", c)
    };
    println!();
}
