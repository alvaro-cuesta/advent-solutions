extern crate advent;
extern crate itertools;

fn count_steps<F: Fn(isize) -> isize>(mut memory: Vec<isize>, mut_fn: F) -> usize {
    let mut ip = 0;

    let length = memory.len();

    itertools::repeat_call(|| {
        let old_ip = ip;
        ip = (ip as isize + memory[ip]) as usize;
        memory[old_ip] = mut_fn(memory[old_ip]);
        ip
    })
    .take_while(|&ip| ip < length)
    .count()
    + 1
}

fn main() {
    let input = advent::download_input(2017, 5);

    let jumps = input.split_terminator('\n')
        .map(|x| x.parse::<isize>().expect("Unexpected non-integer jump"))
        .collect::<Vec<_>>();

    let step1 = count_steps(jumps.clone(), |ip| ip + 1 );

    println!("Step 1: {}", step1);

    let step2 = count_steps(jumps.clone(), |ip| if ip >= 3 { ip - 1 } else { ip + 1 } );

    println!("Step 2: {}", step2);
}
