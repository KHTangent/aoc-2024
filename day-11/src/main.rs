use memoize::memoize;

#[memoize]
fn process_stone_compact(stone: i64, blinks: usize) -> i64 {
	if blinks == 0 {
		return 1;
	}
	let stone_string = stone.to_string();
	if stone == 0 {
		process_stone_compact(1, blinks - 1)
	} else if blinks != 0 && stone_string.len() % 2 == 0 {
		let (first, second) = stone_string.split_at(stone_string.len() / 2);
		let first_result = process_stone_compact(first.parse().unwrap(), blinks - 1);
		let second_result = process_stone_compact(second.parse().unwrap(), blinks - 1);
		first_result + second_result
	} else {
		process_stone_compact(stone * 2024, blinks - 1)
	}
}

fn solution(input: &str, blinks: usize) -> i64 {
	input
		.split(" ")
		.map(|s| s.parse::<i64>().unwrap())
		.map(|stone| process_stone_compact(stone, blinks))
		.sum::<i64>()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(r"125 17");
		let answer = solution(&input, 6);
		assert_eq!(answer, 22);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(r"125 17");
		let answer = solution(&input, 25);
		assert_eq!(answer, 55312);
	}
}

fn main() {
	let input = "3028 78 973951 5146801 5 0 23533 857";
	let answer = solution(&input, 25);
	println!("Answer task 1: {}", answer);
	let answer = solution(&input, 75);
	println!("Answer task 2: {}", answer);
}
