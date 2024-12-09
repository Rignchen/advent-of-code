fn get_input() -> Vec<i32> {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	let values: Vec<u32> = contents.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap()).collect();
	let mut output = Vec::new();
	let mut is_full = false;
	let mut n = 0;
	for (_,j) in values.iter().enumerate() {
		is_full = !is_full;
		for _ in 0..*j {
			if is_full {
				output.push(n);
			} else {
				output.push(-1);
			}
		}
		if is_full {
			n += 1;
		}
	}
	return output;
}

fn main() {
	let mut input = get_input();
	let mut emptys = Vec::new();
	for i in 0..input.len() {
		let j = input[i];
		if j == -1 {
			emptys.push(i);
		}
	}
	for i in (0..input.len()).rev() {
		let j = input[i];
		if j != -1 && emptys.len() > 0 && i > emptys[0] {
			let mut empty = emptys.get(0).unwrap();
			input[*empty] = j;
			input[i] = -1;
			emptys.remove(0);
			emptys.push(i);
			emptys.sort();
		}
	}
	let mut total: i64 = 0;
	println!("{:?}", input);
	for i in 0..input.len() {
		let j = input[i];
		if j != -1 {
			total += (j*i as i32) as i64;
		}
	}
	println!("{:?}", total);
}

