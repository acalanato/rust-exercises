fn main(){
    let mut s = String::from("funciona?");
    let o = "sim";
    println!("s = {}, o = {}", s, o);
    s.push_str(o);
    println!("{}", s);
}

/*fn test(word: str){
    printl!(word);
}
*/
