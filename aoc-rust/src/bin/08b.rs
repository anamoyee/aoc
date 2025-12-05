#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

use aoc::char_to_color;

fn board_union<T: Clone>(
    is_empty_predicate: impl Fn(&T) -> bool,
    board1: &[Vec<T>],
    board2: &[Vec<T>],
) -> Vec<Vec<T>> {
    board1
        .iter()
        .zip(board2.iter())
        .map(|(row1, row2)| {
            row1.iter()
                .zip(row2.iter())
                .map(|(val1, val2)| {
                    if is_empty_predicate(val2) {
                        val1.clone()
                    } else {
                        val2.clone()
                    }
                })
                .collect()
        })
        .collect()
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

    let mut _n_found = 0;

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

                        _n_found += 1;

                        // if _n_found == 2 {
                        // Only let the nth pair through to antonode placing code, for testing only
                        pairs.push(((x1, y1), (x2, y2)));
                        // }
                    }
                }
            }
        }
    }

    pairs
}

/// On the given board, mark the given value at [x as usize][y as usize], return false, if the marking was not successful due to being out of bounds.
fn mark<T>(board: &mut [Vec<T>], value: T, pos: (isize, isize)) -> bool {
    let (x, y) = pos;

    if x < 0 || y < 0 {
        return false;
    }

    let x = x as usize;
    let y = y as usize;

    if let Some(row) = board.get_mut(x) {
        if let Some(cell) = row.get_mut(y) {
            *cell = value;
            return true;
        }
    }

    false
}

fn get_width_height<T>(board: &[Vec<T>]) -> (usize, usize) {
    (board[0].len(), board.len())
}

fn mark_antinodes_on_empty_board(antennae_board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut board = antennae_board
        .iter()
        .map(|line| line.iter().map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // let mut board = antennae_board.clone();

    let (width, height) = get_width_height(&board);

    let antennae_coordinate_pairs = find_antennae_coordinate_pairs(&antennae_board);

    for ((x1, y1), (x2, y2)) in antennae_coordinate_pairs {
        let diff_vector = (
            ((x1 as isize) - (x2 as isize)),
            ((y1 as isize) - (y2 as isize)),
        );

        let mut piss = vec![];

        for i in (-(usize::max(width, height) as isize))..=(usize::max(width, height) as isize) {
            let pee = (
                (x1 as isize) + (i * diff_vector.0),
                (y1 as isize) + (i * diff_vector.1),
            );

            piss.push(pee);
        }

        for &p in piss.iter() {
            // if p == (x1 as isize, y1 as isize) {
            //     continue;
            // }
            // if p == (x2 as isize, y2 as isize) {
            //     continue;
            // }

            // if p.0 < 0 || p.1 < 0 {
            //     continue;
            // }
            // if p.0 >= height as isize || p.1 >= width as isize {
            //     continue;
            // }

            // board[p.0 as usize][p.1 as usize] = '#';

            mark(&mut board, '#', p);
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

    let antinodes_board = mark_antinodes_on_empty_board(antennae_board.clone());

    dbg_board(&antinodes_board, None);

    let both_boards = board_union(|ch| *ch == '.', &antennae_board, &antinodes_board);

    dbg_board(&both_boards, None);

    let antinodes_count = antinodes_board
        .iter()
        .map(|line| line.iter().filter(|&&ch| ch == '#').count())
        .sum::<usize>();

    println!("Total antinodes: {}", antinodes_count);
}
