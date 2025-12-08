use aoc::prelude::*;

#[derive(PartialEq, Debug)]
enum GrCell {
    Empty,
    PaperRoll,
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

fn sum_non_empty_neighbours(grid: &Grid<GrCell>, center_pos: (usize, usize)) -> u32 {
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
        Some(&GrCell::Empty) => 0,
        Some(&_) => 1,
    })
    .sum()
}

fn process_once(grid: &mut Grid<GrCell>) -> usize {
    let accessible_cell_indices = grid
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, ct)| *ct == GrCell::PaperRoll)
                .map(|(y, _)| (x, y, sum_non_empty_neighbours(&grid, (x, y))))
                .filter(|&(_, _, soom)| soom < 4)
                .map(|(x, y, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    for &(x, y) in &accessible_cell_indices {
        grid[x][y] = GrCell::Empty;
    }

    return accessible_cell_indices.len();
}

fn main() {
    let mut grid: Grid<GrCell> = input!()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => GrCell::Empty,
                    '@' => GrCell::PaperRoll,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut total = 0;

    loop {
        let result = process_once(&mut grid);

        if result == 0 {
            break;
        }

        total += result;
    }

    ans!(total);
}
