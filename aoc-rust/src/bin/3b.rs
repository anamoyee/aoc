#[allow(unused)]
use aoc::{input, input_test};
use colored::*;
use regex::Regex;

fn main() {
	let input = input!();

	let re_mul = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();
	let re_do = Regex::new(r#"do\(\)"#).unwrap();
	let re_dont = Regex::new(r#"don't\(\)"#).unwrap();

	let mut dos = re_do
		.find_iter(&input)
		.map(|x| x.start())
		.collect::<Vec<usize>>();

	dos.insert(0, 0);

	let mut donts = re_dont
		.find_iter(&input)
		.map(|x| x.start())
		.collect::<Vec<usize>>();

	donts.insert(0, 0);

	let mut out = 0;
	re_mul.captures_iter(&input).for_each(|caps| {
		let mul_start = caps.get(0).unwrap().start();
		let cloned_dos = dos.clone();
		let furthest_do = cloned_dos
			.iter()
			.filter(|&&x| x < mul_start)
			.next_back()
			.unwrap();

		let cloned_donts = donts.clone();
		let furthest_dont = cloned_donts
			.iter()
			.filter(|&&x| x < mul_start)
			.next_back()
			.unwrap();

		if furthest_dont <= furthest_do {
			out += caps[1].parse::<i32>().unwrap() * caps[2].parse::<i32>().unwrap();
		}
	});

	println!("{}", format!("{:#?}", out).bright_blue().bold())
}
