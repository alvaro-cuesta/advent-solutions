extern crate advent;

fn day1(input: &str, offset: usize) -> u32 {
    input.chars()
        .zip(input.chars().cycle().skip(offset))
        .filter_map(|(a, b)| if a == b {
                Some(a.to_digit(10).expect("Unexpected non-digit in string"))
            } else {
                None
            }
        )
        .sum::<u32>()
}

fn main() {
    let mut input = advent::download_single_input(2017, 1);

    println!("Step 1: {}", day1(&input, 1));
    println!("Step 2: {}", day1(&input, input.len() / 2));
}
