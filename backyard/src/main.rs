use crate::garden::vegetables::Asparagus;


pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
    print_structure();
    
}


fn print_structure(){

    print!("
Project structure so far:
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
")
}

