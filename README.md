# singlenum

Singlenum is a Sudoku solver program written in Rust. It uses my own way on how
to solve a Sudoku, just with somewhat better memory ッ

## Why

Mostly for the purpose of learning Rust, but I also find Sudoku quite fun.

## Example

There are some example puzzles in the repository, see puzzles. If you want to
run on another puzzle create a file (Json) with a single list of squares [0-9]
from top-left to bottom-right.

## Improvements

List of improvements, both to performance, but also idiomatic Rust.

### Performance

Run multiple secure updates in row, see `_engine_*_one_left` methods. This
should be possible with an `_update_square_potentials` in-between.

### Measure performance

Have a set of Sudokus that can be used to measure performance. Use [Hyperfine](https://github.com/sharkdp/hyperfine)
to measure. What about Perf?

Should be possible to use in GitHub workflow. Maybe need to take a baseline and
then compare against that baseline (unsure if we get same HW resources on node).

### TODOs

- Merge `_validate_*` methods. They look astonishing similar!
- Add more tests
- [Use borrowed types for arguments](https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html)
