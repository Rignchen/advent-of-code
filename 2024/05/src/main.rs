use std::collections::HashMap;

fn main() {
	let (hash, input) = get_input();
	let filtered = input.iter().filter(|line| {line.len() > 0}).filter(|line| !is_valid(&hash, line)).collect::<Vec<_>>();
	let mut ordered: Vec<Vec<i32>> = Vec::new();
	filtered.into_iter().for_each(|line| {
		ordered.push(order_list(&hash, line));
	});
	println!("{:?}", ordered);
}

fn is_valid(hash: &HashMap<i32, Vec<i32>>, line: &Vec<i32>) -> bool {
	let mut valid = true;
	let empty = Vec::new();
	for i in 0..line.len() {
		if i > 0 {
			let prev = line[0..i].to_vec();
			let vector = hash.get(&line[i]).unwrap_or(&empty);
			if prev.iter().any(|x| vector.contains(x)) {
				valid = false;
				break;
			}
		}
	}
	valid
}

fn order_list(order: &HashMap<i32, Vec<i32>>, line: &Vec<i32>) -> Vec<i32> {
	let empty = Vec::new();
	let mut ordered: Vec<i32> = Vec::new();
	for i in 0..line.len() {
		let mut inserted = false;
		let value = line[i];
		let after = order.get(&value).unwrap_or(&empty);
		for j in 0..ordered.len() {
			if after.contains(&ordered[j]) {
				ordered.insert(j, value);
				println!("{:?}, j: {}, v: {}", ordered, j, value);
				inserted = true;
				break;
			}
		}
		if !inserted {
			ordered.push(value);
		}
	}
	ordered
}

fn get_input() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
	let file = "data/example.txt";
	let contents = std::fs::read_to_string(file).unwrap();

	let mut parts = contents.split("\n\n");
	let mut hash: HashMap<i32, Vec<i32>> = HashMap::new();
	let _ = parts.next().unwrap().split("\n").for_each(|x| {
		let mut parts = x.split("|");
		let key = parts.next().unwrap().parse::<i32>().unwrap();
		let value = parts.next().unwrap().parse::<i32>().unwrap();
		let _ = hash.entry(key).or_insert(vec![]).push(value);
	});

	let input = parts.next().unwrap().split("\n").map(|x| x.split(",").filter(|x| x.len() > 0).map(|x| x.parse::<i32>().unwrap()).collect()).collect();
	(hash, input)
}

