#[derive(Debug)]
struct Report {
	entries: Vec<i8>,
	is_safe: bool,
	is_increasing: bool,
	removed: i8,
}

impl Report {
	fn new(entrie: String) -> Report {
		// entrie is a string of numbers separated by a space
		let entries: Vec<i8> = entrie.split_whitespace().map(|n| n.parse().unwrap()).collect();

		let (is_increasing, is_safe, removed) = run_with_list_but_one(entries.clone(), |entries| {
			let zip: Vec<(&i8, &i8)> = entries.iter().zip(entries.iter().skip(1)).collect();

			// safe if all values are increasing or decreasing and the max difference between two
			// consecutive values is 3
			let is_increasing = zip.iter().all(|(&a, &b)| a < b);
			let is_safe = zip.iter().all(|(&a, &b)| is_increasing == (a < b) && (b - a).abs() <= 3 && b != a);

			(is_increasing, is_safe)
		});

		Report {
			entries: entries,
			is_safe: is_safe,
			is_increasing: is_increasing,
			removed: removed,
		}
	}
}

fn main() {
	let input = get_input();
	// how many reports are safe
	input.iter().for_each(|r| println!("{:?}", r));
	println!("{}", input.iter().filter(|r| r.is_safe).count());
}

fn get_input() -> Vec<Report> {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents.lines().map(|l| Report::new(l.to_string())).collect()
}

fn run_with_list_but_one<T>(v: Vec<T>, f: fn(Vec<T>) -> (bool, bool)) -> (bool, bool, i8) where T: Clone {
	let mut res;
	res = f(v.clone());
	if res.1 {
		return (res.0, res.1, -1);
	}
	for i in 0..v.len() {
		let mut v2 = v.clone();
		v2.remove(i);
		res = f(v2);
		if res.1 {
			return (res.0, res.1, i as i8);
		}
	}
	(false, false, -1)
}
