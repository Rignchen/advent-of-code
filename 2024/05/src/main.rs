use std::collections::HashMap;

fn main() {
	let (hash, input) = get_input();
	// for each input, ensure that the previous values are not in the vector of values for this key
	let filtered = input.iter().filter(|line| {
		if line.len() == 0 {
			return false;
		}
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
	}).collect::<Vec<&Vec<i32>>>();
	println!("{:?}", filtered);
	println!("{}", filtered.into_iter().map(
		//get the value in the middle of the vector
		|x| x[x.len() / 2]
	).sum::<i32>());
}

fn get_input() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
	let file = "data/input.txt";
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

