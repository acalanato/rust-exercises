
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    };
    //short for the code above
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //test
    if let Some(max) = config_max {
        let min = |x| (x * max) as u8;
        println!("Minimum is configured to be {},which is higher than {}", min(max), max)
    }
    
}
