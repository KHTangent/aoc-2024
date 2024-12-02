use std::fs;

fn is_safe(report: &Vec<i32>) -> bool {
	let is_increasing = report[1] - report[0] > 0;
	report.iter().enumerate().skip(1).all(|(i, _)| {
		let diff = report[i] - report[i - 1];
		match is_increasing {
			true => diff.is_positive() && diff < 4,
			false => diff.is_negative() && diff > -4,
		}
	})
}

fn parse_reports(input: &String) -> Vec<Vec<i32>> {
	input
		.lines()
		.map(|l| {
			l.split_whitespace()
				.map(|n| n.parse::<i32>().unwrap())
				.collect()
		})
		.collect()
}

fn solution(input: &String) -> i32 {
	let reports = parse_reports(input);
	reports.into_iter().filter(is_safe).count() as i32
}

fn solution2(input: &String) -> i32 {
	let reports = parse_reports(input);
	reports
		.into_iter()
		.filter(|report| {
			report.iter().enumerate().any(|(i, _)| {
				let mut without_i = report.clone();
				without_i.remove(i);
				is_safe(&without_i)
			})
		})
		.count() as i32
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 2);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 4);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let answer = solution(&get_entire_input_file());
	println!("Answer: {}", answer);
	let answer2 = solution2(&get_entire_input_file());
	println!("Answer 2: {}", answer2);
}
