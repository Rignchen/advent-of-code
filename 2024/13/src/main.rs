use regex::Regex;

fn get_input() -> Vec<ClawMachine> {
	let file = "data/example.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	let contents = contents.split("\n\n").collect::<Vec<&str>>();
	contents.iter().map(|x| ClawMachine::new(x)).collect()
}

fn main() {
	let input = get_input();
	println!("{:?}", input);
}

#[derive(Debug)]
struct ClawMachine {
	button_x_1: i32,
	button_y_1: i32,
	button_x_2: i32,
	button_y_2: i32,
	prize_x: i32,
	prize_y: i32,
}
impl ClawMachine {
	fn new(input: &str) -> ClawMachine {
		/*
		 * Button A: X+94, Y+34
		 * Button B: X+22, Y+67
		 * Prize: X=8400, Y=5400
		 */
		let re = Regex::new(r"Button A: X\+?(-?\d+), Y\+?(-?\d+)\nButton B: X\+?(-?\d+), Y\+?(-?\d+)\nPrize: X=(-?\d+), Y=(-?\d+)").unwrap();
		let caps = re.captures(input).unwrap();
		ClawMachine {
			button_x_1: caps.get(1).unwrap().as_str().parse().unwrap(),
			button_y_1: caps.get(2).unwrap().as_str().parse().unwrap(),
			button_x_2: caps.get(3).unwrap().as_str().parse().unwrap(),
			button_y_2: caps.get(4).unwrap().as_str().parse().unwrap(),
			prize_x: caps.get(5).unwrap().as_str().parse().unwrap(),
			prize_y: caps.get(6).unwrap().as_str().parse().unwrap(),
		}
	}
}

