use crate::client::ApiClient;
use crate::display::separator;

pub async fn run(client: &ApiClient) -> Result<(), String> {
    let defaults = client.crew_defaults().await?;

    if defaults.is_empty() {
        println!("  No crews returned.");
        return Ok(());
    }

    // Sort: "default" first, then alphabetical
    let mut crews: Vec<(&String, &String)> = defaults.iter().collect();
    crews.sort_by(|(a, _), (b, _)| {
        if a.as_str() == "default" { return std::cmp::Ordering::Less; }
        if b.as_str() == "default" { return std::cmp::Ordering::Greater; }
        a.cmp(b)
    });

    let col = crews.iter().map(|(k, _)| k.len()).max().unwrap_or(4).max(4);
    let header_width = col + 2 + 28;

    println!("  {:<col$}  {}", "CREW", "MODEL", col = col + 2);
    println!("  {}", separator(header_width));

    for (crew, model) in &crews {
        println!("  {:<col$}  {}", crew, model, col = col + 2);
    }

    Ok(())
}
