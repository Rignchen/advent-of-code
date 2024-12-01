fn main() {
	let input = get_input();
	let (mut left_numbers, mut right_numbers) = split_input_in_lists(&input);

	// sort both lists
	left_numbers.sort();
	right_numbers.sort();

	let mut total_diff = 0;
	for i in 0..left_numbers.len() {
		total_diff += (left_numbers[i] - right_numbers[i]).abs();
	}

	println!("The total difference is: {}", total_diff);
}

/// Get the input from the input.txt file
fn get_input() -> String {
	std::fs::read_to_string("input.txt").unwrap()
}

/// Each line of the input is formatted as `<left number>   <right number>`
/// I wat to get all left numbers in a vector and all right numbers in another vector
fn split_input_in_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
	let mut left_numbers = Vec::new();
	let mut right_numbers = Vec::new();

	for line in input.lines() {
		let mut parts = line.split_whitespace();
		left_numbers.push(parts.next().unwrap().parse::<i32>().unwrap());
		right_numbers.push(parts.next().unwrap().parse::<i32>().unwrap());
	}

	(left_numbers, right_numbers)
}
