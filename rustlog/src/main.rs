mod args;
mod reader;
mod filter;

use anyhow::Result;
/// Main function to read a log file and filter its lines by a keyword.

fn main() -> Result<()> {

    let args = args::parse_args();
    let lines = reader::read_lines(&args.file_path)?;
    let filtered_lines = filter::filter_lines(lines, &args.keyword);
    for line in filtered_lines {
        println!("{}", line);
    }
    Ok(())
}
