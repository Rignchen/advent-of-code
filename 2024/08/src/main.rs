use std::collections::HashMap;

fn get_input() -> (Vec<(char, Position)>, (i32, i32)) {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	let mut lines = contents.lines();
	let map_size = (lines.clone().count() as i32, lines.next().unwrap().len() as i32);
	let mut antenas = Vec::new();
	for (y, line) in contents.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c != '.' {
				antenas.push((c, Position { x: x as i32, y: y as i32 }));
			}
		}
	}
	(antenas, map_size)
}

fn main() {
	let (antenas, map_size) = get_input();
	let mut antena_map = HashMap::<char, Vec<Position>>::new();
	for antena in antenas {
		antena_map.entry(antena.0).or_insert(Vec::new()).push(antena.1);
	}
	let mut antinode = Vec::<Position>::new();

	for positions in antena_map.values() {
		for position in positions {
			for other_position in positions {
				if position == other_position {
					continue;
				}
				let vector = position.vector(other_position);
				let to_push = position.from_vector(vector);
				if !antinode.contains(&to_push) && to_push.in_map(map_size) {
					antinode.push(to_push);
				}
				let to_push = other_position.from_vector(vector.reverse());
				if !antinode.contains(&to_push) && to_push.in_map(map_size) {
					antinode.push(to_push);
				}
			}
		}
	}
	println!("{:?}", antinode.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
	x: i32,
	y: i32,
}
impl Position {
	fn vector(&self, other: &Position) -> Vector {
		Vector {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
	fn from_vector(&self, vector: Vector) -> Self {
		Position {
			x: self.x + vector.x,
			y: self.y + vector.y,
		}
	}
	fn in_map(&self, map_size: (i32, i32)) -> bool {
		self.x >= 0 && self.y >= 0 && self.x < map_size.0 && self.y < map_size.1
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
	x: i32,
	y: i32,
}
impl Vector {
	fn reverse(&self) -> Vector {
		Vector {
			x: -self.x,
			y: -self.y,
		}
	}
}
