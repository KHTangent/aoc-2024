use std::{collections::HashSet, fs};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Eq, PartialEq, Clone)]
enum TileType {
	WALL,
	SPACE,
}
#[derive(Eq, PartialEq, Clone, Hash)]
enum Facing {
	UP,
	LEFT,
	RIGHT,
	DOWN,
}

fn test_map(map: &Vec<Vec<TileType>>, initial_pos: (usize, usize, Facing)) -> i64 {
	let (rows, cols) = (map.len(), map[0].len());
	let mut visited: HashSet<(usize, usize)> = HashSet::new();
	let mut current_guard_pos = initial_pos.clone();
	let mut visisted_with_direction: HashSet<(usize, usize, Facing)> = HashSet::new();
	loop {
		visited.insert((current_guard_pos.0, current_guard_pos.1));
		if visisted_with_direction.contains(&current_guard_pos) {
			return -1;
		}
		visisted_with_direction.insert(current_guard_pos.clone());

		match current_guard_pos {
			(x, y, Facing::UP) => {
				if y == 0 {
					break;
				}
				if map[y - 1][x] == TileType::WALL {
					current_guard_pos.2 = Facing::RIGHT;
				} else {
					current_guard_pos.1 -= 1;
				}
			}
			(x, y, Facing::LEFT) => {
				if x == 0 {
					break;
				}
				if map[y][x - 1] == TileType::WALL {
					current_guard_pos.2 = Facing::UP;
				} else {
					current_guard_pos.0 -= 1;
				}
			}
			(x, y, Facing::RIGHT) => {
				if x == cols - 1 {
					break;
				}
				if map[y][x + 1] == TileType::WALL {
					current_guard_pos.2 = Facing::DOWN;
				} else {
					current_guard_pos.0 += 1;
				}
			}
			(x, y, Facing::DOWN) => {
				if y == rows - 1 {
					break;
				}
				if map[y + 1][x] == TileType::WALL {
					current_guard_pos.2 = Facing::LEFT;
				} else {
					current_guard_pos.1 += 1;
				}
			}
		}
	}
	visited.len() as i64
}

fn solution(input: &String) -> i64 {
	let mut current_guard_pos = (0, 0, Facing::UP);
	let map: Vec<Vec<TileType>> = input
		.lines()
		.enumerate()
		.map(|(row, line)| {
			line.bytes()
				.enumerate()
				.map(|(col, c)| {
					if c == '^' as u8 {
						current_guard_pos = (col, row, Facing::UP);
					}
					match c as char {
						'#' => TileType::WALL,
						_ => TileType::SPACE,
					}
				})
				.collect()
		})
		.collect();
	test_map(&map, current_guard_pos)
}

fn solution2(input: &String) -> i64 {
	let mut current_guard_pos = (0, 0, Facing::UP);
	let map: Vec<Vec<TileType>> = input
		.lines()
		.enumerate()
		.map(|(row, line)| {
			line.bytes()
				.enumerate()
				.map(|(col, c)| {
					if c == '^' as u8 {
						current_guard_pos = (col, row, Facing::UP);
					}
					match c as char {
						'#' => TileType::WALL,
						_ => TileType::SPACE,
					}
				})
				.collect()
		})
		.collect();
	let total = (0..map.len())
		.into_par_iter()
		.map(|y| {
			let mut sum = 0;
			let mut map = map.clone();
			for x in 0..map[0].len() {
				if map[y][x] == TileType::WALL || current_guard_pos == (x, y, Facing::UP) {
					continue;
				}
				map[y][x] = TileType::WALL;
				if test_map(&map, current_guard_pos.clone()) == -1 {
					sum += 1;
				}
				map[y][x] = TileType::SPACE;
			}
			sum
		})
		.sum();
	total
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 41);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
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
