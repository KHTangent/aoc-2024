use std::{collections::HashSet, fs};

type Map = Vec<Vec<u8>>;
type PointSet = HashSet<(usize, usize)>;
struct Region {
	perimeter: i64,
	area: i64,
}
fn parse_map(input: &String) -> Map {
	input.lines().map(|l| l.bytes().collect()).collect()
}

fn traverse_tile(map: &Map, visited: &mut PointSet, x: usize, y: usize, region: &mut Region) {
	if visited.contains(&(x, y)) {
		return;
	}
	visited.insert((x, y));
	region.area += 1;
	let this_tile = map[y][x];
	if x > 0 && map[y][x - 1] == this_tile {
		traverse_tile(map, visited, x - 1, y, region);
	} else {
		region.perimeter += 1;
	}
	if x < map[y].len() - 1 && map[y][x + 1] == this_tile {
		traverse_tile(map, visited, x + 1, y, region);
	} else {
		region.perimeter += 1;
	}
	if y > 0 && map[y - 1][x] == this_tile {
		traverse_tile(map, visited, x, y - 1, region);
	} else {
		region.perimeter += 1;
	}
	if y < map.len() - 1 && map[y + 1][x] == this_tile {
		traverse_tile(map, visited, x, y + 1, region);
	} else {
		region.perimeter += 1;
	}
}

fn solution(input: &String) -> i64 {
	let mut sum: i64 = 0;
	let map = parse_map(input);
	let mut visited = PointSet::new();
	for y in 0..map.len() {
		for x in 0..map[y].len() {
			if visited.contains(&(x, y)) {
				continue;
			}
			let mut region = Region {
				perimeter: 0,
				area: 0,
			};
			traverse_tile(&map, &mut visited, x, y, &mut region);
			sum += region.area * region.perimeter;
		}
	}
	sum
}

fn solution2(input: &String) -> i64 {
	let mut sum = 0;
	for line in input.lines() {
		let num = line.parse::<i64>().unwrap_or(0);
		sum += num;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 1930);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"2
4
",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 6);
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
