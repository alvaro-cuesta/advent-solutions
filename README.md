# 🎄 Advent of Code Solutions ([docs](https://alvaro-cuesta.github.io/advent-solutions/advent_solutions/index.html))

My solutions for [Advent of Code](http://adventofcode.com/about) in Rust.

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

- View docs:

    ```sh
    $ cargo doc --open
    ```

- Upload docs to GitHub:

    ```sh
    $ git worktree add gh-pages gh-pages  # if `gh-pages` worktree is missing
    $ rm -rf target/doc
    $ cargo doc --no-deps
    $ cp -rf target/doc/* gh-pages/
    $ cd gh-pages
    $ git add .
    $ git commit -m "Update gh-pages"
    $ git push
    ```

## TODO

- 2017-day20-part1: I'm not sure just comparing by velocity solves it (might
  be two same-length accelerations, one of them accelerating against
  same-length v)

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
