use std::io;

fn main() {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let x : i32 = input.trim().parse().unwrap();
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let y : i32 = input.trim().parse().unwrap();
	match (x < 0, y < 0) {
		(true, true) => println!("3"),
		(true, false) => println!("2"),
		(false, false) => println!("1"),
		(false, true) => println!("4")
	}
}
