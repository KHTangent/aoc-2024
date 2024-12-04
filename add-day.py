#!/usr/bin/env python3

from os import mkdir
from datetime import datetime
from sys import argv


file_main_rs = """use std::fs;

fn solution(input: &String) -> i64 {
	let mut sum = 0;
	for line in input.lines() {
		let num = line.parse::<i64>().unwrap_or(0);
		sum += num;
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
		let input = String::from(r"
");
		let answer = solution(&input);
		assert_eq!(answer, 6);
	}

	#[test]
	fn test_solution2() {
		let input = String::from(r"2
4
");
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
"""

file_Cargo_toml = """[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"""

file_rustfmt_toml = "hard_tabs = true"


def main():
    if len(argv) > 1:
        day_of_month = int(argv[1])
    else:
        day_of_month = datetime.now().day
    dir_name = f"day-{day_of_month:02}"
    mkdir(dir_name)
    mkdir(f"{dir_name}/src")
    with open(f"{dir_name}/src/main.rs", "w") as f:
        f.write(file_main_rs)
    with open(f"{dir_name}/Cargo.toml", "w") as f:
        f.write(file_Cargo_toml.format(dir_name))
    with open(f"{dir_name}/input.txt", "w") as f:
        pass
    with open(f"{dir_name}/rustfmt.toml", "w") as f:
        f.write(file_rustfmt_toml)
    print(f"Created {dir_name}")

if __name__ == "__main__":
    main()
