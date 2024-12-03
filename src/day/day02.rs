use crate::solution::Solution;
use crate::util::measure::MeasureContext;
use arrayvec::ArrayVec;

type PreparedInput = Vec<ArrayVec<u8, 8>>;

fn line(input: &str) -> ArrayVec<u8, 8> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(line).collect()
}

fn find_nonincreasing(mut cur: u8, remaining: &[u8]) -> Option<usize> {
    for (i, &num) in remaining.iter().enumerate() {
        if num <= cur || num - cur > 3 {
            return Some(i);
        }
        cur = remaining[i];
    }
    None
}
fn find_nondecreasing(mut cur: u8, remaining: &[u8]) -> Option<usize> {
    for (i, &num) in remaining.iter().enumerate() {
        if num >= cur || cur - num > 3 {
            return Some(i);
        }
        cur = remaining[i];
    }
    None
}

fn solve_part1(input: &PreparedInput) -> usize {
    input
        .iter()
        .filter(|report| {
            find_nonincreasing(report[0], &report[1..]).is_none()
                || find_nondecreasing(report[0], &report[1..]).is_none()
        })
        .count()
}

fn solve_part2(input: &PreparedInput) -> usize {
    input
        .iter()
        .filter(|&report| {
            let Some(nonincreasing) = find_nonincreasing(report[0], &report[1..report.len() - 1])
            else {
                return true;
            };
            let Some(nondecreasing) = find_nondecreasing(report[0], &report[1..report.len() - 1])
            else {
                return true;
            };
            if find_nonincreasing(report[nonincreasing], &report[nonincreasing + 2..]).is_none() {
                return true;
            } else if nonincreasing > 0 {
                if find_nonincreasing(report[nonincreasing - 1], &report[nonincreasing + 1..])
                    .is_none()
                {
                    return true;
                }
            } else if find_nonincreasing(report[1], &report[2..]).is_none() {
                return true;
            }
            if find_nondecreasing(report[nondecreasing], &report[nondecreasing + 2..]).is_none() {
                return true;
            } else if nondecreasing > 0 {
                if find_nondecreasing(report[nondecreasing - 1], &report[nondecreasing + 1..])
                    .is_none()
                {
                    return true;
                }
            } else if find_nondecreasing(report[1], &report[2..]).is_none() {
                return true;
            }
            false
        })
        .count()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> (Solution, Solution) {
    let input = ctx.measure("prepare", || prepare(input));
    let r = (
        ctx.measure("part1", || solve_part1(&input)).into(),
        ctx.measure("part2", || solve_part2(&input)).into(),
    );
    ctx.measure("drop", move || drop(input));
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 6);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&prepare(EXAMPLE_INPUT)), 2);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&prepare(EXAMPLE_INPUT)), 4);
    }
}
