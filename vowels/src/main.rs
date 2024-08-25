fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    //let (vowel, letter) = (['a','e','i','o','u'], string.as_bytes().iter());
    let (vowel, letter) = (['a','e','i','o','u'], string.chars());

    //let b = ['a','e','i','o','u'];
    for x in letter {
	for y in vowel {
	    if letter[x] == vowel[y] {
		vowels_count += 1;
	    }
	}
    }
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
    println!("Hello, world!");
}

#[test]
fn my_tests() {
  assert_eq!(get_count("abracadabra"), 5);
}
