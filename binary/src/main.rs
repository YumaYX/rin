use clap::Parser;

/// Display a number in decimal and binary, or all numbers 0-255 if no argument is given
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number to display (0-255)
    number: Option<u8>,
}

fn main() {
    let args = Args::parse();

    match args.number {
        Some(n) => {
            // 引数あり: 0～255 のみ成功時に表示
            println!("{n}: {n:08b}");
        }
        None => {
            // 引数なし: 0～255 をすべて表示
            for i in 0..=255 {
                println!("{i}: {i:08b}");
            }
        }
    }
}
