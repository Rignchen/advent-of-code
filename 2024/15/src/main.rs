fn get_input() -> Map {
	let file = "data/example.txt";
	let content = std::fs::read_to_string(file).unwrap();
	let mut content = content.split("\n\n");
	let contents = content.next().unwrap();
	let movements = content.next().unwrap();
 
	let mut walls: Vec<Position> = Vec::new();
	let mut boxes: Vec<Position> = Vec::new();
	let mut robot: Option<Position> = None;
	for (i, line) in contents.lines().enumerate() {
		for (j, c) in line.chars().enumerate() {
			let position = Position {
				x: j as i32,
				y: i as i32,
			};
			match c {
				'#' => walls.push(position),
				'.' => (),
				'O' => boxes.push(position),
				'@' => robot = Some(position),
				_ => panic!("Invalid map tile: {}", c),
			}
		}
	}
 
	Map {
		walls,
		boxes,
		robot: robot.expect("Robot not found"),
	}
}

fn main() {
	let input = get_input();
	println!("{:?}", input);
}

#[derive(Debug)]
struct Map {
	walls: Vec<Position>,
	boxes: Vec<Position>,
	robot: Position,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
	x: i32,
	y: i32,
}
