use std::collections::HashMap;

fn get_input() -> Vec<String> {
	let file = "data/example2.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents.lines().map(|x| x.to_string()).collect()
}

fn main() {
	let input = get_parcels(&get_input());
	for (c,p) in input.iter() {
		println!("Parcel: {}", c);
		let parcels = get_parcels_distinct(p);
		for parcel in parcels {
			println!("\t{:?}", parcel);
			println!("\t\tSides: {}", get_sides_count(&parcel));
		}
	}
}

fn get_parcels(map: &Vec<String>) -> HashMap<char, Vec<(i32, i32)>> {
	let mut parcels = HashMap::new();
	for (y, line) in map.iter().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c.is_alphabetic() {
				parcels.entry(c).or_insert(vec![]).push((x as i32, y as i32));
			}
		}
	}
	parcels
}

fn get_parcels_distinct(map: &Vec<(i32, i32)>) -> Vec<Vec<(i32, i32)>> {
	let mut parcels = Vec::<Vec<(i32,i32)>>::new();
	for p in map {
		let mut nighbours = Vec::<(usize, Vec<(i32,i32)>)>::new();
		for (i,n) in parcels.iter().enumerate() {
			if n.contains(&(p.0 + 1, p.1)) || n.contains(&(p.0 - 1, p.1)) || n.contains(&(p.0, p.1 + 1)) || n.contains(&(p.0, p.1 - 1)) {
				nighbours.push((i, n.clone()));
			}
		}
		if nighbours.len() == 0 {
			parcels.push(vec![*p]);
		} else if nighbours.len() == 1 {
			parcels[nighbours[0].0].push(*p);
		} else {
			for i in 1..nighbours.len() {
				let ng = parcels[nighbours[i].0].clone();
				parcels[nighbours[0].0].extend(ng);
				parcels.remove(nighbours[i].0);
			}
			parcels[nighbours[0].0].push(*p);
		}
	}
	parcels
}

fn get_perimeter(parcel: &Vec<(i32, i32)>) -> i32 {
	let mut perimeter = 0;
	for (x, y) in parcel.iter() {
		if !parcel.contains(&(x + 1, *y)) {
			perimeter += 1;
		}
		if !parcel.contains(&(x - 1, *y)) {
			perimeter += 1;
		}
		if !parcel.contains(&(*x, y + 1)) {
			perimeter += 1;
		}
		if !parcel.contains(&(*x, y - 1)) {
			perimeter += 1;
		}
	}
	perimeter
}

fn get_sides_count(parcel: &Vec<(i32, i32)>) -> i32 {
	let mut sides = Vec::<(Direction, Vec<(i32, i32)>)>::new();
	for (x, y) in parcel.iter() {
		// Right
		if !parcel.contains(&(x + 1, *y)) {
			let filtered_sides: Vec<(Vec<(i32, i32)>, usize)> = sides.iter().zip(0..).filter(|s| s.0.0 == Direction::Right).map(|s| (s.0.1.clone(), s.1)).collect();
			let mut has_side = false;
			for s in filtered_sides {
				if s.0.contains(&(*x, *y - 1)) {
					sides[s.1].1.push((*x, *y));
					has_side = true;
				}
			}
			if !has_side {
				sides.push((Direction::Right, vec![(*x, *y)]));
			}
		}
		// Left
		if !parcel.contains(&(x - 1, *y)) {
			let filtered_sides: Vec<(Vec<(i32, i32)>, usize)> = sides.iter().zip(0..).filter(|s| s.0.0 == Direction::Left).map(|s| (s.0.1.clone(), s.1)).collect();
			let mut has_side = false;
			for s in filtered_sides {
				if s.0.contains(&(*x, *y - 1)) {
					sides[s.1].1.push((*x, *y));
					has_side = true;
				}
			}
			if !has_side {
				sides.push((Direction::Left, vec![(*x, *y)]));
			}
		}
		// Down
		if !parcel.contains(&(*x, y + 1)) {
			let filtered_sides: Vec<(Vec<(i32, i32)>, usize)> = sides.iter().zip(0..).filter(|s| s.0.0 == Direction::Down).map(|s| (s.0.1.clone(), s.1)).collect();
			let mut has_side = false;
			for s in filtered_sides {
				if s.0.contains(&(*x - 1, *y)) {
					sides[s.1].1.push((*x, *y));
					has_side = true;
				}
			}
			if !has_side {
				sides.push((Direction::Down, vec![(*x, *y)]));
			}
		}
		// Up
		if !parcel.contains(&(*x, y - 1)) {
			let filtered_sides: Vec<(Vec<(i32, i32)>, usize)> = sides.iter().zip(0..).filter(|s| s.0.0 == Direction::Up).map(|s| (s.0.1.clone(), s.1)).collect();
			let mut has_side = false;
			for s in filtered_sides {
				if s.0.contains(&(*x - 1, *y)) {
					sides[s.1].1.push((*x, *y));
					has_side = true;
				}
			}
			if !has_side {
				sides.push((Direction::Up, vec![(*x, *y)]));
			}
		}
	}
	sides.len() as i32
}

#[derive(PartialEq)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}
