use clap::Parser;
use std::{process::Command, thread, time::Duration};

#[derive(Parser, Debug)]
#[command(trailing_var_arg = true)]
struct Args {
    /// Number of times to run the command
    #[arg(short = 'n', long = "num", default_value_t = 3)]
    n: u32,

    /// Interval between runs (seconds)
    #[arg(short = 'i', long = "interval", default_value_t = 1.0)]
    interval: f64,

    /// Command to execute (all remaining arguments)
    #[arg(required = true)]
    cmd: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let program = &args.cmd[0];
    let program_args = &args.cmd[1..];

    for i in 0..args.n {
        println!("--- run {} ---", i + 1);

        let status = Command::new(program)
            .args(program_args)
            .status()
            .expect("failed to execute command");

        if !status.success() {
            eprintln!("command failed with status: {}", status);
            break;
        }

        // Do not sleep after the last run
        if i + 1 < args.n && args.interval > 0.0 {
            thread::sleep(Duration::from_secs_f64(args.interval));
        }
    }
}
