use owo_colors::OwoColorize;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use comfy_table::{Table, Cell, Color, Attribute};

pub async fn dns(domain: &str) -> anyhow::Result<()> {
    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts::default(),
    );

    let mut table = Table::new();
    table
        .set_header(vec![
            Cell::new("Type").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ]);

    macro_rules! push {
        ($label:expr, $color:expr, $result:expr) => {
            if let Ok(records) = $result {
                for r in records {
                    table.add_row(vec![
                        Cell::new($label).fg($color).add_attribute(Attribute::Bold),
                        Cell::new(r),
                    ]);
                }
            }
        };
    }

    push!("A",   Color::Green,   resolver.lookup_ip(domain).await.map(|r| r.into_iter().map(|ip| ip.to_string()).collect::<Vec<_>>()));
    push!("MX",  Color::Yellow,  resolver.mx_lookup(domain).await.map(|r| r.into_iter().map(|r| format!("{} (priority {})", r.exchange(), r.preference())).collect::<Vec<_>>()));
    push!("TXT", Color::Blue,    resolver.txt_lookup(domain).await.map(|r| r.into_iter().map(|r| r.to_string()).collect::<Vec<_>>()));
    push!("NS",  Color::Magenta, resolver.ns_lookup(domain).await.map(|r| r.into_iter().map(|r| r.to_string()).collect::<Vec<_>>()));

    println!("\n {} {}\n", "DNS records for".dimmed(), domain.cyan().bold());
    println!("{table}");
    println!();

    Ok(())
}