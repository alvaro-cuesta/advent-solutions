extern crate advent;

fn step1(banks: &Vec<usize>) -> usize {
    use std::collections::HashSet;

    let mut banks = banks.clone();
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

    steps
}

fn step2(banks: &Vec<usize>) -> usize {
    use std::collections::HashMap;

    let mut banks = banks.clone();
    let mut cache = HashMap::new();
    let mut steps = 0usize;

    loop {
        if let Some(initial_steps) = cache.get(&banks) {
            return steps - initial_steps
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

fn main() {
    let banks = advent::download_single_input(2017, 6)
        .split('\t')
        .map(|x| x.parse::<usize>().expect("Unexpected non-integer number of block"))
        .collect::<Vec<_>>();

    println!("Step 1: {}", step1(&banks));
    println!("Step 2: {}", step2(&banks));
}
