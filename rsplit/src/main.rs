use clap::Parser;
use std::fs::{self, File};
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

/// splits a given file into multiple smaller files, each containing a specified number of lines
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// line
    #[arg(short, long, default_value_t = 10)]
    line: usize,

    /// file name
    #[arg(short)]
    file: String,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let path = "./rsplit";
    fs::create_dir_all(path)?;

    let mut file_count = 1;
    let mut current_line_count = 0;
    let mut output_file: Option<BufWriter<File>> = None;

    if let Ok(lines) = read_lines(args.file) {
        for line in lines {
            if current_line_count == 0 {
                let output_file_name = format!("{}/rf.{}", &path, file_count);
                let file = File::create(&output_file_name)?;
                output_file = Some(BufWriter::new(file));
                file_count += 1;
            }

            if let Ok(ip) = line {
                if let Some(writer) = &mut output_file {
                    writeln!(writer, "{}", ip)?;
                }
            }

            current_line_count += 1;

            if current_line_count >= args.line {
                current_line_count = 0;
                output_file = None;
            }
        }
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
