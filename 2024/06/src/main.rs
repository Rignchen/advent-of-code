fn main() {
	let input = get_input();
	let (guard, guard_direction, obstacles) = get_positions(&input);
	let map_size = (input.lines().next().unwrap().len() as i32, input.lines().count() as i32);
	let mut viewed_positions: Vec<Position> = Vec::new();

	walk(guard, guard_direction, obstacles, map_size, |guard, _| {
		if !viewed_positions.iter().any(|x| x.compare(&guard)) {
			viewed_positions.push(guard.clone());
		}
		false
	});

	println!("{}", viewed_positions.len());
}

fn walk(guard: Position, direction: Direction, obstacles: Vec<Position>, map: (i32,i32), mut early_break: impl FnMut(Position, Direction) -> bool) -> () {
	let mut is_out = false;
	let mut guard = guard;
	let mut direction = direction;
	while !is_out {
		let next = guard.get_relative(direction.next());
		if obstacles.iter().any(|&x| x == next) {
			direction = direction.turn_right();
		}
		if early_break(guard, direction) {
			break
		}
		guard.goto(direction.next());
		is_out = guard.x < 0 || guard.y < 0 || guard.x >= map.0 || guard.y >= map.1;
	};
}

fn get_input() -> String {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents
}

fn get_positions(input: &str) -> (Position, Direction, Vec<Position>) {
	let lines = input.lines();
	let mut guard = Position { x: 0, y: 0 }; 
	let mut guard_direction = Direction::Up;
	let mut obstacles: Vec<Position> = Vec::new();

	for (y, line) in lines.enumerate() {
		let chars = line.chars();
		for (x, c) in chars.enumerate() {
			let pos = Position { x: x as i32, y: y as i32 };
			match c {
				'^' => {
					guard = pos;
					guard_direction = Direction::Up;
				}
				'#' => obstacles.push(pos),
				_ => (),
			}
		}
	}

	(guard, guard_direction, obstacles)
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
}

#[derive(Debug, Copy, Clone)]
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
	fn turn_right(&self) -> Self {
		match self {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		}
	}
}
