mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}

		fn seat_at_table() {}
	}

	mod serving {
		fn take_order() {}

		fn serve_order() {}

		fn take_payment() {}
	}
}

fn serve_order() {}

mod back_of_house {
	pub enum Appetizer {
		Soup,
		Salad,
	}

	pub struct BreakFast {
		pub toast: String,
		seasonal_fruit: String,
	}

	impl BreakFast {
		pub fn summer(toast: &str) -> BreakFast {
			BreakFast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}

	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}

	fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restauran() {
	// absolute path
	// crate::front_of_house::hosting::add_to_waitlist();

	// relative path
	// front_of_house::hosting::add_to_waitlist();

	let mut meal = back_of_house::BreakFast::summer("Rye");

	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please", meal.toast);

	// この変数は公開されていないので、コンパイルエラーになる
	// meal.seasonal_fruit = String::from("blueberries");

	// enumの場合には、全ての要素が公開されている
	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;

	// useを使ったので、パスが省略できる
	hosting::add_to_waitlist();
}
