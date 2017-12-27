extern crate threadpool;
extern crate advent_solutions;

use advent_solutions::advent2017::*;

macro_rules! run_day {
    ($day:ident, $args:expr, $download:expr, $pool:expr, $tx:expr) => (
        {
            let day_string = stringify!($day);

            if $args.contains(&day_string) {
                let day_num_str = &day_string[day_string.len() - 2..];
                let day_num = day_num_str
                    .parse::<usize>()
                    .expect("Could not parse day number");

                let tx = $tx.clone();

                let input = $download.input(2017, day_num);

                $pool.execute(move || {
                    let parsed = $day::parse_input(&input);

                    tx.send(format!("Day {}/1: {}", day_num_str, $day::part1(&parsed)))
                        .unwrap();
                    tx.send(format!("Day {}/2: {}", day_num_str, $day::part2(&parsed)))
                        .unwrap();
                });
            }
        }
    )
}

macro_rules! run_day_both {
    ($day:ident, $args:expr, $download:expr, $pool:expr, $tx:expr) => (
        {
            let day_string = stringify!($day);

            if $args.contains(&day_string) {
                let day_num_str = &day_string[day_string.len() - 2..];
                let day_num = day_num_str
                    .parse::<usize>()
                    .expect("Could not parse day number");

                let tx = $tx.clone();

                let input = $download.input(2017, day_num);

                $pool.execute(move || {
                    let parsed = $day::parse_input(&input);
                    let (part1, part2) = $day::solve(&parsed);

                    tx.send(format!("Day {}/1: {}", day_num_str, part1))
                        .unwrap();
                    tx.send(format!("Day {}/2: {}", day_num_str, part2))
                        .unwrap();
                });
            }
        }
    )
}

fn main() {
    let str_args = std::env::args()
        .collect::<Vec<_>>();

    let args = if str_args.len() > 1 {
        str_args.iter()
            .map(|x| x.as_str())
            .collect::<Vec<_>>()
    } else {
        vec![
            "day01", "day02", "day03", "day04", "day05",
            "day06", "day07", "day08", "day09", "day10",
            "day11", "day12", "day13", "day14", "day15",
            "day16", "day17", "day18", "day19", /*"day20",
            "day21",*/ "day22", /*"day23", "day24", "day25",*/
        ]
    };

    let download = advent_solutions::Download::new();

    let pool = threadpool::Builder::new().build();
    let (tx, rx) = ::std::sync::mpsc::channel();

    run_day!(day01, args, download, pool, tx);
    run_day!(day02, args, download, pool, tx);
    run_day!(day03, args, download, pool, tx);
    run_day!(day04, args, download, pool, tx);
    run_day!(day05, args, download, pool, tx);
    run_day!(day06, args, download, pool, tx);
    run_day!(day07, args, download, pool, tx);
    run_day!(day08, args, download, pool, tx);
    run_day!(day09, args, download, pool, tx);
    run_day!(day10, args, download, pool, tx);
    run_day_both!(day11, args, download, pool, tx);
    run_day!(day12, args, download, pool, tx);
    run_day!(day13, args, download, pool, tx);
    run_day!(day14, args, download, pool, tx);
    run_day!(day15, args, download, pool, tx);
    run_day!(day16, args, download, pool, tx);
    run_day!(day17, args, download, pool, tx);
    run_day!(day18, args, download, pool, tx);
    run_day_both!(day19, args, download, pool, tx);
    run_day!(day22, args, download, pool, tx);

    let mut results = vec![];

    for string in rx.iter().take(args.len() * 2) {
        results.push(string);
    }

    results.sort();

    for result in results {
        println!("{}", result);
    }
}
