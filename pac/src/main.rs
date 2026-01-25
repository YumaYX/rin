use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Process a file and print parent lines containing a keyword and their children.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the input file
    file: String,

    /// Trigger text that starts a new parent line
    trigger: String,

    /// Keyword to match in parent lines
    keyword: String,
}

fn process_file<P: AsRef<Path>>(path: P, trigger: &str, keyword: &str) -> io::Result<()> {
    let reader = io::BufReader::new(File::open(path)?);
    let mut printing = false;

    for line_result in reader.lines() {
        let line = line_result?;
        printing = if line.contains(trigger) {
            if line.contains(keyword) {
                println!("{}", line);
                true
            } else {
                false
            }
        } else if printing {
            println!("{}", line);
            true
        } else {
            false
        };
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    process_file(&args.file, &args.trigger, &args.keyword)
}
