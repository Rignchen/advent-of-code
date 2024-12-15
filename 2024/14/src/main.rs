use regex::Regex;

fn get_input() -> (Vec<Robot>, (i32, i32)) {
	let map_size = (11, 7);
	let file = "data/example.txt";

	let contents = std::fs::read_to_string(file).unwrap();
	(contents.lines().map(|line| Robot::new(line)).collect(), map_size)
}

fn main() {
	let (mut robots, map_size) = get_input();
	// after 100 seconds
	robots.iter_mut().for_each(|robot| robot.move_robot(100, map_size));
	print_map(&robots, map_size);
	println!("top left:     {}", robots.iter().filter(|robot| robot.get_quadrant(map_size) == Quadrant::TopLeft).count());
	println!("top right:    {}", robots.iter().filter(|robot| robot.get_quadrant(map_size) == Quadrant::TopRight).count());
	println!("bottom left:  {}", robots.iter().filter(|robot| robot.get_quadrant(map_size) == Quadrant::BottomLeft).count());
	println!("bottom right: {}", robots.iter().filter(|robot| robot.get_quadrant(map_size) == Quadrant::BottomRight).count());
}

fn print_map(robots: &Vec<Robot>, map_size: (i32, i32)) {
	let mut map = vec![vec![0; map_size.0 as usize]; map_size.1 as usize];
	robots.iter()
		.for_each(|robot| {
		map[robot.position.1 as usize][robot.position.0 as usize] += 1;
	});
	map.iter().for_each(|row| {
		row.iter().for_each(|&cell| {
			print!("{}", if cell == 0 { '.' } else { cell.to_string().chars().next().unwrap() });
		});
		println!();
	});
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
enum Quadrant {
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight,
	Middle,
}
