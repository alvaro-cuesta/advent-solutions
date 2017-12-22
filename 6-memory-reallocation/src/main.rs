extern crate advent;

fn main() {
    let mut input = advent::download_input(2017, 6);
    input.pop();

    let mut banks = input.split('\t')
        .map(|x| x.parse::<usize>().expect("Unexpected non-integer number of block"))
        .collect::<Vec<_>>();

    use std::collections::HashSet;
    let mut cache = HashSet::new();
    let mut steps = 0;

    loop {
        if !cache.insert(banks.clone()) {
            break;
        }

        let (idx, &blocks) = {
            let (_, max) = banks.iter().enumerate()
                .max_by_key(|&(_, blocks)| blocks)
                .unwrap();

            banks.iter().enumerate()
                .find(|&(_, blocks)| blocks == max)
                .unwrap()
        };

        banks[idx] = 0;

        for i in 1..blocks + 1 {
            let len = banks.len();
            banks[(idx + i) % len] += 1;
        }

        steps += 1;
    }

    println!("Step 1: {}", steps);

    use std::collections::HashMap;
    let mut cache = HashMap::new();
    let mut steps = 0;

    loop {
        if let Some(initial_steps) = cache.get(&banks) {
            println!("Step 2: {}", steps - initial_steps);
            break;
        }

        cache.insert(banks.clone(), steps);

        let (idx, &blocks) = {
            let (_, max) = banks.iter().enumerate()
                .max_by_key(|&(_, blocks)| blocks)
                .unwrap();

            banks.iter().enumerate()
                .find(|&(_, blocks)| blocks == max)
                .unwrap()
        };

        banks[idx] = 0;

        for i in 1..blocks + 1 {
            let len = banks.len();
            banks[(idx + i) % len] += 1;
        }

        steps += 1;
    }
}
