fn main() {
	let input = get_input();
	println!("{}", input);
}

fn get_input() -> String {
	let file = "data/example.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents
}
