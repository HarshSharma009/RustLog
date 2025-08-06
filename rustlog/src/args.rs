use clap::Parser;

/// Simple log reader that filters by keyword.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path to the log file
    pub file_path: String,


    /// Keyword to filter log lines
    pub keyword: String,

    ///Enable rela-time log tailing
    #[arg(short, long)]
    pub tail: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
