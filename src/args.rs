use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "singlenum", author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    /// File containing puzzle
    #[arg(short, long = "file", required = true)]
    pub file: Utf8PathBuf,

    /// Attempts before giving up
    #[arg(global = true, long, default_value_t = 500)]
    pub attempts: i32,
}
