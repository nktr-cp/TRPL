fn main() {
	let x = 42;
	let y = x;

	println!("x = {}, y = {}", x, y); // これはスタック上でのコピーが行われているので、問題ない

	let s1 = String::from("hello");
	let s2 = s1; // ここでs1の所有権はs2に移動, ムーブされている

	// println!("{}, world!", s1); // この時点で無効化されているのでエラー
	println!("{}, world!", s2); // これは問題ない

	let s1 = String::from("hello");
	let s2 = s1.clone();
	println!("s1 = {}, s2 = {}", s1, s2);

	takes_ownership(s2);
	makes_copy(y);

	// println!("s2 = {}", s2); // ここでs2はすでに所有権を失っていて、エラーになる
	println!("y = {}", y); // i32はCopyトレイトが配置されているので、所有権を失わない

	{
		let s1 = give_ownership();

		let s2 = String::from("hello");

		let s3 = takes_and_gives_back(s2); // s2から所有権を奪い、s3に渡す
	}

	{
		let s1 = String::from("hello");

		let (s2, len) = calculate_length1(s1);

		println!("The length of '{}' is {}.", s2, len);
	}

	{
		let s1 = String::from("hello");

		// 参照をとれば、所有権を失わない
		let len = calculate_length2(&s1);

		println!("The length of '{}' is {}.", s1, len);
	}

	{
		let mut s1 = String::from("hello");

		change(&mut s1);

		println!("s1 = {}", s1);
	}

	let mut s = String::from("hello world");

	let word = first_word(&s[..]);

	// スライスを使用したfirst_wordを使うと、これがコンパイルエラーになる
	// s.clear();

	println!("The first word is: {}", word);
}

// 関数シグニチャに&strを使うことで、Stringと&strの両方を受け取れて、文字列リテラルなども受け取れる
fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}

	&s[..]
}

// sの状態に大きく依存してしまい、好ましくない
// fn first_word(s: &String) -> usize {
// 	let bytes = s.as_bytes();

// 	for (i, &item) in bytes.iter().enumerate() {
// 		if item == b' ' {
// 			return i
// 		}
// 	}

// 	s.len()
// }

fn change(s: &mut String) {
	s.push_str(", world");
}

fn calculate_length2(s: &String) -> usize {
	s.len()
}

fn calculate_length1(s: String) -> (String, usize) {
	let length = s.len();
	(s, length)
}

fn give_ownership() -> String {
	let some_string = String::from("hello");
	some_string
}

fn takes_and_gives_back(a: String) -> String {
	a
}

fn takes_ownership(s: String) {
	println!("{}", s);
}

fn makes_copy(x: i32) {
	println!("{}", x);
}

