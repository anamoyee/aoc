#[allow(unused)]
use aoc::{input, input_test};
use colored::*;
use std::io;

fn dbg_board(board: &[Vec<char>], iteration_i: Option<i32>) {
    _ = "┌─┬┐";
    _ = "│ ││";
    _ = "├─┼┤";
    _ = "└─┴┘";

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
                    '#' => "  ".black().on_bright_yellow(),
                    'X' => "  ".black().on_red(),
                    '^' => "▄▄".black().on_blue(),
                    'v' => "▄▄".black().blue(),
                    '<' => format!("{}{}", " ".blue(), " ".on_blue()).normal(),
                    '>' => format!("{}{}", " ".on_blue(), " ".blue()).normal(),
                    invalid => panic!("Invalid character: {invalid:#?}"),
                }
            );
        }
        println!("│");
    }
    println!("  └{}┘", "─".repeat(width * 2));
}

fn get_player_pos_in_board(board: &[Vec<char>]) -> (usize, usize) {
    for (i, line) in board.iter().enumerate() {
        for (j, chr) in line.iter().enumerate() {
            match chr {
                '^' | 'v' | '<' | '>' => {
                    return (i, j);
                }
                _ => {
                    continue;
                }
            }
        }
    }

    panic!("Could not find the player in this board!");
}

enum PlayerRot {
    Up,
    Down,
    Left,
    Right,
}

impl PlayerRot {
    fn next(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn as_movement_vector(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

fn get_player_rot_in_board(board: &[Vec<char>], pos: &(usize, usize)) -> PlayerRot {
    match board[pos.0][pos.1] {
        '^' => PlayerRot::Up,
        'v' => PlayerRot::Down,
        '<' => PlayerRot::Left,
        '>' => PlayerRot::Right,
        invalid => panic!("Invalid player char {invalid:#?}"),
    }
}

/// return true means "further processing is required"
fn process_frame(board: &mut [Vec<char>]) -> bool {
    let height = board.len();
    let width = board[0].len();

    let player_pos = get_player_pos_in_board(board);
    let player_rot = get_player_rot_in_board(board, &player_pos);
    let player_rot_movement_vector = player_rot.as_movement_vector();

    let next_player_pos = (
        player_pos.0 as isize + player_rot_movement_vector.0,
        player_pos.1 as isize + player_rot_movement_vector.1,
    );

    if next_player_pos.0 < 0
        || next_player_pos.1 < 0
        || next_player_pos.0 >= height as isize
        || next_player_pos.1 >= width as isize
    {
        board[player_pos.0][player_pos.1] = 'X';
        return false;
    }

    let next_player_pos = (next_player_pos.0 as usize, next_player_pos.1 as usize);

    match board[next_player_pos.0][next_player_pos.1] {
        '.' | 'X' => {
            board[player_pos.0][player_pos.1] = 'X';
            board[next_player_pos.0][next_player_pos.1] = player_rot.as_char();
        }
        '#' => {
            board[player_pos.0][player_pos.1] = player_rot.next().as_char();
        }
        invalid => panic!("Invalid character {invalid:#?}"),
    }

    true
}

fn get_total_xs_on_board(board: &[Vec<char>]) -> usize {
    board
        .iter()
        .map(|line| line.iter().filter(|&&x| x == 'X').count())
        .sum::<usize>()
}

fn main() {
    let input = input!().replace("\r\n", "\n");

    let mut board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut i = 0;
    loop {
        let should_print = i % 1000 == 0;

        if should_print {
            dbg_board(&board, Some(i));
        }

        if !process_frame(&mut board) {
            break;
        }

        if should_print {
            _ = io::stdin().read_line(&mut String::new());
        }

        i += 1;
    }
    dbg_board(&board, Some(i));

    println!(
        "Finished after {} iterations. Answer: {}",
        format!("{}", i).blue(),
        format!("{}", get_total_xs_on_board(&board)).green().bold()
    )
}
