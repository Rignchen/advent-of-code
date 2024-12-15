use regex::Regex;

fn get_input() -> Vec<ClawMachine> {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	let contents = contents.split("\n\n").collect::<Vec<&str>>();
	contents.iter().map(|x| ClawMachine::new(x)).collect()
}

fn main() {
	let input = get_input();
	println!("{}",
	         input.iter()
	         .map(|x| x.solve())
	         .filter(|x| x.is_some()).map(|x| x.unwrap())
	         .filter(|(a, b)| *a >= 0 && *b >= 0)
	         .map(|(n, m)| n*3 + m)
	         .sum::<i32>());
}

#[derive(Debug)]
struct ClawMachine {
	button_a: (i32, i32),
	button_b: (i32, i32),
	prize: (i32, i32),
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
			button_a: (
				caps.get(1).unwrap().as_str().parse().unwrap(),
				caps.get(2).unwrap().as_str().parse().unwrap()
			),
			button_b: (
				caps.get(3).unwrap().as_str().parse().unwrap(),
				caps.get(4).unwrap().as_str().parse().unwrap()
			),
			prize: (
				caps.get(5).unwrap().as_str().parse().unwrap(),
				caps.get(6).unwrap().as_str().parse().unwrap()
			),
		}
	}

	fn solve(&self) -> Option<(i32, i32)> {
		let determinant = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;
		if determinant == 0 {
			return None
		}

		// Calculate A and B using Cramer's rule
		let a = (self.button_b.1 * self.prize.0 - self.button_b.0 * self.prize.1) / determinant;
		let b = (self.button_a.0 * self.prize.1 - self.button_a.1 * self.prize.0) / determinant;

		// check result
		if
			a * self.button_a.0 + b * self.button_b.0 == self.prize.0 &&
			a * self.button_a.1 + b * self.button_b.1 == self.prize.1 {
			Some((a, b))
		} else {
			None
		}
	}
}

