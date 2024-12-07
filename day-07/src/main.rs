use std::fs;

use itertools::Itertools;

enum Operators {
	ADD,
	MULTIPLY,
	CONCAT,
}

fn parse_line(line: &str) -> Option<(i64, Vec<i64>)> {
	line.split_once(": ").map(|(a, n)| {
		(
			a.parse::<i64>().unwrap_or(0),
			n.split(" ")
				.map(|i| i.parse::<i64>().unwrap_or(0))
				.collect::<Vec<i64>>(),
		)
	})
}

fn test_with_operators(input: &String, operator_list: &Vec<Operators>) -> i64 {
	let mut total_sum = 0;
	for line in input.lines() {
		let (answer, numbers) = parse_line(line).unwrap();
		let operator_combinations = std::iter::repeat(operator_list.iter())
			.take(numbers.len() - 1)
			.multi_cartesian_product();
		for operator_list in operator_combinations {
			let mut subsum = numbers[0];
			for (i, operator) in operator_list.iter().enumerate() {
				match operator {
					Operators::ADD => subsum += numbers[i + 1],
					Operators::MULTIPLY => subsum *= numbers[i + 1],
					Operators::CONCAT => {
						subsum = format!("{}{}", subsum, numbers[i + 1])
							.parse::<i64>()
							.unwrap()
					}
				}
			}
			if subsum == answer {
				total_sum += answer;
				break;
			}
		}
	}
	total_sum
}

fn solution(input: &String) -> i64 {
	let operators = vec![Operators::ADD, Operators::MULTIPLY];
	test_with_operators(input, &operators)
}

fn solution2(input: &String) -> i64 {
	let operators = vec![Operators::ADD, Operators::MULTIPLY, Operators::CONCAT];
	test_with_operators(input, &operators)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 3749);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 11387);
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
