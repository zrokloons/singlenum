# singlenum

Singlenum is a Sudoku solver program written in Rust. It uses my own way on how
to solve a Sudoku, just with somewhat better memory ッ

<!-- markdownlint-disable no-inline-html -->
<!-- markdownlint-disable no-alt-text -->
<p align="center">
  <img width="600" src="singlenum.svg">
</p>
<!-- markdownlint-enable no-inline-html -->
<!-- markdownlint-enable no-alt-text -->

## Why

Mostly for the purpose of learning Rust, but I also find Sudoku quite fun.

## Example

There are some example puzzles in the repository, see puzzles. If you want to
run on another puzzle create a JSON file with a single array of exactly 81
integers from 0 to 9, from top-left to bottom-right. Zero means an empty cell.
Both front ends reject malformed input, out-of-range values, and duplicate
nonzero clues in a row, column, or 3×3 box. Valid clues do not guarantee that a
puzzle has a solution or that its solution is unique.

### Command line

```sh
cargo run -- --file puzzles/cat/medium/puzzle_aa.json
cargo run -- --path puzzles --novisual
cargo run -- --file puzzles/cat/medium/puzzle_aa.json --just-draw
```

The CLI remains the default executable and does not build GUI dependencies.
`--attempts` sets a positive iteration limit (default: 500).

### Desktop GUI (optional)

```sh
cargo run --features gui --bin singlenum-gui
```

1. Choose **Open puzzle…** to browse for an existing JSON file, or enter a path
   and choose **Load** (relative paths use the working directory).
2. Choose **Solve**. Solving runs on a worker thread while the window remains
   responsive. Loading, reset, and attempt-limit changes are disabled during
   the solve; there is no cancellation button in this milestone.
3. The final solution distinguishes shaded original clues from colored solved
   digits. **Show original** compares it with the loaded puzzle. **Reset**
   discards the solution and restores the original clues without rereading
   the file.

The GUI reports loading errors, solver errors, and attempt-limit exhaustion.
Failed loads preserve the previous puzzle; an unsuccessful solve leaves the
original board visible rather than displaying a partial/guessed board. A limit
or solver error is not proof that no solution exists. Increase the limit to
retry. This milestone displays final solutions only: no editing, hints, step
playback, export, or uniqueness checking.

The optional GUI uses egui/eframe's OpenGL renderer. Building it requires a
current stable Rust toolchain and native desktop development libraries; on
Linux these include X11/Wayland and OpenGL libraries. X11 runtime libraries
such as `libXcursor.so.1` must also be installed to open the window. The Linux
file picker uses an XDG desktop portal (a running portal service and
file-chooser backend are needed); the path field is available without a
picker. Windows and macOS use their native file pickers. A graphical
session is needed to run the GUI.

### Development checks

```sh
cargo fmt --check
cargo test
cargo test --features gui
cargo check --features gui --bin singlenum-gui
cargo clippy --all-targets --all-features
```

The library's `solver` module owns validated puzzle loading and the shared
solve loop. The existing engine and guessing strategy are unchanged; the GUI
does not introduce another solving algorithm.

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
