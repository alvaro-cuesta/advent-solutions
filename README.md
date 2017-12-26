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
    $ cargo test dayN::       # N = 1, 2, ..., 25
    $ cargo test YYYY::       # YYYY = Year
    $ cargo test YYYY::dayN::
    ```

## TODO

- Make main() execute only certain
- Make main() share SESSION
- Add panics to docs
- Document advent lib
- Something is broken in run
- Separate tests in part1 and part2
