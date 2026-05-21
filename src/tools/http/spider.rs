use scraper::{Html, Selector};
use std::collections::HashSet;
use owo_colors::OwoColorize;

pub async fn scrape(url: &str, depth: usize, file_to_save: Option<String>) -> anyhow::Result<()> {
    let url = normalize_url(url);
    let mut visited = HashSet::new();
    let mut results = Vec::new();

    scrape_recursive(&url, depth, &mut visited, &mut results).await?;

    let mut table = comfy_table::Table::new();
    table.set_header(vec![
        comfy_table::Cell::new("URL").add_attribute(comfy_table::Attribute::Bold),
        comfy_table::Cell::new("Links Found").add_attribute(comfy_table::Attribute::Bold),
    ]);

    for (page_url, links) in &results {
        table.add_row(vec![
            page_url.cyan().to_string(),
            links.to_string().yellow().to_string(),
        ]);
    }

    println!("\n {} {}\n", "Scraped".dimmed(), url.cyan().bold());
    println!("{table}\n");

    if let Some(path) = file_to_save {
        let content = results
            .iter()
            .map(|(u, _)| u.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(&path, content)?;
        println!("{} saved to {}", "✓".green().bold(), path.cyan());
    }

    Ok(())
}

fn normalize_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    }
}

async fn scrape_recursive(
    url: &str,
    depth: usize,
    visited: &mut HashSet<String>,
    results: &mut Vec<(String, usize)>,
) -> anyhow::Result<()> {
    if depth == 0 || visited.contains(url) {
        return Ok(());
    }

    visited.insert(url.to_string());

    let response = reqwest::get(url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch {}: {}", url, e))?
        .text()
        .await?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("a[href]").unwrap();

    let links: Vec<String> = document
        .select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .filter(|href| href.starts_with("http"))
        .map(|href| href.to_string())
        .collect();

    results.push((url.to_string(), links.len()));

    for link in links {
        Box::pin(scrape_recursive(&link, depth - 1, visited, results)).await?;
    }

    Ok(())
}