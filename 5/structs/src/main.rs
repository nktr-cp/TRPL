struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

// タプル構造体
// 以下の2つに互換性はない
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
	let mut user1 = build_user(
		String::from("someone@example.com"),
		String::from("someusername123"),
	);

	let user2 = User {
		email: String::from("another@example.com"),
		username: String::from("anotherusername567"),
		// active: user1.active,
		// sign_in_count: user1.sign_in_count,
		..user1
	};
}

fn build_user(email: String, username: String) -> User {
	User {
		// フィールド初期化省略記法
		// email: email,
		email,
		// username: username,
		username,
		active: true,
		sign_in_count: 1,
	}
}
