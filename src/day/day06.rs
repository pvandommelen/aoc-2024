use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::{Position, PositionOffset, RotationalDirection};
use rustc_hash::FxHashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Cell {
    Empty,
    Obstruction,
    GuardUpwardFacing,
}

type PreparedInput = Grid<Cell>;

fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(input.lines().map(|l| {
        l.chars().map(|c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Obstruction,
            '^' => Cell::GuardUpwardFacing,
            _ => panic!("Unexpected char {}", c),
        })
    }))
}

enum WalkResult {
    OutOfBounds,
    Loop,
}

fn walk(
    grid: &PreparedInput,
    additional_obstruction: &Position,
    pos: &Position,
    direction: &PositionOffset,
) -> WalkResult {
    let mut visited = FxHashSet::default();
    let mut pos = *pos;
    let mut direction = *direction;

    loop {
        let previous_visit = visited.insert((pos, direction));
        if !previous_visit {
            return WalkResult::Loop;
        }
        let Some(next_pos) = pos.checked_moved(&grid.dimensions, &direction) else {
            return WalkResult::OutOfBounds;
        };
        let next = grid.get(&next_pos);
        if matches!(next, Cell::Obstruction) || next_pos == *additional_obstruction {
            direction = direction.rotated(&RotationalDirection::Clockwise);
        } else {
            pos = next_pos;
        }
    }
}

fn solve_both(input: &PreparedInput) -> (usize, usize) {
    let mut visited = FxHashSet::default();

    let mut pos = input
        .positions_where(|cell| *cell == Cell::GuardUpwardFacing)
        .next()
        .unwrap();
    let mut direction = PositionOffset::up();

    let mut extra_obstructions = 0;

    loop {
        visited.insert(pos);
        let Some(next_pos) = pos.checked_moved(&input.dimensions, &direction) else {
            break;
        };
        let next = input.get(&next_pos);
        if matches!(next, Cell::Obstruction) {
            direction = direction.rotated(&RotationalDirection::Clockwise);
        } else {
            if matches!(next, Cell::Empty) && !visited.contains(&next_pos) {
                let rotated_direction = direction.rotated(&RotationalDirection::Clockwise);
                if matches!(
                    walk(input, &next_pos, &pos, &rotated_direction),
                    WalkResult::Loop
                ) {
                    extra_obstructions += 1;
                }
            }
            pos = next_pos;
        }
    }

    (visited.len(), extra_obstructions)
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).dimensions, (10, 10).into());
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).0, 41);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).1, 6);
    }

    #[test]
    fn part2_cases() {
        assert_eq!(solve_both(&prepare(".#.\n#^#\n...")).1, 1);
        assert_eq!(solve_both(&prepare("...\n#^#\n.#.")).1, 1);
        assert_eq!(solve_both(&prepare("...\n..#\n#^.\n.#.")).1, 1);
    }
}
