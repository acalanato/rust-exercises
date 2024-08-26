
fn _get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    let list = string.chars().map(|x| x as u8).collect::<Vec<u8>>();
    let letter = "aeiou".chars().map(|x| x as u8).collect::<Vec<u8>>();
    let mut i = 0;
    for x in list.iter() {
	if x == &letter[i] {
	    vowels_count += 1;
	}
	i += 1;
    }
//    list.iter().filter_map(|s| Some(list.contains(letter[s])));

    vowels_count
}

// // // // ** \\ \\ \\ \\

fn get_count(string: &str) -> usize {
    let list = string.chars().map(|x| x as u8).collect::<Vec<u8>>();
    let mut vowels_count: usize = 0;
    for x in list.iter() {
	if "aeiou".as_bytes().contains(x) {vowels_count += 1;}
    }
    vowels_count
}

/* Top answer
fn get_count(string: &str) -> usize {
    string.matches(|x| match x {'a'|'e'|'i'|'o'|'u' => true, _ => false}).count()
}
*/

fn main() {
    println!("{}", get_count("abracadabra"));
}

#[test]
fn my_tests() {
  assert_eq!(get_count("abracadabra"), 5);
}
