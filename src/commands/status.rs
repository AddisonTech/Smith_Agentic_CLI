use colored::Colorize;
use crate::client::ApiClient;

pub async fn run(client: &ApiClient) -> Result<(), String> {
    let resp = client.status().await?;

    let sa_str    = "online".green().bold().to_string();
    let oll_str   = if resp.ollama {
        "online".green().bold().to_string()
    } else {
        "offline  (run: ollama serve)".red().bold().to_string()
    };

    println!("  {:<18} {}", "Smith_Agentic", sa_str);
    println!("  {:<18} {}", "Ollama", oll_str);

    Ok(())
}
