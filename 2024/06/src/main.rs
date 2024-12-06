fn main() {
	let input = get_input();
	let (mut guard, mut guard_direction, obstacles) = get_positions(&input);
	println!("{:?}", guard);
	println!("{:?}", guard_direction);
	println!("{:?}", obstacles);
}

fn get_input() -> String {
	let file = "data/example.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents
}

fn get_positions(input: &str) -> (Position, Direction, Vec<Position>) {
	let mut lines = input.lines();
	let mut guard = Position { x: 0, y: 0 }; 
	let mut guard_direction = Direction::Up;
	let mut obstacles: Vec<Position> = Vec::new();

	for (i, line) in lines.enumerate() {
		let mut chars = line.chars();
		for (j, c) in chars.enumerate() {
			let pos = Position { x: j as i32, y: i as i32 };
			match c {
				'^' | 'v' | '<' | '>' => {
					guard = pos;
					guard_direction = match c {
						'^' => Direction::Up,
						'v' => Direction::Down,
						'<' => Direction::Left,
						'>' => Direction::Right,
						_ => panic!("Invalid direction"),
					};
				}
				'#' => obstacles.push(pos),
				_ => (),
			}
		}
	}

	(guard, guard_direction, obstacles)
}

#[derive(Debug)]
struct Position {
	pub x: i32,
	pub y: i32,
}
#[derive(Debug)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}
