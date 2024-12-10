use std::{collections::HashMap, fs};

#[derive(Clone, Eq, PartialEq, Debug)]
enum Direction {
	NONE,
	UP,
	DOWN,
	LEFT,
	RIGHT,
}

type Map = Vec<Vec<u8>>;
type PointSet = HashMap<(usize, usize), i64>;
fn parse_map(input: &String) -> Map {
	input
		.lines()
		.map(|l| l.bytes().map(|b| b - b'0').collect())
		.collect()
}

fn traverse_find(map: &Map, x: usize, y: usize, came_from: Direction, found_tops: &mut PointSet) {
	if x > 0 && came_from != Direction::LEFT {
		if map[y][x] == 8 && map[y][x - 1] == 9 {
			match found_tops.get_mut(&(x - 1, y)) {
				Some(n) => {
					*n += 1;
				}
				None => {
					found_tops.insert((x - 1, y), 1);
				}
			};
		} else if map[y][x] + 1 == map[y][x - 1] {
			traverse_find(map, x - 1, y, Direction::RIGHT, found_tops);
		}
	}
	if x < map[0].len() - 1 && came_from != Direction::RIGHT {
		if map[y][x] == 8 && map[y][x + 1] == 9 {
			match found_tops.get_mut(&(x + 1, y)) {
				Some(n) => {
					*n += 1;
				}
				None => {
					found_tops.insert((x + 1, y), 1);
				}
			};
		} else if map[y][x] + 1 == map[y][x + 1] {
			traverse_find(map, x + 1, y, Direction::LEFT, found_tops);
		}
	}
	if y > 0 && came_from != Direction::UP {
		if map[y][x] == 8 && map[y - 1][x] == 9 {
			match found_tops.get_mut(&(x, y - 1)) {
				Some(n) => {
					*n += 1;
				}
				None => {
					found_tops.insert((x, y - 1), 1);
				}
			};
		} else if map[y][x] + 1 == map[y - 1][x] {
			traverse_find(map, x, y - 1, Direction::DOWN, found_tops);
		}
	}
	if y < map.len() - 1 && came_from != Direction::DOWN {
		if map[y][x] == 8 && map[y + 1][x] == 9 {
			match found_tops.get_mut(&(x, y + 1)) {
				Some(n) => {
					*n += 1;
				}
				None => {
					found_tops.insert((x, y + 1), 1);
				}
			};
		} else if map[y][x] + 1 == map[y + 1][x] {
			traverse_find(map, x, y + 1, Direction::UP, found_tops);
		}
	}
}

fn solution(input: &String) -> i64 {
	let mut sum = 0;
	let map = parse_map(input);
	for y in 0..map.len() {
		for x in 0..map[y].len() {
			if map[y][x] == 0 {
				let mut tops = PointSet::new();
				traverse_find(&map, x, y, Direction::NONE, &mut tops);
				sum += tops.len();
			}
		}
	}
	sum as i64
}

fn solution2(input: &String) -> i64 {
	let mut sum = 0;
	let map = parse_map(input);
	for y in 0..map.len() {
		for x in 0..map[y].len() {
			if map[y][x] == 0 {
				let mut tops = PointSet::new();
				traverse_find(&map, x, y, Direction::NONE, &mut tops);
				sum += tops.values().sum::<i64>();
			}
		}
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 36);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 81);
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
