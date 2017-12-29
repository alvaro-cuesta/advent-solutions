extern crate threadpool;
extern crate advent_solutions;

use advent_solutions::advent2017::*;

macro_rules! run_day {
    ($day:ident, $args:expr, $downloader:expr, $pool:expr, $tx:expr) => (
        {
            let day_string = stringify!($day);

            if $args.contains(&day_string) {
                let day_num_str = &day_string[day_string.len() - 2..];
                let day_num = day_num_str
                    .parse::<usize>()
                    .expect("Could not parse day number");

                let tx = $tx.clone();

                let input = $downloader.input(2017, day_num);

                $pool.execute(move || {
                    let parsed = $day::parse_input(&input);

                    tx.send((day_num_str, 1, format!("{}", $day::part1(&parsed))))
                        .unwrap();
                    tx.send((day_num_str, 2, format!("{}", $day::part2(&parsed))))
                        .unwrap();
                });
            }
        }
    )
}

macro_rules! run_day_both {
    ($day:ident, $args:expr, $downloader:expr, $pool:expr, $tx:expr) => (
        {
            let day_string = stringify!($day);

            if $args.contains(&day_string) {
                let day_num_str = &day_string[day_string.len() - 2..];
                let day_num = day_num_str
                    .parse::<usize>()
                    .expect("Could not parse day number");

                let tx = $tx.clone();

                let input = $downloader.input(2017, day_num);

                $pool.execute(move || {
                    let parsed = $day::parse_input(&input);
                    let (part1, part2) = $day::solve(&parsed);

                    tx.send((day_num_str, 1, format!("{}", part1)))
                        .unwrap();
                    tx.send((day_num_str, 2, format!("{}", part2)))
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
            "day16", "day17", "day18", "day19", "day20",
            "day21", "day22", "day23", "day24", "day25",
        ]
    };

    let downloader = advent_solutions::Downloader::new();

    let pool = threadpool::Builder::new().build();
    let (tx, rx) = ::std::sync::mpsc::channel();

    run_day!(day01, args, downloader, pool, tx);
    run_day!(day02, args, downloader, pool, tx);
    run_day!(day03, args, downloader, pool, tx);
    run_day!(day04, args, downloader, pool, tx);
    run_day!(day05, args, downloader, pool, tx);
    run_day!(day06, args, downloader, pool, tx);
    run_day!(day07, args, downloader, pool, tx);
    run_day!(day08, args, downloader, pool, tx);
    run_day!(day09, args, downloader, pool, tx);
    run_day!(day10, args, downloader, pool, tx);
    run_day_both!(day11, args, downloader, pool, tx);
    run_day!(day12, args, downloader, pool, tx);
    run_day!(day13, args, downloader, pool, tx);
    run_day!(day14, args, downloader, pool, tx);
    run_day!(day15, args, downloader, pool, tx);
    run_day!(day16, args, downloader, pool, tx);
    run_day!(day17, args, downloader, pool, tx);
    run_day!(day18, args, downloader, pool, tx);
    run_day_both!(day19, args, downloader, pool, tx);
    run_day!(day20, args, downloader, pool, tx);
    run_day!(day21, args, downloader, pool, tx);
    run_day!(day22, args, downloader, pool, tx);
    run_day!(day23, args, downloader, pool, tx);
    run_day!(day24, args, downloader, pool, tx);
    run_day!(day25, args, downloader, pool, tx);

    drop(tx);

    let mut results = vec![];

    for result in rx.iter().take(args.len() * 2) {
        results.push(result);
    }

    results.sort();

    for (day, part, result) in results {
        println!("Day {}/{}: {}", day, part, result);
    }
}
