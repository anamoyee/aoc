#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

fn is_x_mas_at(pos: (usize, usize), board: &[&str]) -> bool {
	// assuming pos is on the boundary of board, caller is responsible for proper checking

	let char11 = board[pos.0 + 1].chars().nth(pos.1 + 1).unwrap();
	let char1_1 = board[pos.0 + 1].chars().nth(pos.1 - 1).unwrap();
	let char_11 = board[pos.0 - 1].chars().nth(pos.1 + 1).unwrap();
	let char_1_1 = board[pos.0 - 1].chars().nth(pos.1 - 1).unwrap();

	if !((char_1_1 == 'M' && char11 == 'S') || (char_1_1 == 'S' && char11 == 'M')) {
		return false;
	}

	if !((char1_1 == 'M' && char_11 == 'S') || (char1_1 == 'S' && char_11 == 'M')) {
		return false;
	}

	true
}

fn main() {
	let input = input!();

	let board = input.lines().collect::<Vec<_>>();

	assert!(board.iter().all(|line| line.len() == board[0].len())); // Same strlen for every row check

	let mut x_mas_amount = 0;

	for (i, &line) in board.iter().enumerate() {
		if i == 0 || i == board.len() - 1 {
			continue;
		}
		for (j, chr) in line.chars().enumerate() {
			if j == 0 || j == line.len() - 1 {
				continue;
			}

			if chr != 'A' {
				continue;
			}

			if is_x_mas_at((i, j), &board) {
				x_mas_amount += 1;
			}

			println!("{},{}", i, j)
		}
	}

	println!("{}", format!("{:#?}", x_mas_amount).bright_blue().bold());
}
