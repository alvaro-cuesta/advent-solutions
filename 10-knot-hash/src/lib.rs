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

pub fn hash_lengths(lengths: &[u8], rounds: usize) -> Vec<u8> {
    let mut nums = new_nums();
    hash(&mut nums, &lengths, rounds);

    nums
}

pub fn hash_str(input: &str, rounds: usize) -> Vec<u8> {
    let mut lengths = input.as_bytes().to_vec();
    lengths.extend([17, 31, 73, 47, 23].iter());

    let mut nums = new_nums();
    hash(&mut nums, &lengths, rounds);

    nums.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |state, x| state ^ x))
        .collect::<Vec<_>>()
}
