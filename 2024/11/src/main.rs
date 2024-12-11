fn get_input() -> Vec<i64> {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
	let mut input = get_input();
	for _ in 0..25 {
		input = blink(input.clone());
		//println!("{:?}", input);
	}
	println!("{}", input.len());
}

fn blink(stones: Vec<i64>) -> Vec<i64> {
	let mut result = vec![];
	for (i, stone) in stones.iter().enumerate() {
		if stone == &0 {
			result.push(1);
		} else if has_even_length(&stone.to_string()) {
			let stone_to_string = stone.to_string();
			let (left, right) = stone_to_string.split_at(stone_to_string.len() / 2);
			result.push(left.parse::<i64>().unwrap());
			result.push(right.parse::<i64>().unwrap());
		} else {
			result.push(stone * 2024);
		}
	}
	result
}

fn has_even_length(text: &str) -> bool {
	text.len() % 2 == 0
}
