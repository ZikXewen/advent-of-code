# Advent of Code

This is my workstation for Advent of Code.

## How I use this

For each question, I create a new binary in `src/bin`.

The main function I write is called `compute`. It takes a `String` as input and returns any type that implements `Display`.[^1]

The `make_main` macro is then used to create the actual main function (and a test function, if prompted with an argument). They read input from `input.txt` and `test.txt` respectively.

I can then run or test with Rust Analyzer's VSCode lens.

## Pulling

For further automation, I can configure some variables in `config.json` then use `pull` binary to directly pull the input from the website into `input.txt`.

Depending on your configuration, it can also automatically creates a new binary for part 1 of the day and open them with VSCode.[^2]

[^1]: I considered using debug formatting, but having clean output feels better to copy-paste into the website.
[^2]: I might add support for other editors later.
