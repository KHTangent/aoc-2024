use itertools::Itertools;

struct Computer {
	reg_a: u64,
	reg_b: u64,
	reg_c: u64,
}

impl Computer {
	pub fn new(reg_a: u64, reg_b: u64, reg_c: u64) -> Computer {
		Computer {
			reg_a,
			reg_b,
			reg_c,
		}
	}

	pub fn run(&mut self, program: &Vec<(u64, u64)>) -> Vec<u64> {
		let mut ip: usize = 0;
		let mut out: Vec<u64> = vec![];
		loop {
			if ip >= program.len() {
				break;
			}
			let (opcode, literal_operand) = program[ip];
			let combo_operand = self.get_combo_operand(literal_operand);
			match opcode {
				0 => self.reg_a = self.reg_a >> combo_operand,
				1 => self.reg_b = self.reg_b ^ literal_operand,
				2 => self.reg_b = combo_operand % 8,
				3 => {
					if self.reg_a != 0 {
						ip = literal_operand as usize / 2;
						continue;
					}
				}
				4 => self.reg_b = self.reg_b ^ self.reg_c,
				5 => out.push(combo_operand % 8),
				6 => self.reg_b = self.reg_a >> combo_operand,
				7 => self.reg_c = self.reg_a >> combo_operand,
				_ => panic!(),
			}

			ip += 1;
		}
		out
	}

	fn get_combo_operand(&self, operand: u64) -> u64 {
		match operand {
			0 => 0,
			1 => 1,
			2 => 2,
			3 => 3,
			4 => self.reg_a,
			5 => self.reg_b,
			6 => self.reg_c,
			_ => panic!(),
		}
	}
}

fn solution(program: &str, reg_a: u64, reg_b: u64, reg_c: u64) -> Vec<u64> {
	let mut computer = Computer::new(reg_a, reg_b, reg_c);
	let program: Vec<(u64, u64)> = program
		.split(",")
		.map(|n| n.parse::<u64>().unwrap())
		.tuples()
		.collect();
	computer.run(&program)
}

fn solution2(input: &String) -> u64 {
	let mut sum = 0;
	for line in input.lines() {
		let num = line.parse::<u64>().unwrap_or(0);
		sum += num;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solution() {
		let program = String::from(r"0,1,5,4,3,0");
		let answer = solution(&program, 729, 0, 0);
		assert_eq!(answer, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
	}

	#[test]
	fn test2_solution() {
		let program = String::from(r"0,1,5,4,3,0");
		let answer = solution(&program, 2024, 0, 0);
		assert_eq!(answer, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
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
fn main() {
	let answer = solution("2,4,1,5,7,5,0,3,4,0,1,6,5,5,3,0", 46187030, 0, 0);
	let s: String = answer.iter().join(",");
	println!("Answer task 1: {}", s);
}
