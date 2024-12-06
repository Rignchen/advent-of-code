fn main() {
	let input = get_input();
	let (mut guard, mut guard_direction, obstacles) = get_positions(&input);
	let mut is_guard_out = false;
	let max_x = input.lines().next().unwrap().len() as i32;
	let max_y = input.lines().count() as i32;
	let mut viewed_positions: Vec<Position> = Vec::new();

	while !is_guard_out {
		if !viewed_positions.iter().any(|x| x.compare(&guard)) {
			viewed_positions.push(guard.clone());
		}
		let next = guard.get_relative(guard_direction.next());
		if obstacles.iter().any(|&x| x.compare(&next)) {
			guard_direction = guard_direction.turn_right();
		}
		guard.goto(guard_direction.next());
		is_guard_out = guard.x < 0 || guard.y < 0 || guard.x >= max_x || guard.y >= max_y;
	}

	println!("{}", viewed_positions.len());
}

fn get_input() -> String {
	let file = "data/input.txt";
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

#[derive(Debug, Copy, Clone)]
struct Position {
	pub x: i32,
	pub y: i32,
}
impl Position {
	fn goto(&mut self, direction: (i32, i32)) {
		self.x += direction.0;
		self.y += direction.1;
	}
	fn get_relative(&self, direction: (i32, i32)) -> Position {
		Position { x: self.x + direction.0, y: self.y + direction.1 }
	}
	fn compare(&self, other: &Position) -> bool {
		self.x == other.x && self.y == other.y
	}
}

#[derive(Debug)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}
impl Direction {
	fn next(&self) -> (i32, i32) {
		match self {
			Direction::Up => (0, -1),
			Direction::Right => (1, 0),
			Direction::Down => (0, 1),
			Direction::Left => (-1, 0),
		}
	}
	fn turn_right(&self) -> Direction {
		match self {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		}
	}
}
