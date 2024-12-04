fn main() {
	let input = get_input();
	let result = find_mas_in_shape_x(&input);
	println!("result: {}", result);
}

fn get_input() -> String {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents
}

/// find all the "mas" shaped as an "X" in the input
/// ```text
/// M.S
/// .A.
/// S.M
/// ```
fn find_mas_in_shape_x(input: &str) -> i32 {
	let mut count = 0;
	let line_count = input.lines().count();
	let line_len = input.lines().next().unwrap().len();
	let lines = input.lines().collect::<Vec<&str>>();

	for i in 1..line_count-1 {
		for j in 1..line_len-1 {
			if lines[i].chars().nth(j).unwrap() == 'A' {
				if 
					  ((lines[i-1].chars().nth(j-1).unwrap() == 'M' && lines[i+1].chars().nth(j+1).unwrap() == 'S')
					|| (lines[i-1].chars().nth(j-1).unwrap() == 'S' && lines[i+1].chars().nth(j+1).unwrap() == 'M'))
					&&((lines[i-1].chars().nth(j+1).unwrap() == 'M' && lines[i+1].chars().nth(j-1).unwrap() == 'S')
					|| (lines[i-1].chars().nth(j+1).unwrap() == 'S' && lines[i+1].chars().nth(j-1).unwrap() == 'M'))
					{
						count += 1;
					}
			}
		}
	}
	count
}

/// find all instences of the word "xmas"
/// words can be horizontal, vertical, or diagonal
/// they can be written forwards or backwards
/// return the number of times the word "xmas" is found
fn find_xmas(input: &str) -> i32 {
	let mut count = 0;
	let xmas = "XMAS";
	let samx = "SAMX";
	let xmas_len = xmas.len();
	let line_count = input.lines().count();
	let line_len = input.lines().next().unwrap().len();

	// horizontal
	for line in input.lines() {
		println!("line: {}", line);
		for i in 0..line_len {
			if i + xmas_len - 1 < line_len {
				println!("line[i..i+xmas_len]: {}", &line[i..i+xmas_len]);
				if (&line[i..i+xmas_len] == xmas || &line[i..i+xmas_len] == samx) {
					count += 1;
				}
			}
		}
	}

	// vertical
	for i in 0..line_count {
		for j in 0..line_len {
			if i + xmas_len - 1 < line_count &&
				input.lines().nth(i).unwrap().chars().nth(j).unwrap() == xmas.chars().nth(0).unwrap() &&
				input.lines().nth(i+1).unwrap().chars().nth(j).unwrap() == xmas.chars().nth(1).unwrap() &&
				input.lines().nth(i+2).unwrap().chars().nth(j).unwrap() == xmas.chars().nth(2).unwrap() &&
				input.lines().nth(i+3).unwrap().chars().nth(j).unwrap() == xmas.chars().nth(3).unwrap() {
				count += 1;
			}
			if i + xmas_len - 1 < line_count &&
				input.lines().nth(i).unwrap().chars().nth(j).unwrap() == samx.chars().nth(0).unwrap() &&
				input.lines().nth(i+1).unwrap().chars().nth(j).unwrap() == samx.chars().nth(1).unwrap() &&
				input.lines().nth(i+2).unwrap().chars().nth(j).unwrap() == samx.chars().nth(2).unwrap() &&
				input.lines().nth(i+3).unwrap().chars().nth(j).unwrap() == samx.chars().nth(3).unwrap() {
				count += 1;
			}
		}
	}

	// diagonal (top left to bottom right)
	for i in 0..line_count {
		for j in 0..line_len {
			if i + xmas_len - 1 < line_count && j + xmas_len - 1 < line_len &&
				input.lines().nth(i).unwrap().chars().nth(j).unwrap() == xmas.chars().nth(0).unwrap() &&
				input.lines().nth(i+1).unwrap().chars().nth(j+1).unwrap() == xmas.chars().nth(1).unwrap() &&
				input.lines().nth(i+2).unwrap().chars().nth(j+2).unwrap() == xmas.chars().nth(2).unwrap() &&
				input.lines().nth(i+3).unwrap().chars().nth(j+3).unwrap() == xmas.chars().nth(3).unwrap() {
				count += 1;
			}
			if i + xmas_len - 1 < line_count && j + xmas_len - 1 < line_len &&
				input.lines().nth(i).unwrap().chars().nth(j).unwrap() == samx.chars().nth(0).unwrap() &&
				input.lines().nth(i+1).unwrap().chars().nth(j+1).unwrap() == samx.chars().nth(1).unwrap() &&
				input.lines().nth(i+2).unwrap().chars().nth(j+2).unwrap() == samx.chars().nth(2).unwrap() &&
				input.lines().nth(i+3).unwrap().chars().nth(j+3).unwrap() == samx.chars().nth(3).unwrap() {
				count += 1;
			}
		}
	}

	// diagonal (top right to bottom left)
	for i in 0..line_count {
		for j in 0..line_len {
			if i + xmas_len - 1 < line_count && j >= xmas_len - 1 &&
				input.lines().nth(i).unwrap().chars().nth(j).unwrap() == xmas.chars().nth(0).unwrap() &&
				input.lines().nth(i+1).unwrap().chars().nth(j-1).unwrap() == xmas.chars().nth(1).unwrap() &&
				input.lines().nth(i+2).unwrap().chars().nth(j-2).unwrap() == xmas.chars().nth(2).unwrap() &&
				input.lines().nth(i+3).unwrap().chars().nth(j-3).unwrap() == xmas.chars().nth(3).unwrap() {
				count += 1;
			}
			if i + xmas_len - 1 < line_count && j >= xmas_len - 1 &&
				input.lines().nth(i).unwrap().chars().nth(j).unwrap() == samx.chars().nth(0).unwrap() &&
				input.lines().nth(i+1).unwrap().chars().nth(j-1).unwrap() == samx.chars().nth(1).unwrap() &&
				input.lines().nth(i+2).unwrap().chars().nth(j-2).unwrap() == samx.chars().nth(2).unwrap() &&
				input.lines().nth(i+3).unwrap().chars().nth(j-3).unwrap() == samx.chars().nth(3).unwrap() {
				count += 1;
			}
		}
	}

	count
}

