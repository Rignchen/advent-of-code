use std::collections::HashMap;

fn get_input() -> Vec<String> {
	let file = "data/input.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents.lines().map(|x| x.to_string()).collect()
}

fn main() {
	let input = get_parcels(&get_input());
	let mut result = 0;
	for p in input.values() {
		let parcels = get_parcels_distinct(p);
		for parcel in parcels {
			result += get_perimeter(&parcel) * parcel.len() as i32;
		}
	}
	println!("Result: {}", result);
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
