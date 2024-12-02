#[derive(Debug)]
struct Report {
	entries: Vec<i8>,
	is_safe: bool,
	is_increasing: bool,
}

impl Report {
	fn new(entrie: String) -> Report {
		// entrie is a string of numbers separated by a space
		let entries: Vec<i8> = entrie.split_whitespace().map(|n| n.parse().unwrap()).collect();

		// set the is_increasing field if all numbers are increasing/decresing
		// set the is_safe field to false if some number are increasing and some are decreasing
		// the difference between each number must be at least 1 and at most 2
		let mut is_increasing = entries.iter().zip(entries.iter().skip(1)).all(|(a, b)| a < b);
		let mut is_safe       = entries.iter().zip(entries.iter().skip(1)).all(|(a, b)| is_increasing == (a < b) && 1 <= (b - a).abs() && (b - a).abs() <= 3);

		Report {
			entries: entries,
			is_safe: is_safe,
			is_increasing: is_increasing,
		}
	}
}

fn main() {
	let input = get_input();
	// how many reports are safe
	println!("{}", input.iter().filter(|r| r.is_safe).count());
}

fn get_input() -> Vec<Report> {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents.lines().map(|l| Report::new(l.to_string())).collect()
}

