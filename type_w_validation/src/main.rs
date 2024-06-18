pub struct TypeValidation {
    value: i32,
}

impl TypeValidation {
    pub fn new(value: i32) -> TypeValidation {
        if value < 1 || value > 100 {
            panic!("Not valid an stuff");
        }
        TypeValidation { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}


fn main() {
    let value  = TypeValidation::new(110).value;
    println!("{}", value);
}
