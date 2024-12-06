use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::{Position, PositionOffset, RotationalDirection};
use rustc_hash::FxHashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Obstruction,
    GuardUpwardFacing,
}

type PreparedInput = Grid<Tile>;

fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(input.lines().map(|l| {
        l.chars().map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Obstruction,
            '^' => Tile::GuardUpwardFacing,
            _ => panic!("Unexpected char {}", c),
        })
    }))
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum WalkResult {
    OutOfBounds,
    Loop,
}

fn walk(
    grid: &PreparedInput,
    pos: &Position,
    direction: &PositionOffset,
    obstruction_test_fn: impl Fn(&Position) -> bool,
    mut each_fn: impl FnMut(&Position, &PositionOffset),
) -> WalkResult {
    let mut visited = FxHashSet::default();
    let mut pos = *pos;
    let mut direction = *direction;

    loop {
        let previous_visit = visited.insert((pos, direction));
        if !previous_visit {
            return WalkResult::Loop;
        }
        each_fn(&pos, &direction);
        let Some(next_pos) = pos.checked_moved(&grid.dimensions, &direction) else {
            return WalkResult::OutOfBounds;
        };
        let next = grid.get(&next_pos);
        if matches!(next, Tile::Obstruction) || obstruction_test_fn(&next_pos) {
            direction = direction.rotated(&RotationalDirection::Clockwise);
        } else {
            pos = next_pos;
        }
    }
}

fn solve_both(input: &PreparedInput) -> (usize, usize) {
    let mut visited = FxHashSet::default();

    let pos = input
        .positions_where(|tile| *tile == Tile::GuardUpwardFacing)
        .next()
        .unwrap();

    let mut extra_obstructions = 0;

    let result = walk(
        input,
        &pos,
        &PositionOffset::up(),
        |_| false,
        |pos, direction| {
            visited.insert(*pos);

            let Some(obstruction_pos) = pos.checked_moved(&input.dimensions, direction) else {
                return;
            };
            let obstruction_tile = input.get(&obstruction_pos);

            if matches!(obstruction_tile, Tile::Empty) && !visited.contains(&obstruction_pos) {
                let rotated_direction = direction.rotated(&RotationalDirection::Clockwise);
                if matches!(
                    walk(
                        input,
                        pos,
                        &rotated_direction,
                        |pos| obstruction_pos == *pos,
                        |_, _| {}
                    ),
                    WalkResult::Loop
                ) {
                    extra_obstructions += 1;
                }
            }
        },
    );
    assert_eq!(result, WalkResult::OutOfBounds);

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
