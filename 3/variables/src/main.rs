fn main() {
	let mut x = 5;

	println!("The value of x is: {}", x);

	x = 6;

	println!("The value of x is: {}", x);

	const MAX_POINTS: u32 = 100_000;

	let y = 5;

	let y = y + 1;

	{
		let y = y * 2;
		println!("The value of y in the inner scope is: {}", y);
	}

	println!("The value of y is: {}", y);

	// シャドーイング、これはok
	let spaces = "    ";
	let spaces = spaces.len();

	// let mut spaces = "    ";
	// spaces = spaces.len(); // これはエラー

	let c = 'z';
	let z = 'ℤ';
	let heart_eyed_cat = '😻';

	let tup: (i32, f64, u8) = (500, 6.4, 1);

	let (x, y, z) = tup;

	println!("The value of y is {}", y);
	println!("The value of z is {}", tup.2);

	another_function();

	println!("{}", echo_function(42));
}

fn another_function() {
	println!("This is another function.");
}

fn echo_function(x: i32) -> i32 {
	x
}
