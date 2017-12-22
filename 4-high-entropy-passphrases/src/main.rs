extern crate advent;

fn main() {
    let input = advent::download_input(2017, 4);

    let step1 = input.lines()
        .filter(|line| line.split(' ')
            .all(|word| line.split(' ')
                .filter(|w| &word == w)
                .count() == 1
            )
        )
        .count();

    println!("Step 1: {}", step1);

    let step2 = input.lines()
        .filter(|line| line.split(' ')
            .all(|word| {
                let mut word_chars = word.chars().collect::<Vec<_>>();
                word_chars.sort();

                line.split(' ')
                    .filter(|w| {
                        let mut w_chars = w.chars().collect::<Vec<_>>();
                        w_chars.sort();

                        word_chars == w_chars
                    })
                    .count() == 1
            })
        )
        .count();

    println!("Step 2: {:?}", step2);
}
