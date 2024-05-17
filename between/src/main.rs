fn main() {
    let x = between(1, 15);
    println!("{x:?}");
}

fn between(a: i16, b: i16) -> Vec<i16> {
    let mut i = a;
    let mut out: Vec<_> = Vec::new();
    loop {
        out.push(i);
        if i < b {i += 1;} else {break};
    }
    return out;
}

/*
fn between(a: i16, b: i16) -> Vec<i16> {
    (a..=b).collect()
}
*/
