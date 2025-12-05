use colored::*;
use std::fs;
use std::path::Path;

pub fn _read_input(day: u8, suffix: &str) -> String {
    let crate_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(crate_root)
        .join("..")
        .join("inputs")
        .join(format!("{day}{suffix}.txt"));

    fs::read_to_string(&input_path).unwrap_or_else(|err| {
        panic!(
            "{}",
            format!("Failed to read input file at {:?}: {}", input_path, err)
                .bright_red()
                .bold()
        );
    })
}

#[macro_export]
macro_rules! input {
    () => {
        input!("")
    };
    ($suffix:expr) => {
        aoc::_read_input(
            file!()
                .replace("\\", "/")
                .split('/')
                .last()
                .unwrap()
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u8>()
                .expect(
                    "Failed to extract day from file name"
                        .bright_red()
                        .bold()
                        .as_ref(),
                ),
            &format!("{}", $suffix),
        )
    };
}

#[macro_export]
macro_rules! input_test {
    () => {
        input!("_test")
    };
}

pub fn char_to_color(c: &char) -> colored::Color {
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
