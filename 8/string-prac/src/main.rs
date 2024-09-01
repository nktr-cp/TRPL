fn main() {
	{
		let mut s = String::new();
	}

	{
		// &str
		let data = "initial contents";

		// String
		let s = data.to_string();

		// the method also works on a literal directly
		let s = "initial contents".to_string();

		// above is equivalent to the following
		let s = String::from("initial contents");

		// String is encoded in UTF-8
		let hello = String::from("السلام عليكم");
	}

	{
		let mut s = String::from("foo");
		s.push_str("bar");

		println!("{}", s);
	}

	{
		let mut s1 = String::from("tic");
		let s2 = String::from("tac");

		s1.push_str(&s2);

		println!("s2 is {}", s2);

		// ここではs1の所有権はsに移動している
		let s = s1 + &s2;

		println!("s is {}", s);
	}

	{
		let a1 = String::from("tic");
		let a2 = String::from("tac");
		let a3 = String::from("toe");

		let s = format!("{}-{}-{}", a1, a2, a3);

		println!("{}", s);
	}

	{
		for c in "नमस्ते".chars() {
			println!("{}", c);
		}

		for c in "नमस्ते".bytes() {
			println!("{}", c);
		}
	}
}