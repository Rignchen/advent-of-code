use std::collections::HashMap;

fn get_input() -> Vec<String> {
	let file = "data/example.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents.lines().map(|x| x.to_string()).collect()
}

fn main() {
	let input = get_parcels(&get_input());
   println!("{:?}", input);
}

fn get_parcels(map: &Vec<String>) -> HashMap<char, Vec<(i32, i32)>> {
	let mut parcels = HashMap::new();
	for (y, line) in map.iter().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c.is_alphabetic() {
				parcels.entry(c).or_insert(vec![]).push((x as i32, y as i32));
			}
		}
	}
	parcels
}
