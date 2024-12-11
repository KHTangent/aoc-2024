use memoize::memoize;

#[memoize]
fn process_stone(input: i64, blinks: usize, stones_per_recursion: usize) -> i64 {
	let mut generations_processed = 0;
	let mut stones = vec![input];
	for _ in 0..blinks.min(stones_per_recursion) {
		let mut new_stones: Vec<i64> = Vec::with_capacity(stones.len() * 2);
		for stone in stones {
			let stone_string = stone.to_string();
			if stone == 0 {
				new_stones.push(1);
			} else if stone_string.len() % 2 == 0 {
				let (first, second) = stone_string.split_at(stone_string.len() / 2);
				new_stones.push(first.parse().unwrap());
				new_stones.push(second.parse().unwrap());
			} else {
				new_stones.push(stone * 2024);
			}
		}
		stones = new_stones;
		generations_processed += 1;
	}
	if generations_processed < blinks {
		stones
			.into_iter()
			.map(|stone| {
				solution(
					stone.to_string().as_str(),
					blinks - stones_per_recursion,
					stones_per_recursion,
				)
			})
			.sum::<i64>()
	} else {
		stones.len() as i64
	}
}

fn solution(input: &str, blinks: usize, stones_per_recursion: usize) -> i64 {
	input
		.split(" ")
		.map(|s| s.parse::<i64>().unwrap())
		.map(|stone| process_stone(stone, blinks, stones_per_recursion))
		.sum::<i64>()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let input = String::from(r"125 17");
		let answer = solution(&input, 6, 20);
		assert_eq!(answer, 22);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(r"125 17");
		let answer = solution(&input, 25, 20);
		assert_eq!(answer, 55312);
	}
}

fn main() {
	let input = "3028 78 973951 5146801 5 0 23533 857";
	let answer = solution(&input, 25, 30);
	println!("Answer task 1: {}", answer);
	let answer = solution(&input, 75, 20);
	println!("Answer task 2: {}", answer);
}
