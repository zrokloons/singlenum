use singlenum::solver::{solve, Puzzle, SolveStatus};

const SOLVED: [usize; 81] = [
    8, 5, 9, 6, 1, 2, 4, 3, 7, 7, 2, 3, 8, 5, 4, 1, 6, 9, 1, 6, 4, 3, 7, 9, 5, 2, 8, 9, 8, 6, 1, 4,
    7, 3, 5, 2, 3, 7, 5, 2, 6, 8, 9, 1, 4, 2, 4, 1, 5, 9, 3, 7, 8, 6, 4, 3, 2, 9, 8, 1, 6, 7, 5, 6,
    1, 7, 4, 2, 5, 8, 9, 3, 5, 9, 8, 7, 3, 6, 2, 4, 1,
];

#[test]
fn validates_length_and_range() {
    for length in [0, 80, 82] {
        assert!(Puzzle::new(vec![0; length])
            .unwrap_err()
            .to_string()
            .contains("81"));
    }
    let mut values = vec![0; 81];
    values[80] = 10;
    assert!(Puzzle::new(values)
        .unwrap_err()
        .to_string()
        .contains("0..=9"));
    assert!(Puzzle::new(vec![0; 81]).is_ok());
    assert!(Puzzle::new(SOLVED.to_vec()).is_ok());
}

#[test]
fn rejects_conflicts_in_each_kind_of_unit() {
    for second in [8, 72, 10] {
        let mut values = vec![0; 81];
        values[0] = 3;
        values[second] = 3;
        assert!(Puzzle::new(values)
            .unwrap_err()
            .to_string()
            .contains("Conflicting clue"));
    }
    let mut values = vec![0; 81];
    values[0] = 3;
    values[40] = 3;
    assert!(Puzzle::new(values).is_ok());
}

#[test]
fn rejects_malformed_or_non_integer_json() {
    for input in ["{", "{}", "null", "[0, -1]", "[1.5]", "[\"1\"]"] {
        assert!(Puzzle::from_reader(input.as_bytes()).is_err());
    }
    let json = serde_json::to_vec(&SOLVED.to_vec()).unwrap();
    assert_eq!(Puzzle::from_reader(&json[..]).unwrap().values(), &SOLVED);
    assert!(Puzzle::load("missing-puzzle.json").is_err());
}

#[test]
fn solves_and_preserves_clues() {
    let mut values = SOLVED.to_vec();
    values[0] = 0;
    values[18] = 0;
    let puzzle = Puzzle::new(values).unwrap();
    let result = solve(&puzzle, 500).unwrap();
    assert_eq!(result.status, SolveStatus::Solved);
    assert_eq!(
        result
            .table
            .squares
            .iter()
            .map(|s| s.value)
            .collect::<Vec<_>>(),
        SOLVED
    );
    assert_eq!(puzzle.values()[0], 0);
    assert_eq!(
        solve(&Puzzle::new(SOLVED.to_vec()).unwrap(), 1)
            .unwrap()
            .status,
        SolveStatus::Solved
    );
}

#[test]
fn respects_limit_and_rejects_nonpositive_budgets() {
    let puzzle = Puzzle::new(vec![0; 81]).unwrap();
    assert_eq!(solve(&puzzle, 1).unwrap().status, SolveStatus::LimitReached);
    assert!(solve(&puzzle, 0).is_err());
    assert!(solve(&puzzle, -1).is_err());
    let mut table = puzzle.table(1);
    assert!(table.snapshot_rollback().is_err());
}

#[test]
fn bundled_puzzles_are_valid() {
    for entry in walkdir::WalkDir::new("puzzles") {
        let entry = entry.unwrap();
        if entry
            .path()
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            Puzzle::load(entry.path()).unwrap();
        }
    }
}
