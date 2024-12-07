fn main() {
	let input = get_input();
	println!("{:?}", input.iter().filter(|c| can_be_calibrated(c.result, c.values.clone())).map(|c| c.result).sum::<i64>());
}

fn can_be_calibrated(result: i64, mut values: Vec<i64>) -> bool {
	println!("{:?} {}", values, result);
	if values.len() == 1 {
		return values[0] == result;
	}
	let interest = (get_value(&mut values, 0), get_value(&mut values, 0));
	values.insert(0, interest.0 + interest.1);
	print!("{} + {} = ", interest.0, interest.1);
	let mut is_calibrated = can_be_calibrated(result, values.clone());
	if !is_calibrated {
		print!("{} * {} = ", interest.0, interest.1);
		values[0] = interest.0 * interest.1;
		is_calibrated = can_be_calibrated(result, values.clone());
	}
	is_calibrated
}

fn get_value<T>(list: &mut Vec<T>, index: usize) -> T where T: Clone {
	let value = list[index].clone();
	list.remove(index);
	value
}

fn get_input() -> Vec<Callibration> {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	// format: result: value1 value2 value3 ...
	contents.lines().map(|s| {
		let mut parts = s.split_whitespace();
		let result = parts.next().unwrap().trim_end_matches(':').parse().unwrap();
		let values = parts.map(|s| s.parse().unwrap()).collect();
		Callibration { result, values }
	}).collect()
}

#[derive(Debug)]
struct Callibration {
	result: i64,
	values: Vec<i64>,
}
