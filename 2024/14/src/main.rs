use regex::Regex;

fn get_input() -> Map {
	let map_size = (11, 7);
	let file = "data/example.txt";
 
	let contents = std::fs::read_to_string(file).unwrap();
	Map::new(contents.lines().map(|line| Robot::new(line)).collect(), map_size)
}

fn main() {
	let mut map = get_input();
	// after 100 seconds
	map.wait(100);
	println!("{}", map.display());
	let quadrants = map.split_quadriants();

	println!("Top left:     {}\n{}", quadrants.0.len(), quadrants.0.display());
	println!("Top right:    {}\n{}", quadrants.1.len(), quadrants.1.display());
	println!("Bottom left:  {}\n{}", quadrants.2.len(), quadrants.2.display());
	println!("Bottom right: {}\n{}", quadrants.3.len(), quadrants.3.display());

	println!("Safety factor: {}", quadrants.0.len() * quadrants.1.len() * quadrants.2.len() * quadrants.3.len());
}

#[derive(Debug, Clone, Copy)]
struct Robot {
	position: (i32, i32),
	velocity: (i32, i32),
}
impl Robot {
	fn new(input: &str) -> Robot {
		let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
		let caps = re.captures(input).unwrap();
		Robot {
			position: (
				caps[1].parse().unwrap(),
				caps[2].parse().unwrap(),
			),
			velocity: (
				caps[3].parse().unwrap(),
				caps[4].parse().unwrap(),
			),
		}
	}

	fn move_robot(&mut self, seconds: i32, map_size: (i32, i32)) {
		self.position.0 += self.velocity.0 * seconds;
		self.position.1 += self.velocity.1 * seconds;
 
		self.position.0 %= map_size.0;
		self.position.1 %= map_size.1;
 
		if self.position.0 < 0 {
			self.position.0 += map_size.0;
		}
		if self.position.1 < 0 {
			self.position.1 += map_size.1;
		}
	}

	fn get_quadrant(&self, map_size: (i32, i32)) -> Quadrant {
		// is the robot in the middle of the x or y axis?
		if self.position.0 == map_size.0 / 2 || self.position.1 == map_size.1 / 2 {
			Quadrant::Middle
		} else {
			match (self.position.0 < map_size.0 / 2, self.position.1 < map_size.1 / 2) {
				(true, true) => Quadrant::TopLeft,
				(true, false) => Quadrant::BottomLeft,
				(false, true) => Quadrant::TopRight,
				(false, false) => Quadrant::BottomRight,
			}
		}
	}
}

struct Map {
	robots: Vec<Robot>,
	map_size: (i32, i32),
}
impl Map {
	fn new(robots: Vec<Robot>, map_size: (i32, i32)) -> Map {
		let mut robots = robots.clone();
		// adjust robot positions to be in the map
		robots.iter_mut().for_each(|robot| {
			robot.position.0 %= map_size.0;
			robot.position.1 %= map_size.1;

			if robot.position.0 < 0 {
				robot.position.0 += map_size.0;
			}
			if robot.position.1 < 0 {
				robot.position.1 += map_size.1;
			}
		});
		Map { robots, map_size }
	}

	fn wait(&mut self, seconds: i32) {
		self.robots.iter_mut().for_each(|robot| robot.move_robot(seconds, self.map_size));
	}

	fn display(&self) -> String {
		let mut map = vec![vec![0 as u8; self.map_size.0 as usize]; self.map_size.1 as usize];
		let mut output = Vec::new();
		self.robots.iter()
			.for_each(|robot| {
				map[robot.position.1 as usize][robot.position.0 as usize] += 1;
			});
		map.iter().for_each(|row| {
			output.push(String::new());
			row.iter().for_each(|cell| {
				output.last_mut().unwrap().push(if *cell > 9 { '#' } else if *cell > 0 { (*cell).to_string().chars().next().unwrap() } else { '.' });
			});
		});
		output.join("\n")
	}

	fn len(&self) -> usize {
		self.robots.len()
	}

	fn split_quadriants(&self) -> (Map, Map, Map, Map, Map) {
		// split the map into 5 parts corresponding to the 5 quadrants
		let mut top_left = vec![];
		let mut top_right = vec![];
		let mut bottom_left = vec![];
		let mut bottom_right = vec![];
		let mut middle = vec![];

		self.robots.iter().for_each(|robot| {
			match robot.get_quadrant(self.map_size) {
				Quadrant::TopLeft => top_left.push(*robot),
				Quadrant::TopRight => top_right.push(*robot),
				Quadrant::BottomLeft => bottom_left.push(*robot),
				Quadrant::BottomRight => bottom_right.push(*robot),
				Quadrant::Middle => middle.push(*robot),
			}
		});

		(
			Map::new(top_left, (self.map_size.0 / 2, self.map_size.1 / 2)),
			Map::new(top_right, (self.map_size.0 / 2, self.map_size.1 / 2)),
			Map::new(bottom_left, (self.map_size.0 / 2, self.map_size.1 / 2)),
			Map::new(bottom_right, (self.map_size.0 / 2, self.map_size.1 / 2)),
			Map::new(middle, self.map_size),
		)
	}
}

#[derive(Debug, PartialEq)]
enum Quadrant {
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight,
	Middle,
}
