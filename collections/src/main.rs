use core::fmt;


fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    let mut e = (6..=8).collect();

    v.append(&mut e);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); //get return None instead of panic
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i += 5
    }
    
    for i in v {
        println!("{}", i)
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum SpreadSheet {
        Int(i32),
        Float(f64),
        Text(String),
    }
/*
    impl fmt::Display for SpreadSheet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", &self)
        }
    }
*/    
    let row = vec![
        SpreadSheet::Int(3),
        SpreadSheet::Text(String::from("banana")),
        SpreadSheet::Float(10.34)
    ];

    println!("Row: {:?}, Colum: {:?}, Text: {:?}", row[0], row[1], row[2])
}
