# ev

## Description

Unix utility to compute the min, max, and expected value of a D&D-like dice roll.

## Compilation

`ev` can be built using Rust's `cargo` tool:

    $ cargo build --release

## Usage

`ev` can be used in two ways: it can read a list of rolls from the
positional command-line arguments and output their statistics, or, if
no positional arguments are given, it can read rolls from stdin.  By
default, the output is spread across multiple lines, in a way that is
easy to read for a human.

    $ ev 1d6 3d4+1
    1d6:
            min: 1
            max: 6
            ev : 3.5
    3d4+1:
            min: 4
            max: 13
            ev : 8.5

    $ echo 5d8-4 | ev
    5d8-4:
            min: 1
            max: 36
            ev : 18.5

If the `ev` command is to be used as part of a Unix pipe-line, the
`-s` flag is helpful: the output for each dice roll will be on a
single line, making integration with tools like `awk` or `sed`
simpler.

    $ ev -s 1d6 3d4+1
    1d6 1 6 3.5
    3d4+1 4 13 8.5

    $ ev -s 2d6 1d8+2 3d4+1 | column -t
    2d6    2  12  7
    1d8+2  3  10  6.5
    3d4+1  4  13  8.5
