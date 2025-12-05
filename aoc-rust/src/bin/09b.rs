#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

#[allow(unused)]
fn dbg(v: &Vec<(Option<usize>, usize)>) {
    for &(opt, len) in v {
        if len == 0 {
            print!("!")
        }

        for i in 0..len {
            if let Some(n) = opt {
                if n > 9 {
                    print!("({n})");
                } else {
                    print!("{n}");
                }
            } else {
                print!(".");
            }
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let mut input = input!("_test")
        .trim()
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            let n = str::parse::<u32>(ch.to_string().as_str()).unwrap();

            if i % 2 == 0 {
                // is file
                (Some(i / 2), n as usize)
            } else {
                // is empty
                (None, n as usize)
            }
        })
        .collect::<Vec<_>>();

    dbg(&input);
    println!();
    println!();
    println!();

    let mut l;
    let mut r = input.len() - 1;

    'outest: loop {
        let mut changes = 0_u32;

        l = 0_usize;

        'outer: loop {
            while input[r].0.is_none() {
                if r == 0 {
                    break 'outer;
                }

                r -= 1;
            }

            // r is pointing to a non-empty

            while input[l].0.is_some() || input[l].1 < input[r].1 {
                l += 1;

                if l > r {
                    if r == 0 {
                        break 'outest;
                    }
                    r -= 1;

                    break 'outer;
                }
            }

            // l is pointing to an empty which can fit item at r
            // l < r

            input[l].1 -= input[r].1;
            if input[l].1 == 0 {
                input.remove(l);
            }

            let removed = input.remove(r);
            r -= 1;
            input.push((None, input[r].1));

            input.insert(l, removed);
            dbg(&input);
            println!("l={l},r={r}");

            changes += 1;
        }

        if changes == 0 {
            break;
        }
    }

    println!();
    println!();
    println!();
    println!("END: l={l},r={r}");
    dbg(&input);

    // let checksum = input
    //     .iter()
    //     .enumerate()
    //     .map(|(i, &n)| if let Some(n) = n { i * n } else { 0 })
    //     .sum::<usize>();

    // println!("Checksum: {}", checksum);
}
