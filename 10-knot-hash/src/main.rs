extern crate advent;

fn new_nums() -> Vec<u8> {
    let mut nums = vec![];

    for i in 0..256 {
        nums.push(i as u8);
    }

    nums
}

fn hash(nums: &mut [u8], lengths: &[u8], rounds: usize) {
    let mut current_pos = 0;
    let mut skip_size = 0;

    for _ in 0..rounds {
        for length in lengths {
            for i in 0..length/2 {
                let ia = (current_pos + i as usize) % nums.len();
                let ib = (current_pos + (length - i - 1) as usize) % nums.len();

                nums.swap(ia, ib);
            }

            current_pos += (*length as usize) + skip_size;
            current_pos %= nums.len();

            skip_size += 1;
            skip_size %= nums.len();
        }
    }
}

fn step1(input: &str) -> u16 {
    let lengths = input
        .split(',')
        .map(|l| l.parse::<u8>().expect("Unexpected non-u8 length"))
        .collect::<Vec<_>>();

    let mut nums = new_nums();
    hash(&mut nums, &lengths, 1);

    (nums[0] as u16) * (nums[1] as u16)
}

fn step2(input: &str) -> String {
    let mut lengths = input.as_bytes().to_vec();
    lengths.extend([17, 31, 73, 47, 23].iter());

    let mut nums = new_nums();
    hash(&mut nums, &lengths, 64);

    nums.chunks(16)
        .map(|chunk| {
            let x = chunk.iter().fold(0, |state, x| state ^ x);
            format!("{:02x}", x)
        })
        .collect::<String>()
}

fn main() {
    let input = advent::download_single_input(2017, 10);

    println!("Step 1: {}", step1(&input));
    println!("Step 2: {}", step2(&input));
}

#[cfg(test)]
mod test {
    #[test]
    fn empty() {
        assert_eq!(&::step2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn aoc2017() {
        assert_eq!(&::step2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn one_two_three() {
        assert_eq!(&::step2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn one_two_four() {
        assert_eq!(&::step2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
