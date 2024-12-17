use crate::solution::SolutionTuple;
use crate::util::measure::MeasureContext;
use itertools::Itertools;

type PreparedInput = ([u64; 3], Vec<u8>);

fn prepare(input: &str) -> PreparedInput {
    let mut sections = input.split("\n\n");
    let register_section = sections.next().unwrap();
    let program_section = sections.next().unwrap();
    assert_eq!(sections.next(), None);

    let registers = register_section
        .lines()
        .map(|line| line.split(": ").last().unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let program = program_section
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    (registers.try_into().unwrap(), program)
}

fn run_program(mut registers: [u64; 3], program: &[u8]) -> ([u64; 3], Vec<u8>) {
    let mut out = vec![];

    let mut i = 0;
    while i + 1 < program.len() {
        let instruction = program[i];
        let operand = program[i + 1];
        assert_eq!(i % 2, 0);

        let literal_operand_value = operand as u64;
        let combo_operand_value = || match operand {
            0..4 => operand as u64,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            7 => panic!("is reserved and will not appear in valid programs"),
            _ => panic!(),
        };

        match instruction {
            0 => {
                registers[0] >>= combo_operand_value();
            }
            1 => {
                registers[1] ^= literal_operand_value;
            }
            2 => {
                registers[1] = combo_operand_value() % 8;
            }
            3 => {
                if registers[0] != 0 {
                    i = literal_operand_value as usize;
                    continue;
                }
            }
            4 => {
                registers[1] ^= registers[2];
            }
            5 => {
                out.push((combo_operand_value() % 8) as u8);
            }
            6 => {
                registers[1] = registers[0] >> combo_operand_value();
            }
            7 => {
                registers[2] = registers[0] >> combo_operand_value();
            }
            _ => panic!(),
        }
        i += 2;
    }
    (registers, out)
}

fn solve_part1(input: &PreparedInput) -> String {
    let (registers, program) = input;

    let (_, out) = run_program(*registers, program);

    out.into_iter().map(|num| num.to_string()).join(",")
}

fn find<const EXAMPLE: bool>(expected: u64, carry: u64) -> Vec<u64> {
    let mut result = vec![];
    for a in 0..8u64 {
        let value = if EXAMPLE {
            a % 8
        } else {
            let mut b = a % 8;
            b ^= 1;

            let c = (a + carry) / (1 << b);
            b ^= 4;
            b ^= c;

            b % 8
        };
        if value == expected {
            result.push(a);
        }
    }
    result
}

fn solve_part2<const EXAMPLE: bool>(input: &PreparedInput) -> u64 {
    let (_, program) = input;

    // Test the fairly specific conditions for which this solution works. (Ignoring the hardcoded instructions for now)
    assert_eq!(program[program.len() - 4], 5);
    assert_eq!(program[program.len() - 2], 3);
    assert_eq!(program[program.len() - 1], 0);
    assert!(
        !program[0..program.len() - 4]
            .iter()
            .step_by(2)
            .any(|instr| *instr == 3 || *instr == 5)
    );

    fn find_recursive<const EXAMPLE: bool>(
        mut expected: impl Iterator<Item = u8> + Clone,
        carry: u64,
    ) -> Vec<u64> {
        let Some(exp) = expected.next() else {
            return vec![if EXAMPLE { carry } else { carry >> 3 }];
        };

        let nums = find::<EXAMPLE>(exp as u64, carry);

        nums.into_iter()
            .flat_map(|num| find_recursive::<EXAMPLE>(expected.clone(), (carry + num) << 3))
            .collect()
    }

    let results = find_recursive::<EXAMPLE>(program.iter().rev().cloned(), 0);
    for result in &results {
        assert_eq!(&run_program([*result, 0, 0], program).1, program);
    }

    assert!(!results.is_empty());
    results[0]
}

pub fn solve(ctx: &mut MeasureContext, input: &str) -> SolutionTuple {
    let input = ctx.measure("prepare", || prepare(input));

    (
        ctx.measure("part1", || solve_part1(&input)),
        ctx.measure("part2", || solve_part2::<false>(&input)),
    )
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    #[test]
    fn prepare_example() {
        assert_eq!(prepare(PART1_EXAMPLE).1.len(), 6);
    }
    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&prepare(PART1_EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part1_small_examples() {
        assert_eq!(run_program([0, 0, 9], &[2, 6]).0[1], 1);
        assert_eq!(run_program([10, 0, 0], &[5, 0, 5, 1, 5, 4]).1, vec![
            0, 1, 2
        ]);
        assert_eq!(
            run_program([2024, 0, 0], &[0, 1, 5, 4, 3, 0]),
            ([0, 0, 0], vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0])
        );
        assert_eq!(run_program([0, 29, 0], &[1, 7]).0, [0, 26, 0]);
        assert_eq!(run_program([0, 2024, 43690], &[4, 0]).0, [0, 44354, 43690]);
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2::<true>(&prepare(
                "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            )),
            117440
        );
    }
}
