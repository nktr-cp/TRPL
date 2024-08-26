enum IpAddrKind {
	V4,
	V6,
}

enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
		println!("call!!");
	}
}

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
	// --snip--
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn main() {
	let four = IpAddrKind::V4;

	let m = Message::Write(String::from("hello"));
	m.call();

	let coin = Coin::Quarter(UsState::Alaska);
	value_in_cents(&coin);

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	let some_u8_value = Some(0u8);
	
	// match some_u8_value {
	// 	Some(3) => println!("three"),
	// 	_ => (),
	// }

	// 以下は上のコードと等価
	if let Some(3) = some_u8_value {
		println!("three");
	}

	let mut count = 0;
	// match coin {
	// 	Coin::Quarter(state) => println!("State quater from {:?}", state),
	// 	_ => count += 1,
	// }

	// 以下は上のコードと等価
	if let Coin::Quarter(state) = coin {
		println!("State quater from {:?}", state);
	} else {
		count += 1;
	}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		// iに束縛する
		Some(i) => Some(i+1),
	}
}

fn value_in_cents(coin: &Coin) -> u32 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		// state変数はCoin::QuarterのUsStateの値に束縛される
		Coin::Quarter(state) => {
			println!("State quater from {:?}", state);
			25
		}
	}
}
