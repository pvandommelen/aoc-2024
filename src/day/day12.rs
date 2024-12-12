use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::Direction;
use crate::util::solver::solve_depth_first;
use rustc_hash::FxHashSet;

type PreparedInput = Grid<u8>;

fn prepare(input: &str) -> PreparedInput {
    Grid::from_rows(input.lines().map(|line| line.as_bytes().iter().cloned()))
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn solve_both(input: &PreparedInput) -> (u64, u64) {
    let mut visited = FxHashSet::with_capacity_and_hasher(input.size(), Default::default());

    let mut p1 = 0;
    let mut p2 = 0;
    input.iter().for_each(|(pos, t)| {
        if !visited.insert(pos) {
            return;
        }

        let mut area = 0;
        let mut perimeter = 0;
        let mut edges = FxHashSet::default();

        solve_depth_first(
            |stack, pos| {
                area += 1;
                DIRECTIONS.iter().for_each(|&direction| {
                    if let Some(pos) = pos.checked_moved(&input.dimensions, &direction) {
                        if input.get(&pos) == t {
                            if visited.insert(pos) {
                                stack.push(pos);
                            }
                            return;
                        }
                    }

                    perimeter += 1;
                    edges.insert((pos, direction));
                })
            },
            vec![pos],
        );

        let mut sides = 0;
        while let Some((pos, direction)) = edges
            .iter()
            .next()
            .cloned()
            .map(|edge| edges.take(&edge).unwrap())
        {
            sides += 1;
            match direction {
                Direction::Up | Direction::Down => {
                    for x in pos.1 + 1..input.dimensions.1 {
                        if !edges.remove(&((pos.0, x).into(), direction)) {
                            break;
                        }
                    }
                    for x in (0..pos.1).rev() {
                        if !edges.remove(&((pos.0, x).into(), direction)) {
                            break;
                        }
                    }
                }
                Direction::Right | Direction::Left => {
                    for y in pos.0 + 1..input.dimensions.0 {
                        if !edges.remove(&((y, pos.1).into(), direction)) {
                            break;
                        }
                    }
                    for y in (0..pos.0).rev() {
                        if !edges.remove(&((y, pos.1).into(), direction)) {
                            break;
                        }
                    }
                }
            }
        }

        p1 += area * perimeter;
        p2 += area * sides;
    });
    (p1, p2)
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    const FIRST_EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";
    const SECOND_EXAMPLE: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const THIRD_EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(FIRST_EXAMPLE).dimensions, (4, 4).into());
    }
    #[rstest]
    #[case(FIRST_EXAMPLE, 140)]
    #[case(SECOND_EXAMPLE, 772)]
    #[case(THIRD_EXAMPLE, 1930)]
    #[test]
    fn part1(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(solve_both(&prepare(input)).0, expected);
    }

    #[rstest]
    #[case(FIRST_EXAMPLE, 80)]
    #[case(SECOND_EXAMPLE, 436)]
    #[case(
        "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        236
    )]
    #[case(
        "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        368
    )]
    #[case(THIRD_EXAMPLE, 1206)]
    #[test]
    fn part2(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(solve_both(&prepare(input)).1, expected);
    }
}
