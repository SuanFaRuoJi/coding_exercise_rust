use std::collections::HashMap;

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
	let mut i: usize = 0;
	let mut max: i32 = 0;
	while i < points.len() {
		let cur: i32 = max_for_one_points(&points[0..]);
		if cur > max {
			max = cur;
		}
		i += 1;
	}
	return max;
}

#[derive(Hash, PartialEq)]
struct fraction {
	x: i32,
	y: i32
}

impl Eq for fraction {}

fn max_for_one_points(points: &[Vec<i32>]) -> i32 {
	let mut slope_count_mapping: HashMap<fraction, i32> = HashMap::new();
	let mut vertical: i32 = 0; 
	let mut origin: i32 = 0;
	let anchor: Vec<i32> = vec![points[0][0], points[0][1]];
	let mut max: i32 = 0;
	for comp in points {
		let delta_x = comp[0] - anchor[0];
		let delta_y = comp[1] - anchor[1];
		if delta_x == 0 && delta_y == 0 {
			origin += 1;
			continue;
		}
		if delta_x == 0 {
			vertical += 1;
			if vertical > max {
				max = vertical;
			}
		} else {
			let delta_gcd = gcd(delta_x, delta_y);
			slope_count_mapping.entry(fraction{x: delta_x/delta_gcd, y: delta_y/delta_gcd})
				.and_modify(|val| {*val += 1;})
				.or_insert(2);
			let cur: i32 = *slope_count_mapping.get(&fraction{x: delta_x/delta_gcd, y: delta_y/delta_gcd}).unwrap();
			if cur > max {
				max = cur;
			}
		}
	}
	return max + origin;
}

fn gcd(x: i32, y: i32) -> i32 {
	if y == 0 {
		return x;
	}
	return gcd(y, x%y);
}
