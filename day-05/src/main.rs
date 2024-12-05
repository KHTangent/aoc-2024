use std::{
	collections::{HashMap, HashSet},
	fs,
};

type RuleMap = HashMap<i64, HashSet<i64>>;

fn create_rule_map(rules: &str) -> RuleMap {
	let mut not_preceded_by_map: HashMap<i64, HashSet<i64>> = HashMap::new();
	for rule in rules.lines() {
		let (before, after) = rule
			.split_once("|")
			.map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
			.unwrap();
		if !not_preceded_by_map.contains_key(&before) {
			not_preceded_by_map.insert(before, HashSet::new());
		}
		not_preceded_by_map.get_mut(&before).unwrap().insert(after);
	}
	not_preceded_by_map
}

fn check_line(row: &Vec<i64>, map: &RuleMap) -> bool {
	let mut accepted = true;
	let mut banned_numbers: HashSet<i64> = HashSet::new();
	for n in row.iter().rev() {
		if banned_numbers.contains(n) {
			accepted = false;
			break;
		}
		if let Some(banned) = map.get(n) {
			banned.iter().for_each(|&banned_number| {
				banned_numbers.insert(banned_number);
			});
		}
	}
	accepted
}

fn solution(input: &String) -> i64 {
	let mut sum = 0;
	let (rules, rows) = input.split_once("\n\n").unwrap();
	let not_preceded_by_map = create_rule_map(rules);
	let parsed_lines: Vec<Vec<i64>> = rows
		.lines()
		.map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
		.collect();
	for row in parsed_lines {
		if check_line(&row, &not_preceded_by_map) {
			sum += row[row.len() / 2];
		}
	}
	sum
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
		let input = String::from(
			r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
		);
		let answer = solution(&input);
		assert_eq!(answer, 143);
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
