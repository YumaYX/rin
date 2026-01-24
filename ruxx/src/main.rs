extern crate ys1r;
mod markdown;
use std::fs;
use std::process;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    if let Err(e) = rt.block_on(run()) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ruby_codes = fs::read_to_string("ruby.rb").expect("failed to read file");
    let output = ruby2rust(&ruby_codes).await.unwrap();
    println!("{}", &output);
    fs::write("rust.rs", output).expect("failed to write file");
    Ok(())
}

async fn ruby2rust(ruby_codes: &str) -> Option<String> {
    // make prompt
    let prompt = compose_convert_prompt(ruby_codes);

    // ollama
    let response = ys1r::ollama::request_ollama(&prompt, Some("gemma3"), None).await;

    // extra code block
    let mut rust_codes = markdown::extract_code_block_or_original(&response);
    while rust_codes.ends_with('\n') || rust_codes.ends_with('\r') {
        rust_codes.pop();
    }
    Some(rust_codes)
}

fn compose_convert_prompt(ruby_codes: &str) -> String {
    let instruction = r#"
    - convert Ruby script to Rust codes.
    - Output is Rust.
    - Just answer.
    - use code block.
    ---
    ```ruby
    "#;
    format!("{}{}\n```", instruction, ruby_codes)
}
