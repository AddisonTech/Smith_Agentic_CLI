use colored::Colorize;
use crate::client::ApiClient;

pub async fn run(client: &ApiClient, run_id: &str) -> Result<(), String> {
    let resp = client.cancel_run(run_id).await?;

    println!("  cancellation sent to run {}", run_id.cyan());
    println!("  status: {}", resp.status.yellow());

    Ok(())
}
