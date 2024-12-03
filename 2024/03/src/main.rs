fn main() {
	let input = get_input();
	// match a regex to get the tokens
	let re = regex::Regex::new(r"mul\(\d+,\d+\)").unwrap();
	let mut sum = 0;
	for cap in re.captures_iter(&input) {
		let mut nums = cap.get(0).unwrap().as_str();
		// remove "mul(" and ")"
		println!("{}", nums);
		nums = &nums[4..nums.len()-1];
		let mut iter = nums.split(",");
		let a: i32 = iter.next().unwrap().parse().unwrap();
		let b: i32 = iter.next().unwrap().parse().unwrap();
		sum += a * b;
	}
	println!("Sum: {}", sum);
}

fn get_input() -> String {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents
}
