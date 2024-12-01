use std::fs;

fn solution(input: &String) -> i32 {
	let mut lists: (Vec<i32>, Vec<i32>) = input
		.lines()
		.map(|line| line.split_once("   ").unwrap_or(("0", "0")))
		.map(|(n1, n2)| (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap()))
		.unzip();
	lists.0.sort();
	lists.1.sort();
	lists
		.0
		.into_iter()
		.zip(lists.1.into_iter())
		.fold(0, |sum, (a, b)| sum + (a - b).abs())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(
			r"3   4
4   3
2   5
1   3
3   9
3   3
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 11);
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
}
