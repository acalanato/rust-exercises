mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        //each field of a struct must have pub in order to be accessible
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        //each enum variant will be pub
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    //abs path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please?", meal.toast);
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
