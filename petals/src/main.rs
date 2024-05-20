fn main() {
    let a = how_much_i_love_you(8);
    let b = 8 % 6;
    println!("{a} \t input:{b}");
}

fn how_much_i_love_you(nb_petals: u16) -> &'static str {
    let nb = nb_petals % 6;
    if nb == 1 {
        "I love you"
    } else if nb == 2 {
        "a little"
    } else if nb == 3 {
        "a lot"
    } else if nb == 4 {
        "passionately"
    } else if nb == 5 {
        "madly"
    } else {
        "not at all"
    }
    
    /*
    match nb_petals % 6 {
        1 => "I love you",
        2 => "a little",
        3 => "a lot",
        4 => "passionately",
        5 => "madly",
        6 => "not at all",
        _=> panic!("Gratz, you broke it!")
    }
    */
}

