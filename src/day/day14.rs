use crate::solution::SolutionTuple;
use crate::util::grid::Grid;
use crate::util::measure::MeasureContext;
use crate::util::position::{Dimensions, Position, PositionOffset};
use winnow::ascii::{dec_int, dec_uint};
use winnow::combinator::{preceded, separated_pair};
use winnow::{PResult, Parser};

#[derive(Debug, Copy, Clone)]
struct Robot {
    pos: Position,
    vel: PositionOffset,
}
type PreparedInput = Vec<Robot>;

fn line(input: &mut &str) -> PResult<Robot> {
    separated_pair(
        preceded("p=", separated_pair(dec_uint, ",", dec_uint)).map(|(x, y)| Position(y, x)),
        " ",
        preceded("v=", separated_pair(dec_int, ",", dec_int)).map(|(x, y)| PositionOffset(y, x)),
    )
    .map(|(pos, vel)| Robot { pos, vel })
    .parse_next(input)
}

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(|l| line.parse(l).unwrap()).collect()
}

fn solve_part1(input: &PreparedInput, dimensions: &Dimensions) -> usize {
    let halfway_y = dimensions.0 / 2;
    let halfway_x = dimensions.1 / 2;

    let mut quadrants = [0, 0, 0, 0];

    input.iter().for_each(|robot| {
        let pos = robot.pos.wrapping_offset(dimensions, &(robot.vel * 100));
        if pos.0 < halfway_y && pos.1 < halfway_x {
            quadrants[0] += 1;
        } else if pos.0 < halfway_y && pos.1 > halfway_x {
            quadrants[1] += 1;
        } else if pos.0 > halfway_y && pos.1 < halfway_x {
            quadrants[2] += 1;
        } else if pos.0 > halfway_y && pos.1 > halfway_x {
            quadrants[3] += 1;
        }
    });
    quadrants.iter().product()
}

fn solve_part2(input: &PreparedInput, dimensions: &Dimensions) -> usize {
    for i in 0usize.. {
        let grid = Grid::<bool>::from_positions(
            *dimensions,
            input.iter().map(|robot| {
                robot
                    .pos
                    .wrapping_offset(dimensions, &(robot.vel * i as isize))
            }),
        );

        if grid.count() != input.len() {
            continue;
        }

        // println!("{}", grid);

        return i;
    }
    panic!();
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let dimensions = Dimensions(103, 101);
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || solve_part1(&input, &dimensions)),
        ctx.measure("part2", || solve_part2(&input, &dimensions)),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 12);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT), &Dimensions(7, 11)), 12);
    }
}
