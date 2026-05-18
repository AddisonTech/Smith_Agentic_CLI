use colored::Colorize;

pub fn print_url(url: &str) {
    println!("{}", format!("  → {}", url).dimmed());
    println!();
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1_024 {
        format!("{} B", bytes)
    } else if bytes < 1_048_576 {
        format!("{:.1} KB", bytes as f64 / 1_024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    }
}

pub fn colorize_line(line: &str) -> String {
    if line.contains("ERROR") || line.contains("Error:") || line.starts_with("[ERROR]") {
        line.red().to_string()
    } else if line.contains("WARNING") || line.contains("Warning") {
        line.yellow().to_string()
    } else if line.starts_with("[SmithAgentic]") {
        line.cyan().to_string()
    } else if line.starts_with('[') {
        line.bright_cyan().dimmed().to_string()
    } else {
        line.to_string()
    }
}

pub fn separator(width: usize) -> String {
    "─".repeat(width)
}
