use std::{cell::LazyCell, fs, str::FromStr};

use regex::Regex;

#[derive(Debug)]
struct Robot {
	pub position: (i64, i64),
	pub velocity: (i64, i64),
}

impl Robot {
	pub fn move_once(&mut self, map_width: i64, map_height: i64) {
		self.position.0 = (self.position.0 + map_width + self.velocity.0) % map_width;
		self.position.1 = (self.position.1 + map_height + self.velocity.1) % map_height;
	}
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Robot {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let re = LazyCell::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());
		let (_, [px, py, vx, vy]) = re.captures(s).ok_or(ParseError)?.extract();
		Ok(Robot {
			position: (px.parse().unwrap(), py.parse().unwrap()),
			velocity: (vx.parse().unwrap(), vy.parse().unwrap()),
		})
	}
}

fn solution(input: &String, map_width: i64, map_height: i64) -> i64 {
	let seconds = 100;
	let mut robots: Vec<Robot> = input.lines().map(|s| s.parse::<Robot>().unwrap()).collect();
	for _ in 0..seconds {
		robots
			.iter_mut()
			.for_each(|r| r.move_once(map_width, map_height));
	}
	let mut quadrant_scores = (0, 0, 0, 0);
	let map_middle_x = map_width / 2;
	let map_middle_y = map_height / 2;
	for robot in robots {
		if robot.position.0 > map_middle_x {
			if robot.position.1 > map_middle_y {
				quadrant_scores.0 += 1;
			} else if robot.position.1 < map_middle_y {
				quadrant_scores.1 += 1;
			}
		} else if robot.position.0 < map_middle_x {
			if robot.position.1 > map_middle_y {
				quadrant_scores.2 += 1;
			} else if robot.position.1 < map_middle_y {
				quadrant_scores.3 += 1;
			}
		}
	}
	quadrant_scores.0 * quadrant_scores.1 * quadrant_scores.2 * quadrant_scores.3
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
			r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
		);
		let answer = solution(&input, 11, 7);
		assert_eq!(answer, 12);
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
	let answer = solution(&file, 101, 103);
	println!("Answer task 1: {}", answer);
	let answer = solution2(&file);
	println!("Answer task 2: {}", answer);
}
