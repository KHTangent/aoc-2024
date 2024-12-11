use std::fs;

fn solution(input: &str, blinks: usize) -> i64 {
	let mut stones: Vec<i64> = input
		.split(" ")
		.map(|s| s.parse::<i64>().unwrap())
		.collect();
	let mut last_len = stones.len();
	for generation in 0..blinks {
		println!(
			"Starting generation {}, have {} stones, increase by {}",
			generation,
			stones.len(),
			stones.len() as f64 / (last_len as f64)
		);
		last_len = stones.len();
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
	}
	stones.len() as i64
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
}

fn main() {
	let input = "3028 78 973951 5146801 5 0 23533 857";
	let answer = solution(&input, 25);
	println!("Answer task 1: {}", answer);
	// Will get killed by the OOM killer
	// let answer = solution(&input, 75);
	// println!("Answer task 2: {}", answer);
}
