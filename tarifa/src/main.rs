use std::io;

fn main() {
	let mut input_X = String::new();
	io::stdin().read_line(&mut input_X);
	let X : i32 = input_X.trim().parse().unwrap();

	let mut input_N = String::new();
	io::stdin().read_line(&mut input_N);
	let N : i32 = input_N.trim().parse().unwrap();

	let mut counter : i32 = X;
	for _ in 0..N {
		let mut input = String::new();
		io::stdin().read_line(&mut input);
		counter +=  X - input.trim().parse::<i32>().unwrap();
	}

	println!("{}", counter);
}
