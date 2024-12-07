#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

fn get_board_dimensions(board: &[&str]) -> (usize, usize) {
    (board.len(), board[0].len())
}

fn find_word_at(word: &str, pos: (usize, usize), dir: (isize, isize), board: &[&str]) -> bool {
    if word.is_empty() {
        return true;
    }

    if board[pos.0].chars().nth(pos.1).unwrap() != word.chars().next().unwrap() {
        return false;
    }

    let dims = get_board_dimensions(board);
    let dims = (dims.0 as isize, dims.1 as isize);

    let pos_finish = (
        pos.0 as isize + dir.0 * (word.len() - 1) as isize,
        pos.1 as isize + dir.1 * (word.len() - 1) as isize,
    );

    if pos == (9, 3) && dir == (-1, -1) {
        dbg!(pos_finish);
    }

    if pos_finish.0 >= dims.0 || pos_finish.1 >= dims.1 || pos_finish.0 < 0 || pos_finish.1 < 0 {
        return false;
    };

    find_word_at(
        &word[1..],
        (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        ),
        dir,
        board,
    )
}

fn main() {
    let input = input!();

    let out = input.lines().collect::<Vec<_>>();

    assert!(out.iter().all(|line| line.len() == out[0].len())); // Same strlen for every row check

    let mut bóls = Vec::new();

    let dirs = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for (i, &line) in out.iter().enumerate() {
        for (j, _) in line.chars().enumerate() {
            let mut new_vec = Vec::new();

            for dir in dirs {
                new_vec.push(find_word_at("XMAS", (i, j), dir, &out));
            }

            if new_vec.iter().filter(|&&x| x).count() > 0 {
                println!("\n{i},{j}");
                println!("---------------");
                for (i, &dir) in dirs.iter().enumerate() {
                    if new_vec[i] {
                        println!("{:>2},{:>2}", dir.0, dir.1)
                    }
                }
                println!("---------------");
            }

            bóls.append(&mut new_vec);
        }
    }

    let bóls = bóls.iter().filter(|&&x| x).count();

    println!("{}", format!("{:#?}", bóls).bright_blue().bold());
}
