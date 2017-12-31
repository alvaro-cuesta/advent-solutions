# Advent of Code Solutions

Solutions for [Advent of Code](http://adventofcode.com/about).

## Running

- Running for your input (requires `SESSION` environment variable or file):

    ```sh
    $ cargo run
    ```

- Filtering by days:

    ```sh
    $ cargo run dayNN dayMM ...
    ```

- Run all tests:

    ```sh
    $ cargo test
    ```

- Or only some:

    ```sh
    $ cargo test dayNN       # NN = 01, 02, ..., 25
    $ cargo test YYYY        # YYYY = Year
    $ cargo test YYYY::dayNN
    ```

## TODO

- 2017
    - Solve 25
    - Document 24, 25
    - I'm not sure just comparing by velocity solves day 20 (might be accelerating against v)

- main.rs
    - If any job panics, it's not handled properly (should be like cargo test)
    - Thread each part separately
    - Async download (limit # to avoid many requests)

- Add panics to docs
- Document advent lib
- Older Advent of Code
- Ensure all doccomments are on pub types
- Move test_inputs to src year?
- Do not return result strings, but a result type
