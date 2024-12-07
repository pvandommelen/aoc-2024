use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use arrayvec::ArrayVec;
use winnow::Parser;
use winnow::ascii::dec_uint;

type Equation = (u64, ArrayVec<u16, 12>);
type PreparedInput = Vec<Equation>;

fn line(input: &str) -> Equation {
    let mut input = input.as_bytes();
    let a = dec_uint::<_, _, ()>(&mut input).unwrap();
    let v = input[2..]
        .split(|c| *c == b' ')
        .map(|x| dec_uint::<_, u16, ()>.parse(x).unwrap())
        .collect();
    (a, v)
}

fn prepare(input: &str) -> PreparedInput {
    input.lines().map(line).collect()
}

fn test<const CONCAT_ENABLED: bool>(numbers: &[u16], i: usize, expected_result: u64) -> bool {
    let last_num = numbers[i] as u64;
    if i == 0 {
        return last_num == expected_result;
    }
    if expected_result % last_num == 0
        && test::<CONCAT_ENABLED>(numbers, i - 1, expected_result / last_num)
    {
        return true;
    }
    if expected_result >= last_num
        && test::<CONCAT_ENABLED>(numbers, i - 1, expected_result - last_num)
    {
        return true;
    }
    if CONCAT_ENABLED {
        let factor = 10u64.pow(last_num.ilog10() + 1);
        if expected_result % factor == last_num
            && test::<CONCAT_ENABLED>(numbers, i - 1, expected_result / factor)
        {
            return true;
        }
    }
    false
}

fn solve_both(input: &PreparedInput) -> (u64, u64) {
    input
        .iter()
        .filter_map(|eq| {
            let p1 = test::<false>(&eq.1, eq.1.len() - 1, eq.0);
            if p1 {
                return Some((eq.0, eq.0));
            }
            let p2 = test::<true>(&eq.1, eq.1.len() - 1, eq.0);
            if p2 {
                return Some((0, eq.0));
            }
            None
        })
        .reduce(|(p1, p2), (p1_entry, p2_entry)| (p1 + p1_entry, p2 + p2_entry))
        .unwrap()
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));
    ctx.measure("both", || solve_both(&input)).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn example_prepare() {
        assert_eq!(prepare(EXAMPLE_INPUT).len(), 9);
    }
    #[test]
    fn example_part1() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).0, 3749);
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_both(&prepare(EXAMPLE_INPUT)).1, 11387);
    }
}
