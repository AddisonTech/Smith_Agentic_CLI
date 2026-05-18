use colored::Colorize;

pub fn run() -> Result<(), String> {
    println!("  Run history is not available via the CLI.");
    println!(
        "  View it in the Smith_Agentic UI at {}",
        "http://localhost:8765".cyan()
    );
    Ok(())
}
