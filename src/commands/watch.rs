use std::time::Duration;
use colored::Colorize;
use crate::client::ApiClient;
use crate::display::colorize_line;

pub async fn run(client: &ApiClient, run_id: &str) -> Result<(), String> {
    println!("  {} {}", "watching".dimmed(), run_id.dimmed());
    println!();

    let mut seen = 0usize;

    loop {
        let status = client.get_run(run_id).await?;

        // Print any new output lines
        for line in &status.output[seen..] {
            println!("{}", colorize_line(line));
        }
        seen = status.output.len();

        match status.status.as_str() {
            "completed" => {
                println!();
                println!("  {} {}", "status".dimmed(), "completed".green().bold());
                if !status.files.is_empty() {
                    println!("  {} {}", "files".dimmed(), status.files.join("  ").dimmed());
                }
                break;
            }
            "error" => {
                println!();
                println!("  {} {}", "status".dimmed(), "error".red().bold());
                break;
            }
            "cancelled" => {
                println!();
                println!("  {} {}", "status".dimmed(), "cancelled".yellow().bold());
                break;
            }
            _ => {
                // starting or running - wait and poll again
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }

    Ok(())
}
