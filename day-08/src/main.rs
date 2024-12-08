use itertools::Itertools;
use std::{
	collections::{HashMap, HashSet},
	fs,
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Point {
	x: i64,
	y: i64,
}
impl Point {
	pub fn new(x: i64, y: i64) -> Point {
		Point { x, y }
	}
	pub fn add(&self, rhs: &Point) -> Point {
		Point {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
	pub fn sub(&self, rhs: &Point) -> Point {
		Point {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
	pub fn scale(&self, scalar: i64) -> Point {
		Point {
			x: self.x * scalar,
			y: self.y * scalar,
		}
	}
	pub fn within(&self, lower_bounds: &Point, higher_bounds: &Point) -> bool {
		self.x >= lower_bounds.x
			&& self.x < higher_bounds.x
			&& self.y >= lower_bounds.y
			&& self.y < higher_bounds.y
	}
}

fn parse_antennas(input: &String) -> HashMap<char, Vec<Point>> {
	let mut map: HashMap<char, Vec<Point>> = HashMap::new();
	for (y, line) in input.lines().enumerate() {
		for (x, char) in line.chars().enumerate() {
			if char == '.' {
				continue;
			}
			if !map.contains_key(&char) {
				map.insert(char, vec![]);
			}
			map.get_mut(&char)
				.unwrap()
				.push(Point::new(x as i64, y as i64));
		}
	}
	map
}

fn solution(input: &String) -> i64 {
	let mut antinodes: HashSet<Point> = HashSet::new();
	let antenna_map = parse_antennas(&input);
	let map_width = input.find("\n").unwrap_or(0) as i64;
	let map_height = input.lines().count() as i64;
	let map_bounds_lower = Point::new(0, 0);
	let map_bounds_higher = Point::new(map_width, map_height);
	for (_frequency, antennas) in antenna_map.iter() {
		for (a1, a2) in antennas.iter().tuple_combinations() {
			let distance = a2.sub(a1);
			if a1
				.sub(&distance)
				.within(&map_bounds_lower, &map_bounds_higher)
			{
				antinodes.insert(a1.sub(&distance));
			}
			if a2
				.add(&distance)
				.within(&map_bounds_lower, &map_bounds_higher)
			{
				antinodes.insert(a2.add(&distance));
			}
		}
	}
	antinodes.len() as i64
}

fn solution2(input: &String) -> i64 {
	let mut antinodes: HashSet<Point> = HashSet::new();
	let antenna_map = parse_antennas(&input);
	let map_width = input.find("\n").unwrap_or(0) as i64;
	let map_height = input.lines().count() as i64;
	let map_bounds_lower = Point::new(0, 0);
	let map_bounds_higher = Point::new(map_width, map_height);
	for (_frequency, antennas) in antenna_map.iter() {
		for (a1, a2) in antennas.iter().tuple_combinations() {
			let distance = a2.sub(a1);
			for scale in 0.. {
				let maybe_antinode = a1.sub(&distance.scale(scale));
				if maybe_antinode.within(&map_bounds_lower, &map_bounds_higher) {
					antinodes.insert(maybe_antinode);
				} else {
					break;
				}
			}
			for scale in 0.. {
				let maybe_antinode = a2.add(&distance.scale(scale));
				if maybe_antinode.within(&map_bounds_lower, &map_bounds_higher) {
					antinodes.insert(maybe_antinode);
				} else {
					break;
				}
			}
		}
	}
	antinodes.len() as i64
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 14);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 34);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let file = get_entire_input_file();
	let answer = solution(&file);
	println!("Answer task 1: {}", answer);
	let answer = solution2(&file);
	println!("Answer task 2: {}", answer);
}
