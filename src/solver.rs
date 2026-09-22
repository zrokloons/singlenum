//! Validated puzzle loading and the solver orchestration shared by both front ends.
use crate::components::table::core::Table;
use crate::enums::Progress;
use anyhow::{ensure, Context, Result};
use std::{fs::File, io::Read, path::Path};

/// A row-major Sudoku board. Zero represents an empty square.
/// Construction validates the shape, value range, and consistency of clues,
/// but does not prove that the puzzle has a solution or a unique solution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Puzzle {
    values: [usize; 81],
}

impl Puzzle {
    pub fn new(values: Vec<usize>) -> Result<Self> {
        ensure!(
            values.len() == 81,
            "Expected 81 values, got {}",
            values.len()
        );
        let mut rows = [0u16; 9];
        let mut columns = [0u16; 9];
        let mut boxes = [0u16; 9];
        for (index, &value) in values.iter().enumerate() {
            let row = index / 9;
            let column = index % 9;
            ensure!(
                value <= 9,
                "Value at row {}, column {} must be in 0..=9",
                row + 1,
                column + 1
            );
            if value == 0 {
                continue;
            }
            let abox = row / 3 * 3 + column / 3;
            let bit = 1 << value;
            ensure!(
                (rows[row] | columns[column] | boxes[abox]) & bit == 0,
                "Conflicting clue {value} at row {}, column {}",
                row + 1,
                column + 1
            );
            rows[row] |= bit;
            columns[column] |= bit;
            boxes[abox] |= bit;
        }
        Ok(Self {
            values: values.try_into().expect("length checked above"),
        })
    }

    pub fn from_reader(reader: impl Read) -> Result<Self> {
        let values = serde_json::from_reader(reader)
            .context("Expected a JSON array of 81 integers in 0..=9")?;
        Self::new(values)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path).with_context(|| format!("Cannot open {}", path.display()))?;
        Self::from_reader(std::io::BufReader::new(file))
            .with_context(|| format!("Invalid puzzle {}", path.display()))
    }

    pub fn values(&self) -> &[usize; 81] {
        &self.values
    }

    pub fn table(&self, attempts: i32) -> Table {
        Table::new(self.values.to_vec(), attempts)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SolveStatus {
    Solved,
    LimitReached,
}

#[derive(Debug)]
pub struct SolveResult {
    pub table: Table,
    pub status: SolveStatus,
    /// Existing engine statistics, formatted identically to the CLI.
    pub message: String,
}

/// Runs the existing engine with a positive iteration budget.
/// A limit or engine error is not proof that a puzzle is unsatisfiable.
pub fn solve(puzzle: &Puzzle, attempts: i32) -> Result<SolveResult> {
    ensure!(attempts > 0, "Attempts must be greater than zero");
    let mut table = puzzle.table(attempts);
    loop {
        match table.complete() {
            Progress::Solved(message) => {
                // The engine checks fullness; also check Sudoku constraints before
                // exposing a completed board as a solution.
                Puzzle::new(table.squares.iter().map(|s| s.value).collect())
                    .context("Solver produced an invalid solution")?;
                ensure!(
                    puzzle
                        .values
                        .iter()
                        .zip(&table.squares)
                        .all(|(&given, square)| given == 0 || given == square.value),
                    "Solver changed an original clue"
                );
                return Ok(SolveResult {
                    table,
                    status: SolveStatus::Solved,
                    message,
                });
            }
            Progress::LimitReached(message) => {
                return Ok(SolveResult {
                    table,
                    status: SolveStatus::LimitReached,
                    message,
                });
            }
            Progress::InProgress(iteration) => log::debug!("[iteration] {iteration}"),
        }
        table.update()?;
        if table.engine()? {
            continue;
        }
        if !table.qualified_guess()? && !table.incompetent_guess()? {
            table.snapshot_rollback()?;
        }
    }
}
