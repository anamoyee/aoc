#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

fn char_to_color(c: &char) -> colored::Color {
    let value = match c {
        '0'..='9' => *c as u8 - b'0',
        'a'..='z' => *c as u8 - b'a' + 10,
        'A'..='Z' => *c as u8 - b'A' + 36,
        _ => 0,
    } + 100;

    let r = value.wrapping_mul(199);
    let g = value.wrapping_mul(227);
    let b = value.wrapping_mul(61);

    colored::Color::TrueColor { r, g, b }
}

fn dbg_board(board: &[Vec<char>], iteration_i: Option<i32>) {
    // print!("\x1B[2J\x1B[H"); // Clear the screen

    let width = board[0].len();
    if iteration_i.is_none() {
        print!("   ");
    } else {
        print!("{:0>2}", iteration_i.unwrap())
    }

    for n in 0..width {
        if n > 9 {
            break;
        }
        print!(" {n}");
    }
    println!();

    println!("  ┌{}┐", "─".repeat(width * 2));
    for (i, row) in board.iter().enumerate() {
        if i <= 9 {
            print!("{} ", i);
        } else {
            print!("  ");
        };
        print!("│");
        for chr in row {
            print!(
                "{}",
                match chr {
                    '.' => format!(
                        "{}{}",
                        "▄".bright_black().on_white(),
                        "▄".white().on_bright_black()
                    )
                    .normal(),
                    '#' => "##".black().on_bright_yellow(),
                    c @ '0'..='9' | c @ 'a'..='z' | c @ 'A'..='Z' =>
                        "  ".black().on_color(char_to_color(c)),
                    _ => "??".normal(),
                }
            );
        }
        println!("│");
    }
    println!("  └{}┘", "─".repeat(width * 2));
}

fn find_antennae_coordinate_pairs(
    antennae_board: &[Vec<char>],
) -> Vec<((usize, usize), (usize, usize))> {
    let antennae_chars = ('0'..='9').chain('a'..='z').chain('A'..='Z');

    let mut pairs = Vec::new();

    for wanted_char in antennae_chars {
        for (x1, line) in antennae_board.iter().enumerate() {
            for (y1, &char1) in line.iter().enumerate() {
                if char1 != wanted_char {
                    continue;
                }

                for (x2, line) in antennae_board.iter().enumerate() {
                    if x1 == x2 {
                        continue;
                    }

                    for (y2, &char2) in line.iter().enumerate() {
                        if char2 != char1 {
                            continue;
                        }

                        if y1 == y2 {
                            continue;
                        }

                        // if pairs.is_empty() { // Only let the first pair through to antonode placing code, for testing only
                        pairs.push(((x1, y1), (x2, y2)));
                        // }
                    }
                }
            }
        }
    }

    pairs
}

fn mark_antinodes_on_empty_board(antennae_board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut board = antennae_board
        .iter()
        .map(|line| line.iter().map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // let mut board = antennae_board.clone();

    let width = board[0].len();
    let height = board.len();

    let antennae_coordinate_pairs = find_antennae_coordinate_pairs(&antennae_board);

    for ((x1, y1), (x2, y2)) in antennae_coordinate_pairs {
        let diff_vector = (
            ((x1 as isize) - (x2 as isize)),
            ((y1 as isize) - (y2 as isize)),
        );

        let p1 = (
            (x1 as isize) + (diff_vector.0),
            (y1 as isize) + (diff_vector.1),
        );
        let p2 = (
            (x2 as isize) + (diff_vector.0),
            (y2 as isize) + (diff_vector.1),
        );
        let p3 = (
            (x1 as isize) - (diff_vector.0),
            (y1 as isize) - (diff_vector.1),
        );
        let p4 = (
            (x2 as isize) - (diff_vector.0),
            (y2 as isize) - (diff_vector.1),
        );

        for p in [p1, p2, p3, p4] {
            if p == (x1 as isize, y1 as isize) {
                continue;
            }
            if p == (x2 as isize, y2 as isize) {
                continue;
            }
            if p.0 < 0 || p.1 < 0 {
                continue;
            }
            if p.0 >= height as isize || p.1 >= width as isize {
                continue;
            }

            board[p.0 as usize][p.1 as usize] = '#';
        }
    }

    board
}

fn main() {
    let input = input!().replace("\r\n", "\n");

    let antennae_board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    dbg_board(&antennae_board, None);

    let antinodes_board = mark_antinodes_on_empty_board(antennae_board);

    dbg_board(&antinodes_board, None);

    let antinodes_count = antinodes_board
        .iter()
        .map(|line| line.iter().filter(|&&ch| ch == '#').count())
        .sum::<usize>();

    println!("Total antinodes: {}", antinodes_count);
}
