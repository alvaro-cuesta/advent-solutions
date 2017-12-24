extern crate advent;

fn main() {
    let input = advent::download_single_input(2017, 17)
        .parse::<usize>().expect("Could not parse input");

    let mut position = 0;
    let mut buffer = vec![0];

    for i in 0..2017 {
        position = 1 + (position + input) % buffer.len();
        buffer.insert(position, i);
    }

    println!("Step 1: {}", buffer[(position + 1) % buffer.len()]);

    let mut position = 0;
    let mut last_i = 0;

    for i in 1..50_000_000 {
        position = 1 + (position + input) % i;

        if position == 1 {
            last_i = i;
        }
    }

    println!("Step 2: {}", last_i);
}
