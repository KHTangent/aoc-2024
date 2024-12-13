use std::{cell::LazyCell, fs, str::FromStr};

use regex::Regex;

#[derive(Debug)]
struct Arcade {
	pub button_a: (i64, i64),
	pub button_b: (i64, i64),
	pub prize: (i64, i64),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl Arcade {
	fn lowest_winning(&self) -> i64 {
		let mut lowest_price = i64::max_value();
		for presses_b in 0..100 {
			for presses_a in 0..100 {
				if self.button_a.0 * presses_a + self.button_b.0 * presses_b == self.prize.0
					&& self.button_a.1 * presses_a + self.button_b.1 * presses_b == self.prize.1
				{
					let price: i64 = presses_a * 3 + presses_b;
					if price < lowest_price {
						lowest_price = price;
					}
				}
			}
		}
		lowest_price
	}
}

impl FromStr for Arcade {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let re = LazyCell::new(|| {
			Regex::new(
				r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X\=(\d+), Y\=(\d+)",
			)
			.unwrap()
		});
		let (_, [ax, ay, bx, by, px, py]) = re.captures(s).ok_or(ParseError)?.extract();
		Ok(Arcade {
			button_a: (ax.parse().unwrap(), ay.parse().unwrap()),
			button_b: (bx.parse().unwrap(), by.parse().unwrap()),
			prize: (px.parse().unwrap(), py.parse().unwrap()),
		})
	}
}

fn solution(input: &String) -> i64 {
	input
		.split("\n\n")
		.map(|a| a.parse::<Arcade>().unwrap().lowest_winning())
		.filter(|&n| n < i64::max_value())
		.sum()
}

fn solution2(input: &String) -> i64 {
	input
		.split("\n\n")
		.map(|a| a.parse::<Arcade>().unwrap())
		.map(|a| {
			(Arcade {
				button_a: a.button_a,
				button_b: a.button_b,
				prize: (a.prize.0 + 10000000000000, a.prize.1 + 10000000000000),
			})
			.lowest_winning()
		})
		.filter(|&n| n < i64::max_value())
		.sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 480);
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
