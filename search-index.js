var searchIndex = {};
searchIndex["advent_solutions"] = {"doc":"Solutions for [Advent of Code].","items":[[3,"Downloader","advent_solutions","",null,null],[4,"Direction","","",null,null],[13,"Up","","",0,null],[13,"Right","","",0,null],[13,"Down","","",0,null],[13,"Left","","",0,null],[11,"new","","",1,{"inputs":[],"output":{"name":"downloader"}}],[11,"input","","",1,{"inputs":[{"name":"self"},{"name":"usize"},{"name":"usize"}],"output":{"name":"string"}}],[0,"parse","","",null,null],[5,"name","advent_solutions::parse","",null,null],[5,"unsigned_number","","",null,null],[5,"signed_number","","",null,null],[0,"iter","advent_solutions","",null,null],[3,"Bits","advent_solutions::iter","",null,null],[5,"min_and_max","","",null,{"inputs":[{"name":"i"}],"output":{"name":"option"}}],[5,"min_and_max_by_key","","",null,{"inputs":[{"name":"i"},{"name":"f"}],"output":{"name":"option"}}],[11,"clone","","",2,{"inputs":[{"name":"self"}],"output":{"name":"bits"}}],[11,"eq","","",2,{"inputs":[{"name":"self"},{"name":"bits"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"self"},{"name":"bits"}],"output":{"name":"bool"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",2,null],[11,"new","","",2,{"inputs":[{"name":"usize"},{"name":"usize"}],"output":{"name":"bits"}}],[11,"next","","",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"clone","advent_solutions","",0,{"inputs":[{"name":"self"}],"output":{"name":"direction"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"direction"}],"output":{"name":"bool"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",0,null],[11,"cw","","",0,{"inputs":[{"name":"self"}],"output":{"name":"direction"}}],[11,"reverse","","",0,{"inputs":[{"name":"self"}],"output":{"name":"direction"}}],[11,"ccw","","",0,{"inputs":[{"name":"self"}],"output":{"name":"direction"}}],[11,"into","","",0,null],[11,"add","","",0,null],[11,"add","","",0,null],[0,"advent2017","","Solutions for [Advent of Code 2017].",null,null],[0,"knot_hash","advent_solutions::advent2017","",null,null],[5,"new_nums","advent_solutions::advent2017::knot_hash","",null,{"inputs":[{"name":"u8"}],"output":{"generics":["u8"],"name":"vec"}}],[5,"hash","","To achieve this, begin with a list of numbers from `0` to `255`, a current position which begins at `0` (the first element in the list), a skip size (which starts at `0`), and a sequence of lengths (your puzzle input). Then, for each length:",null,null],[5,"hash_lengths","","In this example, the first two numbers in the list end up being `3` and `4`; to check the process, you can multiply them together to produce `12`.",null,null],[5,"hash_str","","The logic you've constructed forms a single round of the Knot Hash algorithm; running the full thing requires many of these rounds. Some input and output processing is also required.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"generics":["u8"],"name":"vec"}}],[0,"day01","advent_solutions::advent2017","Day 1: Inverse Captcha",null,null],[5,"count_matching","advent_solutions::advent2017::day01","Finds the sum of all digits that match the digit offset by `offset` in the list.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"u32"}}],[5,"part1","","It goes on to explain that you may only leave by solving a [captcha] to prove you're not a human. Apparently, you only get one millisecond to solve the captcha: too fast for a normal human, but it feels like hours to you.",null,{"inputs":[{"name":"str"}],"output":{"name":"u32"}}],[5,"part2","","You notice a progress bar that jumps to 50% completion. Apparently, the door isn't yet satisfied, but it did emit a star as encouragement. The instructions change:",null,{"inputs":[{"name":"str"}],"output":{"name":"u32"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"str"}}],[0,"day02","advent_solutions::advent2017","Day 2: Corruption Checksum",null,null],[5,"part1","advent_solutions::advent2017::day02","The spreadsheet consists of rows of apparently-random numbers. To make sure the recovery process is on the right track, they need you to calculate the spreadsheet's checksum. For each row, determine the difference between the largest value and the smallest value; the checksum is the sum of all of these differences.",null,{"inputs":[{"name":"i"}],"output":{"name":"usize"}}],[5,"part2","","\"Great work; looks like we're on the right track after all. Here's a star for your effort.\" However, the program seems a little worried. Can programs be worried?",null,{"inputs":[{"name":"i"}],"output":{"name":"usize"}}],[5,"parse_input","","Parses input into a grid of numbers.",null,{"inputs":[{"name":"str"}],"output":{"generics":["vec"],"name":"vec"}}],[0,"day03","advent_solutions::advent2017","Day 3: Spiral Memory",null,null],[5,"part1","advent_solutions::advent2017::day03","While this is very space-efficient (no squares are skipped), requested data must be carried back to square `1` (the location of the only access port for this memory system) by programs that can only move up, down, left, or right. They always take the shortest path: the [Manhattan Distance] between the location of the data and square `1`.",null,{"inputs":[{"name":"usize"}],"output":{"name":"usize"}}],[5,"stress_test","","As a stress test on the system, the programs here clear the grid and then store the value `1` in square `1`. Then, in the same allocation order as shown above, they store the sum of the values in all adjacent squares, including diagonals.",null,null],[5,"part2","","What is the first value written that is larger than your puzzle input?",null,{"inputs":[{"name":"usize"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"usize"}}],[0,"day04","advent_solutions::advent2017","Day 4: High-Entropy Passphrases",null,null],[5,"is_valid_part1","advent_solutions::advent2017::day04","To ensure security, a valid passphrase must contain no duplicate words.",null,{"inputs":[{"name":"str"}],"output":{"name":"bool"}}],[5,"part1","","The system's full passphrase list is available as your puzzle input. How many passphrases are valid?",null,{"inputs":[{"name":"str"}],"output":{"name":"usize"}}],[5,"is_valid_part2","","For added security, yet another system policy  has been put in place. Now, a valid passphrase must contain no two words that are anagrams of each other - that is, a passphrase is invalid if any word's letters can be rearranged to form any other word in the passphrase.",null,{"inputs":[{"name":"str"}],"output":{"name":"bool"}}],[5,"part2","","Under this new system policy, how many passphrases are valid?",null,{"inputs":[{"name":"str"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"str"}}],[0,"day05","advent_solutions::advent2017","Day 5: A Maze of Twisty Trampolines, All Alike",null,null],[5,"parse_input","advent_solutions::advent2017::day05","The message includes a list of the offsets for each jump. Jumps are relative: `-1` moves to the previous instruction, and `2` skips the next one. Start at the first instruction in the list. The goal is to follow the jumps until one leads outside the list.",null,{"inputs":[{"name":"str"}],"output":{"generics":["isize"],"name":"vec"}}],[5,"part1","","In addition, these instructions are a little strange; after each jump, the offset of that instruction increases by `1`. So, if you come across an offset of `3`, you would move three instructions forward, but change it to a `4` for the next time it is encountered.",null,{"inputs":[{"name":"vec"}],"output":{"name":"usize"}}],[5,"part2","","Now, the jumps are even stranger: after each jump, if the offset was three or more, instead decrease it by `1`. Otherwise, increase it by `1` as before.",null,{"inputs":[{"name":"vec"}],"output":{"name":"usize"}}],[5,"count_steps","","Counts the steps required to exit the maze, given a instruction mutation function `mut_fn`.",null,{"inputs":[{"generics":["isize"],"name":"vec"},{"name":"f"}],"output":{"name":"usize"}}],[0,"day06","advent_solutions::advent2017","Day 6: Memory Reallocation",null,null],[5,"parse_input","advent_solutions::advent2017::day06","In this area, there are sixteen memory banks; each memory bank can hold any number of blocks. The goal of the reallocation routine is to balance the blocks between the memory banks.",null,{"inputs":[{"name":"str"}],"output":{"generics":["usize"],"name":"vec"}}],[5,"part1","","The reallocation routine operates in cycles. In each cycle, it finds the memory bank with the most blocks (ties won by the lowest-numbered memory bank) and redistributes those blocks among the banks. To do this, it removes all of the blocks from the selected bank, then moves to the next (by index) memory bank and inserts one of the blocks. It continues doing this until it runs out of blocks; if it reaches the last memory bank, it wraps around to the first one.",null,{"inputs":[{"name":"vec"}],"output":{"name":"usize"}}],[5,"part2","","Out of curiosity, the debugger would also like to know the size of the loop: starting from a state that has already been seen, how many block redistribution cycles must be performed before that same state is seen again?",null,{"inputs":[{"name":"vec"}],"output":{"name":"usize"}}],[0,"day07","advent_solutions::advent2017","Day 7: Recursive Circus",null,null],[3,"Node","advent_solutions::advent2017::day07","You offer to help, but first you need to understand the structure of these towers. You ask each program to yell out their name, their weight, and (if they're holding a disc) the names of the programs immediately above them balancing on that disc. You write this information down (your puzzle input). Unfortunately, in their panic, they don't do this in an orderly fashion; by the time you're done, you're not sure which program gave which information.",null,null],[5,"part1","","Before you're ready to help them, you need to make sure your information is correct. What is the name of the bottom program?",null,null],[5,"part2","","The programs explain the situation: they can't get down. Rather, they could get down, if they weren't expending all of their energy trying to keep the tower balanced. Apparently, one program has the wrong weight, and until it's fixed, they're stuck here.",null,null],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"generics":["node"],"name":"vec"}}],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"node"}}],[11,"eq","","",3,{"inputs":[{"name":"self"},{"name":"node"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"self"},{"name":"node"}],"output":{"name":"bool"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",3,null],[11,"from_bytes","","",3,null],[11,"list_from_bytes","","",3,null],[0,"day08","advent_solutions::advent2017","Day 8: I Heard You Like Registers",null,null],[3,"Instruction","advent_solutions::advent2017::day08","Each instruction consists of several parts: the register to modify, whether to increase or decrease that register's value, the amount by which to increase or decrease it, and a condition. If the condition fails, skip the instruction without modifying the register. The registers all start at `0`. The instructions look like this:",null,null],[5,"part1","","What is the largest value in any register after completing the instructions in your puzzle input?",null,null],[5,"part2","","To be safe, the CPU also needs to know the highest value held in any register during this process so that it can decide how much memory to allocate to these operations. For example, in the above instructions, the highest value ever held was `10` (in register `c` after the third instruction was evaluated).",null,null],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"generics":["instruction"],"name":"vec"}}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"instruction"}}],[11,"eq","","",4,{"inputs":[{"name":"self"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"self"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",4,null],[11,"from_bytes","","",4,null],[11,"list_from_bytes","","",4,null],[11,"exec","","",4,{"inputs":[{"name":"self"},{"name":"hashmap"}],"output":null}],[0,"day09","advent_solutions::advent2017","Day 9: Stream Processing",null,null],[4,"Node","advent_solutions::advent2017::day09","You sit for a while and record part of the stream (your puzzle input). The characters represent groups - sequences that begin with `{` and end with `}`. Within a group, there are zero or more other things, separated by commas: either another group or garbage. Since groups can contain other groups, a `}` only closes the most-recently-opened unclosed group - that is, they are nestable. Your puzzle input represents a single, large group which itself contains many smaller ones.",null,null],[13,"Group","","",5,null],[13,"Garbage","","",5,null],[5,"part1","","Your goal is to find the total score for all groups in your input. Each group is assigned a score which is one more than the score of the group that immediately contains it. (The outermost group gets a score of `1`.)",null,{"inputs":[{"name":"node"}],"output":{"name":"usize"}}],[5,"part2","","Now, you're ready to remove the garbage.",null,{"inputs":[{"name":"node"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"node"}}],[11,"clone","","",5,{"inputs":[{"name":"self"}],"output":{"name":"node"}}],[11,"eq","","",5,{"inputs":[{"name":"self"},{"name":"node"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"self"},{"name":"node"}],"output":{"name":"bool"}}],[11,"fmt","","",5,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",5,null],[11,"score","","",5,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"usize"}}],[11,"count_garbage","","",5,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"from_bytes","","",5,null],[0,"day10","advent_solutions::advent2017","Day 10: Knot Hash",null,null],[5,"part1","advent_solutions::advent2017::day10","However, you should instead use the standard list size of `256` (with values `0` to `255`) and the sequence of lengths in your puzzle input. Once this process is complete, what is the result of multiplying the first two numbers in the list?",null,{"inputs":[{"name":"str"}],"output":{"name":"u16"}}],[5,"part2","","Finally, the standard way to represent a Knot Hash is as a single [hexadecimal] string; the final output is the dense hash in hexadecimal notation. Because each number in your dense hash will be between `0` and `255` (inclusive), always represent each number as two hexadecimal digits (including a leading zero as necessary). So, if your first three numbers are `64, 7, 255`, they correspond to the hexadecimal numbers `40, 07, ff`, and so the first six characters of the hash would be `4007ff`. Because every Knot Hash is sixteen such numbers, the hexadecimal representation is always `32` hexadecimal digits (`0`-`f`) long.",null,{"inputs":[{"name":"str"}],"output":{"name":"string"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"str"}}],[0,"day11","advent_solutions::advent2017","Day 11: Hex Ed",null,null],[5,"solve","advent_solutions::advent2017::day11","You have the path the child process took. Starting where he started, you need to determine the fewest number of steps required to reach him. (A \"step\" means to move from the hex you are in to any adjacent hex.)",null,null],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"str"}}],[0,"day12","advent_solutions::advent2017","Day 12: Digital Plumber",null,null],[5,"parse_connections","advent_solutions::advent2017::day12","",null,null],[5,"part1","","You need to figure out how many programs are in the group that contains program ID `0`.",null,{"inputs":[{"name":"hashmap"}],"output":{"name":"usize"}}],[5,"part2","","There are more programs than just the ones in the group containing program ID `0`. The rest of them have no way of reaching that group, and still might have no way of reaching each other.",null,{"inputs":[{"name":"hashmap"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"generics":["usize","vec"],"name":"hashmap"}}],[0,"day13","advent_solutions::advent2017","Day 13: Packet Scanners",null,null],[3,"Layer","advent_solutions::advent2017::day13","Within each layer, a security scanner moves back and forth within its range. Each security scanner starts at the top and moves down until it reaches the bottom, then moves up until it reaches the top, and repeats. A security scanner takes one picosecond to move one step. Drawing scanners as `S`, the first few picoseconds look like this:",null,null],[3,"Firewall","","By studying the firewall briefly, you are able to record (in your puzzle input) the depth of each layer and the range of the scanning area for the scanner within it, written as `depth: range`. Each layer has a thickness of exactly `1`. A layer at depth `0` begins immediately inside the firewall; a layer at depth `1` would start immediately after that.",null,null],[5,"part1","","Your plan is to hitch a ride on a packet about to move through the firewall. The packet will travel along the top of each layer, and it moves at one layer per picosecond. Each picosecond, the packet moves one layer forward (its first move takes it into layer 0), and then the scanners move one step. If there is a scanner at the top of the layer as your packet enters it, you are caught. (If a scanner moves into the top of its layer while you are there, you are not caught: it doesn't have time to notice you before you leave.) If you were to do this in the configuration above, marking your current position with parentheses, your passage through the firewall would look like this:",null,{"inputs":[{"name":"str"}],"output":{"name":"usize"}}],[5,"part2","","Now, you need to pass through the firewall without being caught - easier said than done.",null,{"inputs":[{"name":"str"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"str"}}],[11,"clone","","",6,{"inputs":[{"name":"self"}],"output":{"name":"layer"}}],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"layer"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"self"},{"name":"layer"}],"output":{"name":"bool"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",6,null],[11,"from_str","","",6,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"clone","","",7,{"inputs":[{"name":"self"}],"output":{"name":"firewall"}}],[11,"eq","","",7,{"inputs":[{"name":"self"},{"name":"firewall"}],"output":{"name":"bool"}}],[11,"ne","","",7,{"inputs":[{"name":"self"},{"name":"firewall"}],"output":{"name":"bool"}}],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",7,null],[11,"step","","",7,{"inputs":[{"name":"self"}],"output":null}],[11,"index","","",7,null],[11,"index_mut","","",7,null],[0,"day14","advent_solutions::advent2017","Day 14: Disk Defragmentation",null,null],[3,"Grid","advent_solutions::advent2017::day14","The disk in question consists of a 128x128 grid; each square of the grid is either free or used. On this disk, the state of the grid is tracked by the bits in a sequence of [knot hashes].",null,null],[5,"make_hashes","","A total of 128 knot hashes are calculated, each corresponding to a single row in the grid; each hash contains 128 bits which correspond to individual grid squares. Each bit of a hash indicates whether that square is free (`0`) or used (`1`).",null,{"inputs":[{"name":"str"}],"output":{"generics":["vec"],"name":"vec"}}],[5,"part1","","The output of a knot hash is traditionally represented by 32 hexadecimal digits; each of these digits correspond to 4 bits, for a total of `4 * 32 = 128` bits. To convert to bits, turn each hexadecimal digit to its equivalent binary value, high-bit first: `0` becomes `0000`, `1` becomes `0001`, `e` becomes `1110`, `f` becomes `1111`, and so on; a hash that begins with `a0c2017...` in hexadecimal would begin with `10100000110000100000000101110000...` in binary.",null,{"inputs":[{"name":"i"}],"output":{"name":"u32"}}],[5,"part2","","Now, all the defragmenter needs to know is the number of regions. A region is a group of used squares that are all adjacent, not including diagonals. Every used square is in exactly one region: lone used squares form their own isolated regions, while several adjacent squares all count as a single region.",null,{"inputs":[{"name":"i"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"generics":["vec"],"name":"vec"}}],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"day15","advent_solutions::advent2017","Day 15: Dueling Generators",null,null],[3,"Generator","advent_solutions::advent2017::day15","The generators both work on the same principle. To create its next value, a generator will take the previous value it produced, multiply it by a factor (generator A uses `16807`; generator B uses `48271`), and then keep the remainder of dividing that resulting product by `2147483647`. That final remainder is the value it produces next.",null,null],[5,"part1","","Here, you can see that the lowest (here, rightmost) 16 bits of the third value match: `1110001101001010`. Because of this one match, after processing these five pairs, the judge would have added only `1` to its total.",null,null],[5,"part2","","In the interest of trying to align a little better, the generators get more picky about the numbers they actually give to the judge.",null,null],[5,"parse_input","","",null,null],[11,"new","","",9,{"inputs":[{"name":"u32"},{"name":"u32"}],"output":{"name":"generator"}}],[11,"next","","",9,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[0,"day16","advent_solutions::advent2017","Day 16: Permutation Promenade",null,null],[4,"Move","advent_solutions::advent2017::day16","The programs' dance consists of a sequence of dance moves:",null,null],[13,"Spin","","",10,null],[13,"Exchange","","",10,null],[13,"Partner","","",10,null],[5,"part1","","You watch the dance for a while and record their dance moves (your puzzle input). In what order are the programs standing after their dance?",null,null],[5,"part2","","Now that you're starting to get a feel for the dance moves, you turn your attention to the dance as a whole.",null,null],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"generics":["move"],"name":"vec"}}],[11,"clone","","",10,{"inputs":[{"name":"self"}],"output":{"name":"move"}}],[11,"eq","","",10,{"inputs":[{"name":"self"},{"name":"move"}],"output":{"name":"bool"}}],[11,"ne","","",10,{"inputs":[{"name":"self"},{"name":"move"}],"output":{"name":"bool"}}],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",10,null],[11,"apply","","",10,{"inputs":[{"name":"self"},{"generics":["char"],"name":"vec"}],"output":{"generics":["char"],"name":"vec"}}],[11,"list_from_bytes","","",10,null],[0,"day17","advent_solutions::advent2017","Day 17: Spinlock",null,null],[5,"part1","advent_solutions::advent2017::day17","For example, if the spinlock were to step `3` times per insert, the circular buffer would begin to evolve like this (using parentheses to mark the current position after each iteration of the algorithm):",null,{"inputs":[{"name":"usize"}],"output":{"name":"usize"}}],[5,"part2","","The spinlock does not short-circuit. Instead, it gets more angry. At least, you assume that's what happened; it's spinning significantly faster than it was a moment ago.",null,{"inputs":[{"name":"usize"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"usize"}}],[0,"day18","advent_solutions::advent2017","Day 18: Duet",null,null],[4,"Value","advent_solutions::advent2017::day18","Many of the instructions can take either a register (a single letter) or a number. The value of a register is the integer it contains; the value of a number is that number.",null,null],[13,"Literal","","",11,null],[13,"Register","","",11,null],[4,"Instruction","","There aren't that many instructions, so it shouldn't be hard to figure out what they do. Here's what you determine:",null,null],[13,"Snd","","",12,null],[13,"Set","","",12,null],[13,"Add","","",12,null],[13,"Mul","","",12,null],[13,"Mod","","",12,null],[13,"Rcv","","",12,null],[13,"Jgz","","",12,null],[5,"part1","","For example:",null,null],[5,"part2","","As you congratulate yourself for a job well done, you notice that the documentation has been on the back of the tablet this entire time. While you actually got most of the instructions correct, there are a few key differences. This assembly code isn't about sound at all - it's meant to be run twice at the same time.",null,null],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"generics":["instruction"],"name":"vec"}}],[11,"clone","","",11,{"inputs":[{"name":"self"}],"output":{"name":"value"}}],[11,"eq","","",11,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"ne","","",11,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"fmt","","",11,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",11,null],[11,"from_bytes","","",11,null],[11,"from","","",11,{"inputs":[{"name":"isize"}],"output":{"name":"self"}}],[11,"from","","",11,{"inputs":[{"name":"char"}],"output":{"name":"self"}}],[11,"clone","","",12,{"inputs":[{"name":"self"}],"output":{"name":"instruction"}}],[11,"eq","","",12,{"inputs":[{"name":"self"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"ne","","",12,{"inputs":[{"name":"self"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"fmt","","",12,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",12,null],[11,"from_bytes","","",12,null],[11,"list_from_bytes","","",12,null],[0,"day19","advent_solutions::advent2017","Day 19: A Series of Tubes",null,null],[5,"solve","advent_solutions::advent2017::day19","The little packet looks up at you, hoping you can help it find the way. What letters will it see (in the order it would see them) if it follows the path? (The routing diagram is very wide; make sure you view it without line wrapping.)",null,null],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"str"}}],[0,"day20","advent_solutions::advent2017","Day 20: Particle Swarm",null,null],[3,"Particle","advent_solutions::advent2017::day20","",null,null],[5,"part1","","Each tick, all particles are updated simultaneously. A particle's properties are updated in the following order:",null,{"inputs":[{"name":"vec"}],"output":{"name":"usize"}}],[5,"part2","","To simplify the problem further, the GPU would like to remove any particles that collide. Particles collide if their positions ever exactly match. Because particles are updated simultaneously, more than two particles can collide at the same time and place. Once particles collide, they are removed and cannot collide with anything else after that tick.",null,{"inputs":[{"name":"vec"}],"output":{"name":"usize"}}],[5,"parse_input","","It transmits to you a buffer (your puzzle input) listing each particle in order (starting with particle `0`, then particle `1`, particle `2`, and so on). For each particle, it provides the `X`, `Y`, and `Z` coordinates for the particle's position (`p`), velocity (`v`), and acceleration (`a`), each in the format `<X,Y,Z>`.",null,{"inputs":[{"name":"str"}],"output":{"generics":["particle"],"name":"vec"}}],[11,"clone","","",13,{"inputs":[{"name":"self"}],"output":{"name":"particle"}}],[11,"eq","","",13,{"inputs":[{"name":"self"},{"name":"particle"}],"output":{"name":"bool"}}],[11,"ne","","",13,{"inputs":[{"name":"self"},{"name":"particle"}],"output":{"name":"bool"}}],[11,"fmt","","",13,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",13,null],[0,"day21","advent_solutions::advent2017","Day 21: Fractal Art",null,null],[3,"Grid","advent_solutions::advent2017::day21","The image consists of a two-dimensional square grid of pixels that are either on (`#`) or off (`.`).",null,null],[5,"solve","","The program always begins with this pattern:",null,{"inputs":[{"name":"hashmap"},{"name":"usize"}],"output":{"name":"usize"}}],[5,"part1","","How many pixels stay on after `5` iterations?",null,{"inputs":[{"name":"hashmap"}],"output":{"name":"usize"}}],[5,"part2","","How many pixels stay on after `18` iterations?",null,{"inputs":[{"name":"hashmap"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"generics":["grid","grid"],"name":"hashmap"}}],[11,"clone","","",14,{"inputs":[{"name":"self"}],"output":{"name":"grid"}}],[11,"eq","","",14,{"inputs":[{"name":"self"},{"name":"grid"}],"output":{"name":"bool"}}],[11,"ne","","",14,{"inputs":[{"name":"self"},{"name":"grid"}],"output":{"name":"bool"}}],[11,"fmt","","",14,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",14,null],[11,"from_bytes","","",14,null],[11,"fmt","","",14,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"day22","advent_solutions::advent2017","Day 22: Sporifica Virus",null,null],[3,"Memory","advent_solutions::advent2017::day22","Diagnostics have also provided a map of the node infection status (your puzzle input). Clean nodes are shown as `.`; infected nodes are shown as `#`. This map only shows the center of the grid; there are many more nodes beyond those shown, but none of them are currently infected.",null,null],[3,"Carrier","","To [prevent overloading] the nodes (which would render them useless to the virus) or detection by system administrators, exactly one virus carrier moves through the network, infecting or cleaning nodes as it moves. The virus carrier is always located on a single node in the network (the current node) and keeps track of the direction it is facing.",null,null],[4,"Node","","",null,null],[13,"Clean","","",15,null],[13,"Weakened","","",15,null],[13,"Infected","","",15,null],[13,"Flagged","","",15,null],[5,"part1_with_bursts","","For example, suppose you are given a map like this:",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"usize"}}],[5,"part1","","",null,{"inputs":[{"name":"str"}],"output":{"name":"usize"}}],[5,"part2_with_bursts","","Start with the same map (still using `.` for clean and `#` for infected) and still with the virus carrier starting in the middle and facing up.",null,{"inputs":[{"name":"str"},{"name":"usize"}],"output":{"name":"usize"}}],[5,"part2","","",null,{"inputs":[{"name":"str"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"str"}}],[11,"clone","","",15,{"inputs":[{"name":"self"}],"output":{"name":"node"}}],[11,"eq","","",15,{"inputs":[{"name":"self"},{"name":"node"}],"output":{"name":"bool"}}],[11,"fmt","","",15,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",15,null],[11,"try_from","","",15,{"inputs":[{"name":"char"}],"output":{"name":"result"}}],[11,"into","","",15,{"inputs":[{"name":"self"}],"output":{"name":"char"}}],[11,"fmt","","",15,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",16,{"inputs":[{"name":"self"}],"output":{"name":"memory"}}],[11,"eq","","",16,{"inputs":[{"name":"self"},{"name":"memory"}],"output":{"name":"bool"}}],[11,"ne","","",16,{"inputs":[{"name":"self"},{"name":"memory"}],"output":{"name":"bool"}}],[11,"fmt","","",16,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",16,null],[11,"index","","",16,null],[11,"index_mut","","",16,null],[11,"fmt","","",16,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",17,{"inputs":[{"name":"self"}],"output":{"name":"carrier"}}],[11,"eq","","",17,{"inputs":[{"name":"self"},{"name":"carrier"}],"output":{"name":"bool"}}],[11,"ne","","",17,{"inputs":[{"name":"self"},{"name":"carrier"}],"output":{"name":"bool"}}],[11,"fmt","","",17,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",17,null],[11,"part2_with_bursts","","As you go to remove the virus from the infected nodes, it evolves to resist your attempt.",17,{"inputs":[{"name":"self"},{"name":"memory"}],"output":null}],[0,"day23","advent_solutions::advent2017","Day 23: Coprocessor Conflagration",null,null],[4,"Value","advent_solutions::advent2017::day23","",null,null],[13,"Literal","","",18,null],[13,"Register","","",18,null],[4,"Instruction","","The code it's running seems to be a variant of the kind you saw recently on that [tablet]. The general functionality seems very similar, but some of the instructions are different:",null,null],[13,"Set","","",19,null],[13,"Sub","","",19,null],[13,"Mul","","",19,null],[13,"Jnz","","",19,null],[5,"part1","","The coprocessor is currently set to some kind of debug mode, which allows for testing, but prevents it from doing any meaningful work.",null,null],[5,"is_prime","","",null,{"inputs":[{"name":"usize"}],"output":{"name":"bool"}}],[5,"part2","","Now, it's time to fix the problem.",null,null],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"generics":["instruction"],"name":"vec"}}],[11,"clone","","",18,{"inputs":[{"name":"self"}],"output":{"name":"value"}}],[11,"eq","","",18,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"ne","","",18,{"inputs":[{"name":"self"},{"name":"value"}],"output":{"name":"bool"}}],[11,"fmt","","",18,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",18,null],[11,"from_bytes","","",18,null],[11,"from","","",18,{"inputs":[{"name":"isize"}],"output":{"name":"self"}}],[11,"from","","",18,{"inputs":[{"name":"char"}],"output":{"name":"self"}}],[11,"clone","","",19,{"inputs":[{"name":"self"}],"output":{"name":"instruction"}}],[11,"eq","","",19,{"inputs":[{"name":"self"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"ne","","",19,{"inputs":[{"name":"self"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"fmt","","",19,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",19,null],[11,"from_bytes","","",19,null],[11,"list_from_bytes","","",19,null],[0,"day24","advent_solutions::advent2017","Day 24: Electromagnetic Moat",null,null],[3,"Component","advent_solutions::advent2017::day24","Each component has two ports, one on each end. The ports come in all different types, and only matching types can be connected. You take an inventory of the components by their port types (your puzzle input). Each port is identified by the number of pins it uses; more pins mean a stronger connection for your bridge. A `3/7` component, for example, has a type-`3` port on one side, and a type-`7` port on the other.",null,null],[5,"part1","","The strength of a bridge is the sum of the port types in each component. For example, if your bridge is made of components `0/3`, `3/7`, and `7/4`, your bridge has a strength of `0+3 + 3+7 + 7+4 = 24`.",null,{"inputs":[{"name":"vec"}],"output":{"name":"usize"}}],[5,"part2","","The bridge you've built isn't long enough; you can't jump the rest of the way.",null,{"inputs":[{"name":"vec"}],"output":{"name":"usize"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"generics":["component"],"name":"vec"}}],[11,"clone","","",20,{"inputs":[{"name":"self"}],"output":{"name":"component"}}],[11,"eq","","",20,{"inputs":[{"name":"self"},{"name":"component"}],"output":{"name":"bool"}}],[11,"ne","","",20,{"inputs":[{"name":"self"},{"name":"component"}],"output":{"name":"bool"}}],[11,"fmt","","",20,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",20,null],[0,"day25","advent_solutions::advent2017","Day 25: The Halting Problem",null,null],[3,"State","advent_solutions::advent2017::day25","",null,null],[3,"Blueprint","","You find the Turing machine blueprints (your puzzle input) on a tablet in a nearby pile of debris. Looking back up at the broken Turing machine above, you can start to identify its parts:",null,null],[5,"part1","","For example, suppose you found the following blueprint:",null,{"inputs":[{"name":"blueprint"}],"output":{"name":"usize"}}],[5,"part2","","The Turing machine, and soon the entire computer, springs back to life. A console glows dimly nearby, awaiting your command.",null,{"inputs":[{"name":"blueprint"}],"output":{"name":"str"}}],[5,"parse_input","","",null,{"inputs":[{"name":"str"}],"output":{"name":"blueprint"}}],[11,"clone","","",21,{"inputs":[{"name":"self"}],"output":{"name":"state"}}],[11,"eq","","",21,{"inputs":[{"name":"self"},{"name":"state"}],"output":{"name":"bool"}}],[11,"ne","","",21,{"inputs":[{"name":"self"},{"name":"state"}],"output":{"name":"bool"}}],[11,"fmt","","",21,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"hash","","",21,null],[11,"clone","","",22,{"inputs":[{"name":"self"}],"output":{"name":"blueprint"}}],[11,"eq","","",22,{"inputs":[{"name":"self"},{"name":"blueprint"}],"output":{"name":"bool"}}],[11,"ne","","",22,{"inputs":[{"name":"self"},{"name":"blueprint"}],"output":{"name":"bool"}}],[11,"fmt","","",22,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[14,"to_str","advent_solutions","",null,null],[14,"from_str","","",null,null],[14,"from_str_bytes","","",null,null],[14,"lines","","",null,null]],"paths":[[4,"Direction"],[3,"Downloader"],[3,"Bits"],[3,"Node"],[3,"Instruction"],[4,"Node"],[3,"Layer"],[3,"Firewall"],[3,"Grid"],[3,"Generator"],[4,"Move"],[4,"Value"],[4,"Instruction"],[3,"Particle"],[3,"Grid"],[4,"Node"],[3,"Memory"],[3,"Carrier"],[4,"Value"],[4,"Instruction"],[3,"Component"],[3,"State"],[3,"Blueprint"]]};
initSearch(searchIndex);
