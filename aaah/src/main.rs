use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	io::stdin().read_line(&mut input1).unwrap();
	io::stdin().read_line(&mut input2).unwrap();
	if input1.trim().len() >= input2.trim().len() {
		println!("go");
	} else {
		println!("no");
	}
}
