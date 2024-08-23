fn find_short(s: &str) -> u32 {
    let mut o = 0_u32;
    let list = s.split(" ").collect::<Vec<_>>();
    //try sorting algorithm first
    //for x in list {
	//todo!("sortin")
    //}
}

fn main() {
    let words = find_short("várias palavras aqui para encontrar a menor de todas");
    println!("Shortest: {}", words);
}
