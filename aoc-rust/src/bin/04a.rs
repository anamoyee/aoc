use aoc::prelude::*;

#[derive(PartialEq, Debug)]
enum CellType {
    Empty,
    Roll,
}

type Grid<T> = Vec<Vec<T>>;

trait SafeGet2D<T> {
    fn safe_get2d(&self, x: isize, y: isize) -> Option<&T>;
}

impl<T> SafeGet2D<T> for Grid<T> {
    fn safe_get2d(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        self.get(x).and_then(|row| row.get(y))
    }
}

fn sum_non_empty_neighbours(grid: &Grid<CellType>, center_pos: (usize, usize)) -> u32 {
    [
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
    ]
    .iter()
    .map(|&(off_0, off_1)| {
        grid.safe_get2d(center_pos.0 as isize + off_0, center_pos.1 as isize + off_1)
    })
    .map(|opt| match opt {
        None => 0,
        Some(&CellType::Empty) => 0,
        Some(&_) => 1,
    })
    .sum()
}

fn main() {
    let grid: Grid<CellType> = input!()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => CellType::Empty,
                    '@' => CellType::Roll,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let answer = grid
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, ct)| *ct == CellType::Roll)
                .map(|(y, _)| sum_non_empty_neighbours(&grid, (x, y)))
                .filter(|&soom| soom < 4)
                .count()
        })
        .sum::<usize>();

    ans!(answer);
}
