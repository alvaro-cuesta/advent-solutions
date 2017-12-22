extern crate advent;

fn main() {
    let input = advent::download_input(2017, 2);

    let lines = input.lines()
        .map(|line| line.split('\t')
            .map(|x| x.parse::<u32>().expect("Unexpected non-integer in spreadsheet"))
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let step1 = lines.iter()
        .map(|line| match advent::min_and_max(line.iter()) {
            Some((min, Some(max))) => max - min,
            Some((_, None)) => 0,
            _ => panic!("Unexpected empty line"),
        })
        .sum::<u32>();

    println!("Step 1: {}", step1);

    let step2 = lines.iter()
        .map(|line| {
            let mut divisible_pairs = line.iter()
                .enumerate()
                .filter_map(|(i, x)| {
                    line.iter()
                        .skip(i + 1)
                        .find(|&y| (x % y) == 0 || (y % x) == 0)
                        .map(|y| (x, y))
                });

            let (x, y) = divisible_pairs.next().expect("No divisible pair found");

            assert!(divisible_pairs.next().is_none(), "More than one divisible pairs");

            if x > y { x / y } else { y / x }
        })
        .sum::<u32>();

    println!("Step 2: {:?}", step2);
}
