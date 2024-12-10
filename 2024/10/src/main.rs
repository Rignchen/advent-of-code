fn get_input() -> String {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents
}

fn main() {
	let input = get_input();
	let mut trails = Vec::new();
	for x in 0..input.lines().nth(0).unwrap().len() {
		for y in 0..input.lines().count() {
			if input.lines().nth(y).unwrap().chars().nth(x).unwrap().to_digit(10).unwrap() == 0 {
				let trail = find_trail_top(&input, x, y);
				if trail.len() > 0 {
					println!("Trail found: {:?}", trail);
					trail.into_iter().for_each(|t| trails.push(t));
				}
			}
		}
	}
	println!("Trails: {:?}", trails);
	println!("Sum: {}", trails.iter().map(|t| t.0).count());
}

fn find_trail_top(input: &str, x: usize, y: usize) -> Vec<(usize, usize)> {
	let current = input.lines().nth(y).unwrap().chars().nth(x).unwrap().to_digit(10).unwrap();
	println!("Checking {} ({}, {})", current, x, y);
	if current == 9 {
		return vec![(x, y)];
	}
	let mut trail: Vec<(usize, usize)> = Vec::new();

	if x > 0 &&
		input.lines().nth(y).unwrap().chars().nth(x - 1).unwrap().to_digit(10).unwrap() == current + 1 {
		let result = find_trail_top(input, x - 1, y);
		for r in result {
			trail.push(r);
		}
	}
	if x + 1 < input.lines().nth(y).unwrap().len() &&
		input.lines().nth(y).unwrap().chars().nth(x + 1).unwrap().to_digit(10).unwrap() == current + 1 {
		let result = find_trail_top(input, x + 1, y);
		for r in result {
			trail.push(r);
		}
	}
	if y > 0 &&
		input.lines().nth(y - 1).unwrap().chars().nth(x).unwrap().to_digit(10).unwrap() == current + 1 {
		let result = find_trail_top(input, x, y - 1);
		for r in result {
			trail.push(r);
		}
	}
	if y + 1 < input.lines().count() &&
		input.lines().nth(y + 1).unwrap().chars().nth(x).unwrap().to_digit(10).unwrap() == current + 1 {
		let result = find_trail_top(input, x, y + 1);
		for r in result {
			trail.push(r);
		}
	}

	trail
}
