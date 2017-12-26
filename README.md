# Advent of Code 2017 Solutions

Solutions for [Advent of Code 2017](http://adventofcode.com/2017).

## Running

- Running for your input (requires `SESSION` environment variable or file):

    ```sh
    $ cargo run
    ```

- Run all tests:

    ```sh
    $ cargo test
    ```

- Or only some:

    ```sh
    $ cargo test dayNN::       # NN = 01, 02, ..., 25
    $ cargo test YYYY::        # YYYY = Year
    $ cargo test YYYY::dayNN::
    ```

## TODO

- Make main() execute only certain
- Make main() share SESSION
- Add panics to docs
- Document advent lib
- Separate tests in part1 and part2
- Solve 20, 21, 23, 24, 25
