pub fn new_nums(max_n: u8) -> Vec<u8> {
    (0..(max_n as usize) + 1).map(|x| x as u8).collect()
}

/// To achieve this, begin with a *list* of numbers from `0` to `255`, a
/// *current position* which begins at `0` (the first element in the list),
/// a *skip size* (which starts at `0`), and a sequence of *lengths* (your
/// puzzle input). Then, for each length:
///
/// -   *Reverse* the order of that *length* of elements in the *list*,
///     starting with the element at the *current position*.
/// -   *Move* the *current position* forward by that *length* plus the
///     *skip size*.
/// -   *Increase* the *skip size* by one.
///
/// The *list* is circular; if the *current position* and the *length* try
/// to reverse elements beyond the end of the list, the operation reverses
/// using as many extra elements as it needs from the front of the list. If
/// the *current position* moves past the end of the list, it wraps around
/// to the front. *Lengths* larger than the size of the *list* are invalid.
///
/// Here's an example using a smaller list:
///
/// Suppose we instead only had a circular list containing five elements,
/// `0, 1, 2, 3, 4`, and were given input lengths of `3, 4, 1, 5`.
///
/// -   The list begins as `[0] 1 2 3 4` (where square brackets indicate the
///     *current position*).
/// -   The first length, `3`, selects `([0] 1 2) 3 4` (where parentheses
///     indicate the sublist to be reversed).
/// -   After reversing that section (`0 1 2` into `2 1 0`), we get
///     `([2] 1 0) 3 4`.
/// -   Then, the *current position* moves forward by the *length*, `3`,
///     plus the *skip size*, 0: `2 1 0 [3] 4`. Finally, the *skip size*
///     increases to `1`.
///
/// <!-- -->
///
/// -   The second length, `4`, selects a section which wraps:
///     `2 1) 0 ([3] 4`.
/// -   The sublist `3 4 2 1` is reversed to form `1 2 4 3`:
///     `4 3) 0 ([1] 2`.
/// -   The *current position* moves forward by the *length* plus the *skip
///     size*, a total of `5`, causing it not to move because it wraps
///     around: `4 3 0 [1] 2`. The *skip size* increases to `2`.
///
/// <!-- -->
///
/// -   The third length, `1`, selects a sublist of a single element, and so
///     reversing it has no effect.
/// -   The *current position* moves forward by the *length* (`1`) plus the
///     *skip size* (`2`): `4 [3] 0 1 2`. The *skip size* increases to `3`.
///
/// <!-- -->
///
/// -   The fourth length, `5`, selects every element starting with the
///     second: `4) ([3] 0 1 2`. Reversing this sublist (`3 0 1 2 4` into
///     `4 2 1 0 3`) produces: `3) ([4] 2 1 0`.
/// -   Finally, the *current position* moves forward by `8`: `3 4 2 1 [0]`.
///     The *skip size* increases to `4`.
///
/// ```
/// # use advent_solutions::advent2017::knot_hash;
/// let mut nums = knot_hash::new_nums(4);
///
/// knot_hash::hash(&mut nums, &[3, 4, 1, 5], 1);
/// assert_eq!(nums, &[3, 4, 2, 1, 0]);
/// ```
pub fn hash(nums: &mut [u8], lengths: &[u8], rounds: usize) {
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

/// In this example, the first two numbers in the list end up being `3` and
/// `4`; to check the process, you can multiply them together to produce
/// `12`.
///
/// ```
/// # use advent_solutions::advent2017::knot_hash;
/// let hashed = knot_hash::hash_lengths(4, &[3, 4, 1, 5], 1);
/// assert_eq!(hashed[0], 3);
/// assert_eq!(hashed[1], 4);
/// ```
///
pub fn hash_lengths(max_num: u8, lengths: &[u8], rounds: usize) -> Vec<u8> {
    let mut nums = new_nums(max_num);
    hash(&mut nums, &lengths, rounds);

    nums
}

/// The logic you've constructed forms a single *round* of the *Knot Hash*
/// algorithm; running the full thing requires many of these rounds. Some
/// input and output processing is also required.
///
/// First, from now on, your input should be taken not as a list of numbers,
/// but as a string of bytes instead. Unless otherwise specified, convert
/// characters to bytes using their [ASCII codes]. This will allow you to
/// handle arbitrary ASCII strings, and it also ensures that your input
/// lengths are never larger than `255`. For example, if you are given
/// `1,2,3`, you should convert it to the ASCII codes for each character:
/// `49,44,50,44,51`.
///
/// Once you have determined the sequence of lengths to use, add the
/// following lengths to the end of the sequence: `17, 31, 73, 47, 23`. For
/// example, if you are given `1,2,3`, your final sequence of lengths should
/// be `49,44,50,44,51,17,31,73,47,23` (the ASCII codes from the input
/// string combined with the standard length suffix values).
///
/// Second, instead of merely running one *round* like you did above, run a
/// total of `64` rounds, using the same *length* sequence in each round.
/// The *current position* and *skip size* should be preserved between
/// rounds. For example, if the previous example was your first round, you
/// would start your second round with the same *length* sequence
/// (`3, 4, 1, 5, 17, 31, 73, 47, 23`, now assuming they came from ASCII
/// codes and include the suffix), but start with the previous round's
/// *current position* (`4`) and *skip size* (`4`).
///
/// Once the rounds are complete, you will be left with the numbers from `0`
/// to `255` in some order, called the *sparse hash*. Your next task is to
/// reduce these to a list of only `16` numbers called the *dense hash*. To
/// do this, use numeric bitwise [XOR] to combine each consecutive block of
/// `16` numbers in the sparse hash (there are `16` such blocks in a list of
/// `256` numbers). So, the first element in the dense hash is the first
/// sixteen elements of the sparse hash XOR'd together, the second element
/// in the dense hash is the second sixteen elements of the sparse hash
/// XOR'd together, etc.
///
/// For example, if the first sixteen elements of your sparse hash are as
/// shown below, and the XOR operator is `^`, you would calculate the first
/// output number like this:
///
/// ```text
/// 65 ^ 27 ^ 9 ^ 1 ^ 4 ^ 3 ^ 40 ^ 50 ^ 91 ^ 7 ^ 6 ^ 0 ^ 2 ^ 5 ^ 68 ^ 22 = 64
/// ```
///
/// Perform this operation on each of the sixteen blocks of sixteen numbers
/// in your sparse hash to determine the sixteen numbers in your dense hash.
///
///   [ASCII codes]: https://en.wikipedia.org/wiki/ASCII#Printable_characters
///   [XOR]: https://en.wikipedia.org/wiki/Bitwise_operation#XOR
pub fn hash_str(input: &str, rounds: usize) -> Vec<u8> {
    let mut lengths = input.as_bytes().to_vec();
    lengths.extend([17, 31, 73, 47, 23].iter());

    let mut nums = new_nums(255);
    hash(&mut nums, &lengths, rounds);

    nums.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |state, x| state ^ x))
        .collect::<Vec<_>>()
}
