extern crate advent;

fn step1(input: &str) -> usize {
    input.lines()
        .filter(|line| line.split(' ')
            .all(|word| line.split(' ')
                .filter(|w| &word == w)
                .count() == 1
            )
        )
        .count()
}

fn step2(input: &str) -> usize {
    input.lines()
        .filter(|line| line.split(' ')
            .all(|a| {
                let mut a_chars = a.chars().collect::<Vec<_>>();
                a_chars.sort();

                line.split(' ')
                    .filter(|b| {
                        let mut b_chars = b.chars().collect::<Vec<_>>();
                        b_chars.sort();

                        a_chars == b_chars
                    })
                    .count() == 1
            })
        )
        .count()
}

fn main() {
    let input = advent::download_input(2017, 4);

    println!("Step 1: {}", step1(&input));
    println!("Step 2: {:?}", step2(&input));
}
