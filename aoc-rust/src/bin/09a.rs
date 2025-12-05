#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

#[allow(unused)]
fn dbg(v: &Vec<Option<usize>>) {
    for &opt in v {
        if let Some(n) = opt {
            if n > 9 {
                print!("({n})")
            } else {
                print!("{n}")
            }
        } else {
            print!(".")
        }
    }
    println!();
}

fn main() {
    let mut input = input!()
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, ch)| {
            let n = str::parse::<u32>(ch.to_string().as_str()).unwrap();

            if i % 2 == 0 {
                // is file
                vec![Some(i / 2); n as usize]
            } else {
                // is empty
                vec![None; n as usize]
            }
        })
        .collect::<Vec<_>>();

    let mut l = 0_usize;
    let mut r = input.len() - 1;

    'outer: loop {
        while input[r].is_none() {
            if r == 0 {
                break 'outer;
            }

            r -= 1;
        }

        while input[l].is_some() {
            l += 1;

            if l == input.len() {
                break 'outer;
            }
        }

        if l >= r {
            break;
        }

        // dbg(&input);
        input.swap(l, r);
    }

    println!();
    // dbg(&input);

    let checksum = input
        .iter()
        .enumerate()
        .map(|(i, &n)| if let Some(n) = n { i * n } else { 0 })
        .sum::<usize>();

    println!("Checksum: {}", checksum);
}
