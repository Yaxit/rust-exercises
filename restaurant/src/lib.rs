mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,  // public
        seasonal_fruit: String, // private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                // fuit is not exposed, but can be used inside the mkodule
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod customer {
    // idiomatice use:
    // for functions, use the parent to clarify that the fn ios not locally defined eg: crate::front_of_house::hosting
    // for structs and enum specify the full path eg: std::collection::Hashmap
    use crate::front_of_house::hosting;
    // if pub use crate::front_of_house::hosting, people can use hosting directly from here

    pub fn eat_at_restaurant() {
        hosting::add_to_waitinglist();
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitinglist();
    
    // relative path
    front_of_house::hosting::add_to_waitinglist();

    let mut meal = back_of_house::Breakfast::summer("Rye");  // select a toast
    meal.toast = String::from("Wheat");    // can still; be chanbges since public
    // meal.seasonal_fruit = String::from("blueberries");  // would fail

    println!("Toast type is {}", meal.toast);

    // for a public enum, all options are public (of course)
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}