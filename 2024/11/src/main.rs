use std::collections::HashMap;

fn get_input() -> Vec<u64> {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
	let input = get_input();
	let mut recursive_cache = HashMap::<(u64, u64), u64>::new();
	let mut value_cache = HashMap::<u64, Vec<u64>>::new();
	let mut result = 0;
	println!("{:?}", input);
	for stone in input {
		result += calcul_value(stone, 75, &mut recursive_cache, &mut value_cache);
	}
	println!("{:?}", result);
}

fn calcul_value(stone: u64, left: u64, recursive_cache: &mut HashMap<(u64, u64), u64>, value_cache: &mut HashMap<u64, Vec<u64>>) -> u64 {
	println!("{:?} {:?}", stone, left);
	// look in cache if we already calculated the value
	if recursive_cache.contains_key(&(stone, left)) {
		return *recursive_cache.get(&(stone, left)).unwrap();
	}

	let values = if let Some(values) = value_cache.get(&stone) {
		values.clone()
	} else {
		let values = get_values(stone);
		value_cache.insert(stone, values.clone());
		values
	};
	if left == 1 {
		return values.len() as u64;
	}
	let mut result = 0;
	for value in values {
		result += calcul_value(value, left - 1, recursive_cache, value_cache);
	}

	// insert the result in the cache for future use
	recursive_cache.insert((stone, left), result);
	result
}

fn get_values(stone: u64) -> Vec<u64> {
	if stone == 0 {
		return vec![1];
	} else if has_even_length(&stone.to_string()) {
		let stone_to_string = stone.to_string();
		let (left, right) = stone_to_string.split_at(stone_to_string.len() / 2);
		return vec![left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()];
	} else {
		return vec![stone * 2024];
	}
}

fn has_even_length(text: &str) -> bool {
	text.len() % 2 == 0
}
