use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};

	let mut s = String::new();

	// ファイルの中身をsに読み出す
	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

fn read_username_from_file2() -> Result<String, io::Error> {
	// let mut f = File::open("hello.txt")?;
	// let mut s = String::new();
	// f.read_to_string(&mut s)?;
	// Ok(s)

	let mut s = String::new();

	// much simpler
	File::open("hello.txt")?.read_to_string(&mut s)?;

	Ok(s);
}

fn main() {
	// panic!("crash and burn");
	// let v = vec![1, 2, 3];

	// panic by library
	// v[99];

	let f = File::open("hello.txt");

	let f = match f {
		Ok(file) => file,
		Err (ref error) if error.kind() == ErrorKind::NotFound => {
			match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => {
					panic!(
						"Tried to create file but there was a problem: {:?}",
						e
					)
				},
			}
		},
		Err(error) => {
			panic!(
				"There was a problem opening the file: {:?}",
				error
			)
		},
	};

	let g = File::open("42.txt").unwrap();
}