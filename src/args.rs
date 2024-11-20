use camino::Utf8PathBuf;
use clap::{Args, Parser};

#[derive(Debug, Parser, PartialEq)]
#[command(name = "singlenum", author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Arguments {
    /// Attempts before giving up
    #[arg(global = true, long, default_value_t = 500)]
    pub attempts: i32,

    #[clap(flatten)]
    pub group: RequiredOption,
}

#[derive(Debug, Args, PartialEq)]
#[group(required = true, multiple = false)]
pub struct RequiredOption {
    /// File containing puzzle
    #[arg(short, long = "file")]
    pub file: Utf8PathBuf,
    ///// Path containing puzzles
    //#[arg(short, long = "path")]
    //pub path: Utf8PathBuf,
}
