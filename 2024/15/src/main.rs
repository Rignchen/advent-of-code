fn get_input() -> (Map, Vec<Direction>) {
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
 
	(Map {
		walls,
		boxes,
		robot: robot.expect("Robot not found"),
	}, movements.chars().filter(|c| *c != '\n').map(|c| match c {
		'>' => Direction::Right,
		'<' => Direction::Left,
		'^' => Direction::Up,
		'v' => Direction::Down,
		_ => panic!("Invalid direction: {}", c),
	}).collect())
}

fn main() {
	let (mut map, directions) = get_input();
	println!("{}", map.display());
	println!("{:?}", directions);
}

#[derive(Debug)]
struct Map {
	walls: Vec<Position>,
	boxes: Vec<Position>,
	robot: Position,
}
impl Map {
	fn display(&self) -> String {
		let width = self.walls.iter().map(|p| p.x).max().unwrap() as usize + 1;
		let height = self.walls.iter().map(|p| p.y).max().unwrap() as usize + 1;
		let mut map = vec![vec!['.'; width]; height];
		for wall in &self.walls {
			map[wall.y as usize][wall.x as usize] = '#';
		}
		for boxe in &self.boxes {
			if map[boxe.y as usize][boxe.x as usize] != '.' {
				panic!("a boxe is on a wall in position {:?}", boxe);
			}
			map[boxe.y as usize][boxe.x as usize] = 'O';
		}
		if map[self.robot.y as usize][self.robot.x as usize] != '.' {
			panic!("the robot is on a wall in position {:?}", self.robot);
		}
		map[self.robot.y as usize][self.robot.x as usize] = '@';
		map.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n")
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
	x: i32,
	y: i32,
}

#[derive(Debug, Clone)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}
