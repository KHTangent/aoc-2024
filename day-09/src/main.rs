use std::fs;

fn solution(input: &String) -> i64 {
	let mut buffer: Vec<Option<i64>> = Vec::with_capacity(input.len() * 7);
	for (index, char) in input.trim().bytes().enumerate() {
		let char_number = char - '0' as u8;
		for _ in 0..char_number {
			match index % 2 {
				0 => {
					buffer.push(Some(index as i64 / 2));
				}
				_ => {
					buffer.push(None);
				}
			}
		}
	}
	for i in (0..buffer.len()).rev() {
		if let Some(_) = buffer[i] {
			let first_empty_index = buffer.iter().position(|b| b.is_none());
			if first_empty_index.is_none() || first_empty_index.unwrap() > i {
				break;
			}
			buffer.swap(i, first_empty_index.unwrap());
		}
	}
	buffer
		.iter()
		.enumerate()
		.fold(0, |prev, (i, el)| prev + (i as i64) * el.unwrap_or(0))
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
		let input = String::from(r"2333133121414131402");
		let answer = solution(&input);
		assert_eq!(answer, 1928);
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
