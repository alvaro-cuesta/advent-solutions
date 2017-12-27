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

- Add panics to docs
- Document advent lib
- Solve 20, 21, 23, 24, 25
- Older Advent of Code
- Use lines! in list_from_bytes (20, 21)
- Ensure all doccomments are on pub types
- Move test_inputs to src year?
- Thread each part separately
- Async download (limit # to avoid many requests)
- Reuse client in download (keep-alive?)
- Do not return result strings, but a result type
- If any job panics, it's not handled properly (should be like cargo test)
