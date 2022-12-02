# Advent of Code

This is my workstation for Advent of Code.

## How I use this

For each question, I create a new binary in `src/bin`.

The main function I write is called `compute`. It takes a `String` as input and returns any type that implements `Display`.

    Note: I considered using debug formatting, but having clean output feels better to copy-paste into the website.

The `make_main` macro is then used to create the actual main function (and a test function, if prompted with an argument). They read input from `input.txt` and `test.txt` respectively.

I can then run the binary with `cargo run --bin <name>` and run the test with `cargo test --bin <name>`.
