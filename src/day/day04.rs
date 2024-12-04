use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;

type PreparedInput = Grid<u8>;

fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(input.lines().map(|r| r.as_bytes().iter().copied()))
}

#[inline]
fn test_slice(slice: &[u8]) -> bool {
    slice == [b'X', b'M', b'A', b'S'] || slice == [b'S', b'A', b'M', b'X']
}

fn solve_part1(input: &PreparedInput) -> usize {
    let mut count = input.rows().fold(0, |count, row| {
        row.windows(4).fold(count, |mut count, slice| {
            count += if test_slice(slice) { 1 } else { 0 };
            count
        })
    });

    fn handle_slice_iterator(
        buf: &mut Vec<u8>,
        slices: impl Iterator<Item = impl Iterator<Item = u8>>,
    ) -> usize {
        slices.fold(0, |count, slice| {
            buf.clear();
            buf.extend(slice);
            buf.windows(4).fold(count, |mut count, slice| {
                count += if test_slice(slice) { 1 } else { 0 };
                count
            })
        })
    }

    let mut buf = Vec::with_capacity(input.dimensions.0);
    count += handle_slice_iterator(&mut buf, input.columns());
    count += handle_slice_iterator(&mut buf, input.diagonals_lower());
    count += handle_slice_iterator(&mut buf, input.diagonals_upper().skip(1));
    count += handle_slice_iterator(&mut buf, input.anti_diagonals_upper());
    count += handle_slice_iterator(&mut buf, input.anti_diagonals_lower().skip(1));
    count
}

fn solve_part2(input: &PreparedInput) -> usize {
    input
        .iter_windows3_where(|c| *c == b'A')
        .fold(0, |mut count, window| {
            if (*window.top_left() == b'M' && *window.bottom_right() == b'S'
                || *window.top_left() == b'S' && *window.bottom_right() == b'M')
                && (*window.top_right() == b'M' && *window.bottom_left() == b'S'
                    || *window.top_right() == b'S' && *window.bottom_left() == b'M')
            {
                count += 1
            }
            count
        })
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
