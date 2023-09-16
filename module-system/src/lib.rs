// Create a library that works like a resturant
// This is a library crate since the file name is lib.rs
// We can have 0 or 1 library crate in the package
// We can have multiple binary crates (main.rs is a binary crate) in a a package by creating a bin folder

mod front_of_house;

pub fn eat_at_restaurant() {
  // Absolute path to module
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path to module. starts with current module
  front_of_house::hosting::add_to_waitlist();

  // We utilize the use keyword to bring a path into scope
  // Convention is that you bring the parent module into scope and call its children function.
  use crate::front_of_house::hosting; // absolute path
  use self::front_of_house::serving; // relative path
  // other items like structs and enums can be brought in full
  use self::back_of_house::Breakfast as Lunch; // We can rename the item if names are conflicting.

  let mut meal = back_of_house::Breakfast::summer("ope");
  meal.toast = String::from("Bread");

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house {
  pub struct Breakfast {
    pub toast: String, // Attributes of a struct must be decalred public to be used
    seasonal_fruit: String,
  }

  pub enum Appetizer { // Attributes of enums are made public automatically when the enum itself is public
    Soup,
    Salad,
  }

  impl Breakfast {
      pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
          toast: String::from(toast),
          seasonal_fruit: String::from("peaches")
        }
      }
  }

  fn fix_incorrect_order() {
    cook_order();
    super::serve_order(); // A child can access its parent functions.
  }

  fn cook_order() {}
}