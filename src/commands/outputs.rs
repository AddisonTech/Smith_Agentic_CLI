use crate::client::ApiClient;
use crate::display::{format_bytes, separator};

pub async fn run(client: &ApiClient) -> Result<(), String> {
    let resp = client.list_outputs().await?;

    if resp.files.is_empty() {
        println!("  outputs/ is empty.");
        println!("  Run a crew first, then check back.");
        return Ok(());
    }

    let path_col = resp.files.iter().map(|f| f.path.len()).max().unwrap_or(4).max(4);
    let size_col = 10usize;
    let total_width = path_col + 2 + size_col;

    println!("  {:<path_col$}  {:>size_col$}", "PATH", "SIZE", path_col = path_col, size_col = size_col);
    println!("  {}", separator(total_width));

    for f in &resp.files {
        println!(
            "  {:<path_col$}  {:>size_col$}",
            f.path,
            format_bytes(f.size),
            path_col = path_col,
            size_col = size_col
        );
    }

    println!();
    let n = resp.files.len();
    println!("  {} file{}", n, if n == 1 { "" } else { "s" });

    Ok(())
}
