use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// ===============================
// Process file line by line with trigger
// ===============================
//
// Logic:
// - When a line contains the trigger, it starts a new "parent".
// - If the parent line contains the keyword, print it and subsequent child lines.
// - Stop printing when the next parent is found (trigger line).
// - Lines not starting with a trigger are treated as child lines.
// - Memory usage is minimal because we print immediately and don't store lines.
//

fn process_file<P: AsRef<Path>>(path: P, trigger: &str, keyword: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut printing = false;

    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(trigger) {
            // New parent line
            if line.contains(keyword) {
                // Start printing this parent and its children
                println!("{}", line);
                printing = true;
            } else {
                // This parent doesn't match keyword, stop printing
                printing = false;
            }
        } else if printing {
            // Child lines of a matching parent
            println!("{}", line);
        }
        // else: skip non-parent lines when not printing
    }

    Ok(())
}

// ===============================
// main
// ===============================

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: pac <file> <trigger-text> <keyword>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let trigger = &args[2];
    let keyword = &args[3];

    process_file(filename, trigger, keyword)?;

    Ok(())
}
