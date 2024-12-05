use std::collections::HashMap;

fn main() {
	let (hash, input) = get_input();
	println!("{:?}", hash);
	println!("{:?}", input);
}

fn get_input() -> (HashMap<i32, i32>, Vec<Vec<i32>>) {
	let file = "data/example.txt";
	let contents = std::fs::read_to_string(file).unwrap();

	let mut parts = contents.split("\n\n");
	let mut hash: HashMap<i32, i32> = HashMap::new();
	let _ = parts.next().unwrap().split("\n").for_each(|x| {
		let mut parts = x.split("|");
		let key = parts.next().unwrap().parse::<i32>().unwrap();
		let value = parts.next().unwrap().parse::<i32>().unwrap();
		hash.insert(key, value);
	});

	let input = parts.next().unwrap().split("\n").map(|x| x.split(",").filter(|x| x.len() > 0).map(|x| x.parse::<i32>().unwrap()).collect()).collect();
	(hash, input)
}

