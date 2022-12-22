# Advent of Code

This is my workstation for Advent of Code.

## How I use this

If you are looking at a code with `make_main!` macro, that is deprecated. See below for [How I used to use this](#how-i-used-to-use-this-deprecated).

TL;DR: Check out [examples](examples). That should almost clarify everything.

For an actual competition, I can use the [quick setup](#pulling) for a headstart. That requires some daily configuration though.

For casual use, I can just create a new binary in `src/bin` and use the `aoc::main` macro to ease the work of writing the main and test functions. Check out [its documentation](src/lib.rs#41) for more information.

After confirming the input and the test case, I can then run or test with Rust Analyzer's VSCode lens.

## Pulling

For further automation, I can configure some variables in `config.json` then use `pull` binary to directly pull the input from the website into `input.txt`. Note that the test input is not pulled.

Depending on your configuration, it can also automatically creates a new binary for part 1 of the day and open them with VSCode.[^1]

## How I used to use this (Deprecated)

For each question, I create a new binary in `src/bin`.

The main function I write is called `compute`. It takes a `String` as input and returns any type that implements `Display`.[^2]

The `make_main` macro is then used to create the actual main function (and a test function, if prompted with an argument). They read input from `input.txt` and `test.txt` respectively.

I can then run or test with Rust Analyzer's VSCode lens.

[^1]: I might add support for other editors later.
[^2]: I considered using debug formatting, but having clean output feels better to copy-paste into the website.
