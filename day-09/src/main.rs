use std::fs;

fn parse_individual(input: &String) -> Vec<Option<i64>> {
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
	buffer
}

fn calculate_checksum(buffer: &Vec<Option<i64>>) -> i64 {
	buffer
		.iter()
		.enumerate()
		.fold(0, |prev, (i, el)| prev + (i as i64) * el.unwrap_or(0))
}

fn solution(input: &String) -> i64 {
	let mut buffer = parse_individual(input);
	for i in (0..buffer.len()).rev() {
		if let Some(_) = buffer[i] {
			let first_empty_index = buffer.iter().position(|b| b.is_none());
			if first_empty_index.is_none() || first_empty_index.unwrap() > i {
				break;
			}
			buffer.swap(i, first_empty_index.unwrap());
		}
	}
	calculate_checksum(&buffer)
}

fn solution2(input: &String) -> i64 {
	let mut buffer = parse_individual(input);
	let mut search_backwards_from = buffer.len() as i64;
	loop {
		let file_end = (0..search_backwards_from)
			.rev()
			.find(|&i| buffer[i as usize].is_some())
			.unwrap() as i64;
		let file_start = (0..file_end).rev().find(|&i| {
			i == 0 || buffer[i as usize].is_none_or(|c| c != buffer[file_end as usize].unwrap())
		});
		let file_start = match file_start {
			Some(n) => n + 1,
			None => break,
		};
		let file_length = file_end - file_start;
		let free_space_start = (0..file_start).find(|&i| match buffer[i as usize] {
			Some(_) => false,
			None => {
				for j in i + 1..i + 1 + file_length {
					if buffer[j as usize].is_some() {
						return false;
					}
				}
				true
			}
		});
		match free_space_start {
			Some(free_space_start) => {
				for i in 0..file_length + 1 {
					buffer.swap((file_start + i) as usize, (free_space_start + i) as usize);
				}
			}
			None => search_backwards_from = file_start,
		}
		if search_backwards_from <= 0 {
			break;
		}
	}
	calculate_checksum(&buffer)
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
		let input = String::from(r"2333133121414131402");
		let answer = solution2(&input);
		assert_eq!(answer, 2858);
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
