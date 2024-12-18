use aoc_2024::day::*;
use aoc_2024::solution::SolutionTuple;
use aoc_2024::util::measure::MeasureContext;
use clap::Parser;
use std::hint::black_box;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day
    day: Option<usize>,
    #[arg(short, long, default_value = "1")]
    repeat: u32,
    #[arg(short, long, default_value = "0")]
    warmup: u32,
}

fn read_input(day: usize) -> String {
    std::fs::read_to_string(format!("./input/day{:0>2}.txt", day)).unwrap()
}

fn main() {
    let args = Args::parse();
    assert!(args.repeat > 0);

    let all_days = [
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
        day17::solve,
        day18::solve,
    ];

    let day_and_solver: Vec<_> = match args.day {
        None => all_days
            .into_iter()
            .enumerate()
            .map(|(i, solve)| (i + 1, solve, read_input(i + 1)))
            .collect(),
        Some(d) => vec![(d, all_days[d - 1], read_input(d))],
    };

    let total_duration = day_and_solver
        .into_iter()
        .map(|(day, solver, input)| {
            {
                let mut ctx = MeasureContext::new();

                for _ in 0..args.warmup {
                    black_box(solver(&mut ctx, black_box(&input)));
                }
            }

            let mut ctx = MeasureContext::new();
            let start = Instant::now();
            for _ in 0..args.repeat - 1 {
                black_box(solver(&mut ctx, black_box(&input)));
            }
            let SolutionTuple(p1, p2) = solver(&mut ctx, black_box(&input));
            let end = Instant::now();

            println!("day{}/part1: {}", day, p1);
            println!("day{}/part2: {}", day, p2);
            println!(
                "day{}/solve_time: {:?}{}",
                day,
                (end - start) / args.repeat,
                if ctx.measurements().next().is_none() {
                    "".to_string()
                } else {
                    format!(
                        " ({})",
                        ctx.measurements()
                            .map(|(label, duration)| {
                                format!("{}: {:?}", label, duration / args.repeat)
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            );
            (end - start) / args.repeat
        })
        .sum::<Duration>();
    if args.day.is_none() {
        println!("Total solve time: {:?}", total_duration);
    }
}
