use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;

type PreparedInput = Grid<u8>;

fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(input.lines().map(|r| r.as_bytes().iter().copied()))
}

fn solve_part1(input: &PreparedInput) -> usize {
    let mut count = 0;
    input.positions_where(|c| *c == b'X').for_each(|pos| {
        for dx in -1isize..2 {
            for dy in -1isize..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let offset = (dx, dy).into();
                let mut values = pos
                    .positions(&input.dimensions, &offset)
                    .map(|p| input.get(&p));

                if matches!(values.next(), Some(b'M'))
                    && matches!(values.next(), Some(b'A'))
                    && matches!(values.next(), Some(b'S'))
                {
                    count += 1;
                }
            }
        }
    });
    count
}

fn solve_part2(input: &PreparedInput) -> usize {
    let mut count = 0;
    input.iter_windows3().for_each(|window| {
        if *window.center() != b'A' {
            return;
        }

        if (*window.top_left() == b'M' && *window.bottom_right() == b'S'
            || *window.top_left() == b'S' && *window.bottom_right() == b'M')
            && (*window.top_right() == b'M' && *window.bottom_left() == b'S'
                || *window.top_right() == b'S' && *window.bottom_left() == b'M')
        {
            count += 1;
        }
    });
    count
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || solve_part1(&input)),
        ctx.measure("part2", || solve_part2(&input)),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).dimensions, (10, 10).into());
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 18);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 9);
    }
}
