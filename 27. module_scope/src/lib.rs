use back_of_house::Breakfast;

mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }
    mod serving {
        use super::hosting;

        // crate::front_of_house::hosting::
        fn take_order() {}

        fn serve_order() {
            // Absolute path
            crate::front_of_house::hosting::seat_at_table();

            // Relative path
            super::hosting::seat_at_table();
            hosting::seat_at_table();
        }

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
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Always controlled"),
            }
        }
    }
}

// absolute path vs relative path

fn eat_at_restaurent() {
    // crate::front_of_house:: from crate every thing is private
    front_of_house::hosting::seat_at_table();
    crate::front_of_house::hosting::seat_at_table();
}

fn my_fun() {
    // let breakfast_1: Breakfast = Breakfast {
    //     toast: "Shiv",
    //     seasonal_fruit: "123",
    // }; // we can not access the private variable
    let breakfast_1: Breakfast = back_of_house::Breakfast::summer("Shiv");
}

mod sum1 {
    pub fn sum(a: u8, b: u8) -> u8 {
        return a + b;
    }
}

mod sum2 {
    pub fn sum(a: u8, b: u8) -> bool {
        return a > b;
    }
}

use crate::sum1::sum;
use crate::sum2::sum as sum3;

fn solve() {
    let c = sum(10, 20);
    let d = sum3(20, 30);
}

mod sum_of_two {
    pub fn sum(a: u8, b: u8) -> u8 {
        return a + b;
    }
}

mod sum_of_three {
    pub fn sum(a: u8, b: u8) -> bool {
        return a > b;
    }
}

use crate::sum_of_three as b;
use crate::sum_of_two as a;

fn solve1() {
    let c = a::sum(10, 20);
    let d = b::sum(20, 30);
}
