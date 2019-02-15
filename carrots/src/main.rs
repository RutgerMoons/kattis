use std::io;

fn main() {
	let mut input = String::new();
	io::stdin().read_line(&mut input);
	let mut chunks = input.split_whitespace();
	let N = chunks.next().unwrap().parse().unwrap();
	let carrots : u32 = chunks.next().unwrap().parse().unwrap();
	for _ in 0..N {
		let mut input = String::new();
		io::stdin().read_line(&mut input);
	}

	println!("{}", carrots);
}
