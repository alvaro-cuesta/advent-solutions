extern crate advent;

fn main() {
    let mut input = advent::download_input(2017, 1);
    input.pop();

    let step1 = input.chars()
        .zip(input.chars().cycle().skip(1))
        .filter_map(|(a, b)| if a == b { Some(a.to_digit(10).unwrap()) } else { None })
        .sum::<u32>();

    println!("Step 1: {}", step1);

    let step2 = input.chars()
        .zip(input.chars().cycle().skip(input.len() / 2))
        .filter_map(|(a, b)| if a == b { Some(a.to_digit(10).unwrap()) } else { None })
        .sum::<u32>();

    println!("Step 2: {}", step2);
}
