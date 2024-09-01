enum SpreadSheetCell {
	Int(i32),
	Float(f64),
	Text(String),
}

fn main() {
	let v1: Vec<i32> = Vec::new();
	let v2 = vec![1, 2, 3];

	let mut v3 = Vec::new();
	v3.push(5);
	v3.push(6);

	{
		let v = vec![1, 2, 3, 4];
		let third: &i32 = &v[2];
		println!("The third element is {}", third);

		match v.get(2) {
			Some(third) => println!("The third element is {}", third),
			None => println!("There is no third element."),
		}
	}

	for i in &mut v3 {
		*i += 50;
		println!("{}", i);
	}

	let row = vec![
		SpreadSheetCell::Int(3),
		SpreadSheetCell::Float(4.2),
		SpreadSheetCell::Text(String::from("blue")),
	];
}