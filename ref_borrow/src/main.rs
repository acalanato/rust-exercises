fn main() {
    let s1 = String::from("hello");
    let len = calculate_len(&s1); // press & to reference
    
    println!("The lenght of '{}' is {}", s1, len);

    let mut s2 = String::from("Mutate");
    // only one &mut reference at time is allowed for each value
    change(&mut s2); //making it mutable also changes the type
    println!("{}", s2);
    mix_mut(&mut s2);

//    let reference_to_nothing = dangle();

}

fn calculate_len(s: &String) -> usize {
    s.len()
} // s goes out of scope but doesn't drop because it isn't owned

fn change(s: &mut String) { //&mut solve this
    s.push_str(" me");
    {
	s.push_str(" out")
    }// can't mix mutable and immutable references
}

fn mix_mut(s: &mut String){
    let _r1 = &s;
    let _r2 = &s;

    let r3 = &s;
    
    println!("{:?}", r3);
}

// Dangling is the error of using the wrong memory address with ptr

/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/

fn _no_dangle() -> String {
    let s = String::from("no dangle");
    s
}

/*

At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.

*/
