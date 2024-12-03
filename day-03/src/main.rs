use regex::Regex;
use std::cell::LazyCell;
use std::fs;

fn solution(input: &String) -> i32 {
	let mut sum = 0;
	let re = LazyCell::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
	for (_, [l, r]) in re.captures_iter(input).map(|c| c.extract()) {
		sum += l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap();
	}
	sum
}

fn solution2(input: &String) -> i32 {
	let mut cloned = input.clone();
	loop {
		let remove_start = match cloned.find("don't()") {
			Some(i) => i,
			None => break,
		};
		let remove_end = match cloned[remove_start..].find("do()") {
			Some(i) => remove_start + i,
			None => cloned.len(),
		};
		cloned.replace_range(remove_start..remove_end, "");
	}

	solution(&cloned)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
		);
		let answer = solution(&input);
		assert_eq!(answer, 161);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 48);
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
	println!("Answer2: {}", answer2);
}
