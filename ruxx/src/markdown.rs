fn extract_code_block(input: &str) -> Option<String> {
    let start = input.find("```")?;
    let after_start = &input[start + 3..];

    // Skip language identifier if present
    let content_start = after_start.find('\n').map_or(0, |pos| pos + 1);
    let rest = &after_start[content_start..];

    let end = rest.find("```")?;
    Some(rest[..end].to_string())
}

pub fn extract_code_block_or_original(input: &str) -> String {
    extract_code_block(input).unwrap_or_else(|| input.to_string())
}
