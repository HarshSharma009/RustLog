use clap::Parser;

/// Simple log reader that filters by keyword.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to the log file
    pub file_path: String,


    /// Keyword to filter log lines
    pub keyword: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
// This function parses command line arguments using Clap and returns an Args struct.
// It expects a file path and a keyword for filtering log lines.
// The Args struct is defined with two fields: file_path and keyword, both of which are required.
// The `parse_args` function uses the Clap library to handle command line argument parsing.
// The Args struct is annotated with `Parser` to enable automatic parsing of command line arguments.
// The `clap` crate is used for command line argument parsing, and it provides a convenient way to define and parse command line options.
// The `Args` struct includes metadata such as author, version, and about information for the command line tool.
// The `parse_args` function is called in the main function to retrieve the command line arguments provided by the user.