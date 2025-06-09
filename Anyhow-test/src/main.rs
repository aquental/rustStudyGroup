use anyhow::{Context, Result};

fn process_data(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path).context("Failed to read file")?;
    if content.is_empty() {
        anyhow::bail!("File is empty");
    }
    Ok(content)
}

fn main() -> Result<()> {
    let result = process_data("data.txt")?;
    println!("Content: {}", result);
    Ok(())
}
