use regex::Regex;

fn get_input() -> (Vec<Robot>, (i32, i32)) {
	let map_size = (11, 7);
	let file = "data/example.txt";

	let contents = std::fs::read_to_string(file).unwrap();
	(contents.lines().map(|line| Robot::new(line)).collect(), map_size)
}

fn main() {
	let (mut robots, map_size) = get_input();
   println!("{:?}", robots);
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
}
