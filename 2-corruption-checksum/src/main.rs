extern crate advent;

fn step1<'a, I, J>(lines: I) -> u32
    where I: IntoIterator<Item=J>,
          J: IntoIterator<Item=&'a u32>,
{
    lines.into_iter()
        .map(|line| match advent::min_and_max(line) {
            Some((min, Some(max))) => max - min,
            Some((_, None)) => 0,
            _ => panic!("Unexpected empty line"),
        })
        .sum::<u32>()
}

fn step2<'a, I, J>(lines: I) -> u32
    where I: IntoIterator<Item=J>,
          J: IntoIterator<Item=&'a u32> + Copy,
{
    lines.into_iter()
        .map(|line| {
            let mut divisible_pairs = line.into_iter()
                .enumerate()
                .filter_map(|(i, x)| line.into_iter()
                    .skip(i + 1)
                    .find(|&y| (x % y) == 0 || (y % x) == 0)
                    .map(|y| (x, y))
                );

            let (x, y) = divisible_pairs.next().expect("No divisible pair found");

            assert!(divisible_pairs.next().is_none(), "More than one divisible pairs");

            if x > y { x / y } else { y / x }
        })
        .sum::<u32>()
}

fn main() {
    let lines = advent::download_input(2017, 2)
        .lines()
        .map(|line| line.split('\t')
            .map(|x| x.parse::<u32>().expect("Unexpected non-integer in spreadsheet"))
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    println!("Step 1: {}", step1(&lines));
    println!("Step 2: {}", step2(&lines));
}
