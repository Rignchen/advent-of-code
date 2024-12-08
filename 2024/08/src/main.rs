fn get_input() -> (Vec<Antena>, (i32, i32)) {
	let file = "data/example.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	let mut lines = contents.lines();
	let map_size = (lines.clone().count() as i32, lines.next().unwrap().len() as i32);
	let mut antenas = Vec::new();
	for (y, line) in contents.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c != '.' {
				antenas.push(Antena {
					signal: c,
					x: x as i32,
					y: y as i32,
				});
			}
		}
	}
	(antenas, map_size)
}

fn main() {
	let input = get_input();
	println!("{:?}", input);
}

#[derive(Debug)]
struct Antena {
	signal: char,
	x: i32,
	y: i32,
}
