/*
 *                        COLUMNS
 *
 *           0   1   2   3   4   5   6   7   8                BOXES
 *  LINES
 *         ╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗
 *    0    ║ 0 │ 1 │ 2 ║ 3 │ 4 │ 5 ║ 6 │ 7 │ 8 ║     ╔═════╗ ╔═════╗ ╔═════╗
 *         ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢     ║     ║ ║     ║ ║     ║
 *    1    ║ 9 │ 10│ 11║ 12│ 13│ 14║ 15│ 16│ 17║     ║  0  ║ ║  1  ║ ║  2  ║
 *         ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢     ║     ║ ║     ║ ║     ║
 *    2    ║ 18│ 19│ 20║ 21│ 22│ 23║ 24│ 25│ 26║     ╚═════╝ ╚═════╝ ╚═════╝
 *         ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 *    3    ║ 27│ 28│ 29║ 30│ 31│ 32║ 33│ 34│ 35║     ╔═════╗ ╔═════╗ ╔═════╗
 *         ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢     ║     ║ ║     ║ ║     ║
 *    4    ║ 36│ 37│ 38║ 39│ 40│ 41║ 42│ 43│ 44║     ║  3  ║ ║  4  ║ ║  5  ║
 *         ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢     ║     ║ ║     ║ ║     ║
 *    5    ║ 45│ 46│ 47║ 48│ 49│ 50║ 51│ 52│ 53║     ╚═════╝ ╚═════╝ ╚═════╝
 *         ╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣
 *    6    ║ 54│ 55│ 56║ 57│ 58│ 59║ 60│ 61│ 62║     ╔═════╗ ╔═════╗ ╔═════╗
 *         ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢     ║     ║ ║     ║ ║     ║
 *    7    ║ 63│ 64│ 65║ 66│ 67│ 68║ 69│ 70│ 71║     ║  6  ║ ║  7  ║ ║  8  ║
 *         ╟───┼───┼───╫───┼───┼───╫───┼───┼───╢     ║     ║ ║     ║ ║     ║
 *    8    ║ 72│ 73│ 74║ 75│ 76│ 77║ 78│ 79│ 80║     ╚═════╝ ╚═════╝ ╚═════╝
 *         ╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝
 *
 */

use anyhow::Result as AnyhowResult;
use camino::Utf8PathBuf;
use clap::Parser;
use singlenum::args::Arguments;
use singlenum::components::table;
use singlenum::components::table::draw::draw_table;
use singlenum::enums::Progress;
use std::fs::File;
use std::io::BufReader;
use walkdir::WalkDir;

fn main() -> AnyhowResult<()> {
    env_logger::init();
    let args: Arguments = Arguments::parse();

    if let Some(file) = args.group.file {
        if !file.exists() {
            println!("File: {file:?} does not exist!");
        } else {
            runner(file, args.attempts, args.novisual, args.just_draw)?;
        }
    } else if let Some(path) = args.group.path {
        // Find all puzzle files!
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if let Some(extension) = entry.path().extension() {
                if extension.to_str().unwrap() == "json" {
                    let a: Utf8PathBuf = Utf8PathBuf::from(entry.path().to_str().unwrap());
                    runner(a, args.attempts, args.novisual, args.just_draw)?;
                }
            }
        }
    }
    Ok(())
}

fn runner(
    puzzle: Utf8PathBuf,
    attempts: i32,
    novisual: bool,
    just_draw: bool,
) -> AnyhowResult<bool> {
    let file = File::open(&puzzle)?;
    let reader = BufReader::new(file);
    let layout: Vec<usize> = serde_json::from_reader(reader)?;

    let mut table = table::core::Table::new(layout, attempts);

    println!("{}", &puzzle);
    draw_table(&table, novisual);

    if just_draw {
        return Ok(true);
    }

    loop {
        match table.complete() {
            Progress::Solved(msg) => {
                draw_table(&table, novisual);
                println!("Puzzle solved {msg}");
                return Ok(true);
            }
            Progress::LimitReached(msg) => {
                draw_table(&table, novisual);
                println!("Unable to solve puzzle {msg}");
                return Ok(false);
            }
            Progress::InProgress(iteration) => log::debug!("[iteration] {iteration}"),
        };

        // Update line, column, box, and finally squares. Then run Engine to set squares
        table.update()?;
        if table.engine()? {
            continue;
        }

        // Guess, first a qualified guess, then a somewhat less qualified (incompetent)
        if !table.qualified_guess()? && !table.incompetent_guess()? {
            table.snapshot_rollback()?
        }
    }
}
