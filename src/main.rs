extern crate advent_solutions;

use advent_solutions::advent2017::*;

macro_rules! run_day {
    ($day:ident, $args:expr, $download:expr) => (
        {
            let day_string = stringify!($day);
            let day_num_str = &day_string[day_string.len() - 2..];
            let day_num = day_num_str
                .parse::<usize>()
                .expect("Could not parse day number");

            if $args.len() == 1 || $args.contains(&day_string) {
                let input = $download.input(2017, day_num);
                let parsed = $day::parse_input(&input);

                println!("Day {}/1: {}", day_num_str, $day::part1(&parsed));
                println!("Day {}/2: {}", day_num_str, $day::part2(&parsed));
            }
        }
    )
}

macro_rules! run_day_both {
    ($day:ident, $args:expr, $download:expr) => (
        {
            let day_string = stringify!($day);
            let day_num_str = &day_string[day_string.len() - 2..];
            let day_num = day_num_str
                .parse::<usize>()
                .expect("Could not parse day number");

            if $args.len() == 1 || $args.contains(&day_string) {
                let input = $download.input(2017, day_num);
                let parsed = $day::parse_input(&input);
                let (part1, part2) = $day::solve(&parsed);

                println!("Day {}/1: {}", day_num_str, part1);
                println!("Day {}/2: {}", day_num_str, part2);
            }
        }
    )
}

fn main() {
    let str_args = std::env::args()
        .collect::<Vec<_>>();

    let args = str_args.iter()
        .map(|x| x.as_str())
        .collect::<Vec<_>>();

    let download = advent_solutions::Download::new();

    run_day!(day01, args, download);
    run_day!(day02, args, download);
    run_day!(day03, args, download);
    run_day!(day04, args, download);
    run_day!(day05, args, download);
    run_day!(day06, args, download);
    run_day!(day07, args, download);
    run_day!(day08, args, download);
    run_day!(day09, args, download);
    run_day!(day10, args, download);
    run_day_both!(day11, args, download);
    run_day!(day12, args, download);
    run_day!(day13, args, download);
    run_day!(day14, args, download);
    run_day!(day15, args, download);
    run_day!(day16, args, download);
    run_day!(day17, args, download);
    run_day!(day18, args, download);
    run_day_both!(day19, args, download);
    run_day!(day22, args, download);
}
