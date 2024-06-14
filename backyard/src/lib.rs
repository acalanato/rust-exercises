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

/*

    Alternate File Paths

So far we’ve covered the most idiomatic file paths the Rust compiler uses,
but Rust also supports an older style of file path. For a module named
front_of_house declared in the crate root, the compiler will look for the module’s code in:

        src/front_of_house.rs (what we covered)
        src/front_of_house/mod.rs (older style, still supported path)

    For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:

        src/front_of_house/hosting.rs (what we covered)
        src/front_of_house/hosting/mod.rs (older style, still supported path)

If you use both styles for the same module, you’ll get a compiler error.
Using a mix of both styles for different modules in the same project is allowed,
but might be confusing for people navigating your project.

The main downside to the style that uses files named mod.rs is that your
project can end up with many files named mod.rs, which can get confusing
when you have them open in your editor at the same time.

*/
