
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
	x
    } else {
	y
    }
}

fn return_string() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result =  longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn return_another() {
    let string1 = String::from("as long as the string is long");
    {
	let string2 = String::from("xyz");
	let result = longest(&string1, &string2);
    	println!("The longest string is {result}");
    }
}

struct Excerpt<'a> {
    part: &'a str,
}

fn capivaras() {
    let capi = String::from("As capivaras voadoras não sabiam nadar. Portanto...");
    let first_capi = capi.split(".").next().expect("Não há pontos");
    let i = Excerpt {
	part: first_capi,
    };
    println!("{}", i.part)
}

fn main() {
//    let outer;
    
    let inner = 5;
    let outer = &inner;
    println!("outer: {outer}");
    return_string();
    return_another();
    capivaras();
}

//resume from listing 10-25
