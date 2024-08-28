
fn update_light(current: &str) -> String {
    match current {
	"red" => "green".to_string(),
	"green" => "yellow".to_string(),
	"yellow" => "red".to_string(),
	_ => "".to_string(),
    }
}

fn main() {
    update_light("red");
    update_light("green");
    update_light("blue");
    println!("Sucess!");
}

#[test]
fn basic_test() {
  assert_eq!(update_light("green"), "yellow");
  assert_eq!(update_light("yellow"), "red");
  assert_eq!(update_light("red"), "green");
}
