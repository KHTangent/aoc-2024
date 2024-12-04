use std::fs;

fn solution(input: &String) -> i32 {
	let mut sum: i32 = 0;
	let mut lines = input
		.lines()
		.map(|s| s.to_string())
		.collect::<Vec<String>>();
	for _ in 0..4 {
		sum += lines
			.iter()
			.map(|line| line.matches("XMAS").count() as i32)
			.sum::<i32>();
		let line_length = lines[0].len();
		for row in 0..lines.len() - 3 {
			for col in 0..line_length - 3 {
				if lines[row].as_bytes()[col] == 'X' as u8
					&& lines[row + 1].as_bytes()[col + 1] == 'M' as u8
					&& lines[row + 2].as_bytes()[col + 2] == 'A' as u8
					&& lines[row + 3].as_bytes()[col + 3] == 'S' as u8
				{
					sum += 1;
				}
			}
		}
		lines = get_rotated(&lines);
	}
	sum
}

fn solution2(input: &String) -> i32 {
	let mut sum: i32 = 0;
	let lines = input
		.lines()
		.map(|s| s.as_bytes().to_vec())
		.collect::<Vec<Vec<u8>>>();
	let line_len = lines[0].len();
	for row in 1..lines.len() - 1 {
		for col in 1..line_len - 1 {
			if lines[row][col] != 'A' as u8 {
				continue;
			}
			if ((lines[row + 1][col + 1] == 'M' as u8 && lines[row - 1][col - 1] == 'S' as u8)
				|| (lines[row + 1][col + 1] == 'S' as u8 && lines[row - 1][col - 1] == 'M' as u8))
				&& ((lines[row - 1][col + 1] == 'M' as u8 && lines[row + 1][col - 1] == 'S' as u8)
					|| (lines[row - 1][col + 1] == 'S' as u8
						&& lines[row + 1][col - 1] == 'M' as u8))
			{
				sum += 1;
			}
		}
	}
	sum
}

fn get_rotated(input: &Vec<String>) -> Vec<String> {
	let row_length = input[0].len();
	let mut output: Vec<String> = vec![];
	for i in 0..row_length {
		let mut row: Vec<char> = input
			.iter()
			.map(|v| v.as_str().as_bytes()[i] as char)
			.collect();
		row.reverse();
		output.push(row.iter().collect());
	}
	output
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rotate() {
		let input = vec![String::from("ABC"), String::from("DEF")];
		let output = vec![String::from("DA"), String::from("EB"), String::from("FC")];
		assert_eq!(get_rotated(&input), output);
	}

	#[test]
	fn test_solution() {
		let input = String::from(
			r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 18);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(
			r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
",
		);
		let answer = solution2(&input);
		assert_eq!(answer, 9);
	}
}

fn get_entire_input_file() -> String {
	let filename = "input.txt";
	let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
	contents
}

fn main() {
	let answer = solution(&get_entire_input_file());
	let answer2 = solution2(&get_entire_input_file());
	println!("Answer: {}", answer);
	println!("Answer2: {}", answer2);
}
