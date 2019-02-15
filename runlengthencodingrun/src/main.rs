use std::io;

fn encode(input : &str) -> String {
	let mut prev = 'a';
	let mut counter = 0;
	let mut result = String::new();
	for c in input.chars() {
		if counter == 0 {
			counter += 1;
			prev = c;
		} else if c == prev {
			counter += 1;
		} else {
			result.push(prev);
			result.push_str(&counter.to_string());
			prev = c;
			counter = 1;
		}
	}

	result.push(prev);
	result.push_str(&counter.to_string());
	result
}

fn decode(input : &str) -> String {
	let mut result = String::new();
	let mut char = 'a';
	let mut counter = String::from("0");
	for c in input.chars() {
		if c.is_numeric() {
			counter.push(c);
		} else {
			result.push_str(&char.to_string().repeat(counter.parse().unwrap()));
			counter = String::new();
			char = c;
		}
	}
	result.push_str(&char.to_string().repeat(counter.parse().unwrap()));
	result
}

fn main() {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let mut iter = input.trim().split_whitespace();
	let should_encode = match iter.next().unwrap() {
		"E" => true,
		_ => false
	};
	let str = iter.next().unwrap();

	let result;
	if should_encode {
		result = encode(&str);
	} else {
		result = decode(&str);
	}

	println!("{}", &result);
}
