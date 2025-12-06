use aoc::prelude::*;

use bare_metal_modulo::{MNum, ModNumC};

fn main() {
    let moves: Vec<(char, u32)> = input!()
        .lines()
        .map(|s| {
            let mut chars = s.chars();

            (
                chars.next().unwrap(),
                chars.collect::<String>().parse::<u32>().unwrap(),
            )
        })
        .collect();

    let mut dial: ModNumC<u32, 100> = ModNumC::new(50);
    let mut zeroes: u32 = 0;

    for (dir, amt) in moves {
        if dir == 'R' {
            dial += amt;
        } else {
            dial -= amt;
        }

        if dial.a() == 0 {
            zeroes += 1;
        }
    }

    ans!(zeroes);
}
