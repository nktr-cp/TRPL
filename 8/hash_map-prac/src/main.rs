use std::collections::HashMap;

fn main() {
	{
		let mut scores = std::collections::HashMap::new();

		scores.insert(String::from("Blue"), 10);
		scores.insert(String::from("Yellow"), 50);

		for (key, value) in &scores {
			println!("{} {}", key, value);
		}
	}

	{
		let teams = vec![String::from("Blue"), String::from("Yellow")];
		let initial_scores = vec![10, 50];

		let socres: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
	}

	{
		let field_name = String::from("Favorite color");
		let field_value = String::from("Blue");

		let mut map = HashMap::new();
		map.insert(field_name, field_value);

		// この時点で所有権はハッシュマップに移動
		// println!("{}", field_name);
	}

	{
		let mut scores = HashMap::new();

		scores.insert(String::from("Blue"), 10);

		scores.entry(String::from("Yellow")).or_insert(50);
		scores.entry(String::from("Blue")).or_insert(25);

		println!("{:?}", scores);
	}

	{
		let text = "hello world wonderful world";

		let mut map = HashMap::new();

		for word in text.split_whitespace() {
			let count = map.entry(word).or_insert(0);
			*count += 1;
		}

		println!("{:?}", map);
	}
}
