// This file is part of RustLog, a simple CLI log filtering tool.
// It reads log files and filters lines based on a keyword.
fn main() -> anyhow::Result<()> {
    let args = rustlog::args::parse_args();
    let lines = rustlog::reader::read_lines(&args.file_path)?;
    let filtered = rustlog::filter::filter_lines(lines, &args.keyword);

    for line in filtered {
        println!("{}", line);
    }

    Ok(())
}
