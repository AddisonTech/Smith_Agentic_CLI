use colored::Colorize;
use crate::client::{ApiClient, RunRequest};

pub async fn run(
    client: &ApiClient,
    crew:   &str,
    goal:   &str,
    model:  Option<&str>,
    chain:  bool,
) -> Result<(), String> {
    let req = RunRequest {
        goal:  goal.to_string(),
        crew:  crew.to_string(),
        model: model.map(|s| s.to_string()),
        chain,
        hitl:  false,
    };

    let resp = client.start_run(&req).await?;

    println!("  {:<10} {}", "run_id".dimmed(), resp.run_id.cyan().bold());
    println!("  {:<10} {}", "crew".dimmed(), crew);
    if let Some(m) = model {
        println!("  {:<10} {}", "model".dimmed(), m);
    }
    if chain {
        println!("  {:<10} safety → ops", "chain".dimmed());
    }
    println!();
    println!("  Follow output with:");
    println!("    {}", format!("smith watch {}", resp.run_id).cyan());

    Ok(())
}
