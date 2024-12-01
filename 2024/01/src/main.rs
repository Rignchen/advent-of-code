fn main() {
    println!("Hello, world!");
}

/// Get the input from the input.txt file
fn get_input() -> String {
	std::fs::read_to_string("input.txt").unwrap()
}

