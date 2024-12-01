use aoc_2024::day::*;
use aoc_2024::util::measure::MeasureContext;
use clap::Parser;
use std::hint::black_box;
use std::time::Instant;

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

    let all_days = [day01::solve];

    let day_and_solver: Vec<_> = match args.day {
        None => all_days
            .into_iter()
            .enumerate()
            .map(|(i, solve)| (i + 1, solve, read_input(i + 1)))
            .collect(),
        Some(d) => vec![(d, all_days[d - 1], read_input(d))],
    };

    let start = Instant::now();
    day_and_solver.into_iter().for_each(|(day, solver, input)| {
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
        let (p1, p2) = solver(&mut ctx, black_box(&input));
        let end = Instant::now();

        println!("day{}/part1: {}", day, p1);
        println!("day{}/part2: {}", day, p2);
        println!("day{}/solve_time: {:?}", day, (end - start) / args.repeat);
        ctx.measurements().for_each(|(label, duration)| {
            println!(
                "day{}/solve_time/{}: {:?}",
                day,
                label,
                duration / args.repeat
            );
        });
    });
    let end = Instant::now();
    println!("Total solve_time: {:?}", (end - start) / args.repeat);
}
