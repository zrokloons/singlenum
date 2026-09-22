use std::process::Command;

fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_singlenum"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn existing_cli_modes_are_preserved() {
    let file = "puzzles/cat/medium/puzzle_aa.json";
    let output = cli(&["--file", file, "--just-draw"]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with(file));
    assert!(stdout.contains('╔'));
    assert!(!stdout.contains("Puzzle solved"));

    let output = cli(&["--file", file, "--attempts", "1", "--novisual"]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Unable to solve puzzle [iterations: 1,"));
    assert!(!stdout.contains('╔'));

    let output = cli(&["--path", "puzzles/cat/medium", "--just-draw", "--novisual"]);
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("puzzle_aa.json"));
    assert!(stdout.contains("puzzle_ab.json"));
    assert!(!stdout.contains('╔'));
}

#[test]
fn nonpositive_attempts_fail_instead_of_running_without_a_limit() {
    let output = cli(&[
        "--file",
        "puzzles/cat/medium/puzzle_aa.json",
        "--attempts",
        "0",
        "--novisual",
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("Attempts must be greater than zero"));
}
