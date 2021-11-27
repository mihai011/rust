mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() -> u8 {
            2 + 2
        }
        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {
            println!("Serving order")
        }

        fn take_payment() {}
    }
}

mod back_of_the_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer_breakfast(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn cook_order() {
        println!("'Cooking order!");
    }

    pub fn fix_incorect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //absolute path
    hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_the_house::Breakfast::summer_breakfast("Rye");
    // Change our mind about what bread we;'d like
    println!("I'd like {} toast please", meal.toast);

    let test: String;

    // The next line won't compile if we uncomment it: we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    //meal.seasonal_fruit =  String::from("Blueberries");
}

mod tests {
    use crate::{
        back_of_the_house::{self, Breakfast},
        front_of_house,
    };

    #[test]
    fn it_works() {
        assert!(front_of_house::hosting::add_to_waitlist() == 4);
    }

    #[test]
    fn test_incorrect_order() {
        back_of_the_house::fix_incorect_order();
    }

    #[test]
    fn test_breakfast() {
        let breakfast: Breakfast = Breakfast::summer_breakfast("toast_with_bacon");

        assert!(breakfast.toast == "toast_with_bacon")
    }
}
