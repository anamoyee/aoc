#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

fn _dbg_board(board: &[Vec<char>], iteration_i: Option<i32>) {
    _ = "┌─┬┐";
    _ = "│ ││";
    _ = "├─┼┤";
    _ = "└─┴┘";

    print!("\x1B[2J\x1B[H"); // Clear the screen

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
                    'O' => "  ".black().on_purple(),
                    'X' => "  ".black().on_red(),
                    '^' => "▄▄".black().on_blue(),
                    'v' => "▄▄".black().blue(),
                    '<' => format!("{}{}", " ".on_blue(), " ".blue()).normal(),
                    '>' => format!("{}{}", " ".blue(), " ".on_blue()).normal(),
                    _ => "??".normal(),
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

#[derive(PartialEq, std::fmt::Debug)]
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
fn process_frame(
    board: &mut [Vec<char>],
    known_player_pos: Option<(usize, usize)>,
) -> (bool, (usize, usize)) {
    let height = board.len();
    let width = board[0].len();

    let player_pos = known_player_pos.unwrap_or_else(|| get_player_pos_in_board(board));
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
        return (false, (0, 0));
    }

    let next_player_pos = (next_player_pos.0 as usize, next_player_pos.1 as usize);

    match board[next_player_pos.0][next_player_pos.1] {
        '.' | 'X' => {
            board[player_pos.0][player_pos.1] = 'X';
            board[next_player_pos.0][next_player_pos.1] = player_rot.as_char();
            (true, next_player_pos)
        }
        '#' | 'O' => {
            board[player_pos.0][player_pos.1] = player_rot.next().as_char();
            (true, player_pos)
        }
        invalid => panic!("Invalid character {invalid:#?}"),
    }
}

fn is_board_finite(board: &mut [Vec<char>]) -> bool {
    let mut board_ids = Vec::new();

    let mut i = 0;

    let mut last_player_pos: Option<(usize, usize)> = None;

    loop {
        i += 1;

        let this_board_player_pos = get_player_pos_in_board(board);
        let this_board_id = (
            this_board_player_pos.0,
            this_board_player_pos.1,
            get_player_rot_in_board(board, &this_board_player_pos),
        );

        if board_ids.contains(&this_board_id) {
            return false;
        }

        let (continue_polling, player_pos) = process_frame(board, last_player_pos);
        last_player_pos = Some(player_pos);

        if !continue_polling {
            return true;
        }

        board_ids.push(this_board_id);
    }
}

fn main() {
    let input = input!().replace("\r\n", "\n");

    let original_board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut infinite_number = 0u32;
    let (h, w) = (original_board.len(), original_board[0].len());

    for (i, line) in original_board.iter().enumerate() {
        for (j, &chr) in line.iter().enumerate() {
            println!("{}/{}", i * h + j, h * w);

            if chr != '.' {
                continue;
            }

            let mut this_board = original_board.clone();

            this_board[i][j] = 'O';

            let is_finite = is_board_finite(&mut this_board);
            // dbg_board(&this_board, None);

            if !is_finite {
                infinite_number += 1;
            }

            // std::io::stdin().read_line(&mut String::new()).unwrap();
        }
    }

    println!("Infinite count: {infinite_number}")
}
