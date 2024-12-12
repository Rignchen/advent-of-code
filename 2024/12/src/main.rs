use std::collections::HashMap;

fn get_input() -> Vec<String> {
	let file = "data/example2.txt";
	let contents = std::fs::read_to_string(file).unwrap();
	contents.lines().map(|x| x.to_string()).collect()
}

fn main() {
	let input = get_parcels(&get_input());
	for (parcel, coords) in input.iter() {
		println!("Parcel: {}", parcel);
		println!("\tTotal perimeter: {}", get_perimeter(coords));
		let parcels = get_parcels_distinct(coords);
		for (i, p) in parcels.iter().enumerate() {
			println!("\tParcel {}", i);
			println!("\t\tPerimeter: {}", get_perimeter(p));
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
