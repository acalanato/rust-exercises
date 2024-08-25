
fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    let list = string.chars().map(|x| x as u8).collect::<Vec<u8>>();
    let letter = "aeiou".chars().map(|x| x as u8).collect::<Vec<u8>>();

    for x in list {
	if letter.contains(&list[x]) {
	    vowels_count += 1;
	}
    }
    //let b = ['a','e','i','o','u'];
    vowels_count
}
 /*
  for x in row {
    for y in col{
      row[x][y] = word[y]
    }
  }
        break 'out
 */




fn main() {
    println!("{}", get_count("abracadabra"));
}

#[test]
fn my_tests() {
  assert_eq!(get_count("abracadabra"), 5);
}
