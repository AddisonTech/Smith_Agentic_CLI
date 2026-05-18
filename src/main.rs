mod client;
mod commands;
mod display;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name    = "smith",
    about   = "CLI for Smith_Agentic — run agent crews from your terminal",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check whether Smith_Agentic and Ollama are online
    Status,

    /// List available crews and their default models
    Crews,

    /// Start a new crew run
    Run {
        /// Crew to use: default, plc, react, vision, safety, ops
        #[arg(long, default_value = "default")]
        crew: String,

        /// Goal for the crew to accomplish
        #[arg(long)]
        goal: String,

        /// Override the default Ollama model for this run
        #[arg(long)]
        model: Option<String>,

        /// Auto-chain safety and ops crews after the primary run finishes
        #[arg(long)]
        chain: bool,
    },

    /// Poll a run and stream its output until it finishes
    Watch {
        /// Run ID returned by `smith run`
        run_id: String,
    },

    /// Run history is visible in the Smith_Agentic UI (http://localhost:8765)
    Runs,

    /// List files in the outputs/ directory
    Outputs,

    /// Cancel an active run
    Cancel {
        /// Run ID to cancel
        run_id: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let base_url = std::env::var("SMITH_AGENTIC_URL")
        .unwrap_or_else(|_| "http://localhost:8765".to_string());
    let base_url = base_url.trim_end_matches('/').to_string();

    display::print_url(&base_url);

    let client = client::ApiClient::new(base_url);

    let result = match cli.command {
        Commands::Status => commands::status::run(&client).await,
        Commands::Crews  => commands::crews::run(&client).await,

        Commands::Run { crew, goal, model, chain } =>
            commands::run::run(&client, &crew, &goal, model.as_deref(), chain).await,

        Commands::Watch { run_id } =>
            commands::watch::run(&client, &run_id).await,

        Commands::Runs =>
            commands::runs::run(),

        Commands::Outputs =>
            commands::outputs::run(&client).await,

        Commands::Cancel { run_id } =>
            commands::cancel::run(&client, &run_id).await,
    };

    if let Err(e) = result {
        eprintln!("\n{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
