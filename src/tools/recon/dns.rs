use owo_colors::OwoColorize;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;

pub async fn dns(domain: &str) -> anyhow::Result<()> {
    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts::default(),
    );

    println!("\n {} {}\n", "DNS records for".dimmed(), domain.cyan().bold());

    macro_rules! lookup {
        ($label:expr, $color:ident, $result:expr) => {
            match $result {
                Ok(records) => {
                    for r in records {
                        println!("  {}  {}", $label.$color().bold(), r);
                    }
                }
                Err(_) => {} // silently skip if record type doesn't exist
            }
        };
    }

    lookup!(" A  ", green,   resolver.lookup_ip(domain).await.map(|r| r.into_iter().map(|ip| ip.to_string()).collect::<Vec<_>>()));
    lookup!(" MX ", yellow,  resolver.mx_lookup(domain).await.map(|r| r.into_iter().map(|r| format!("{} ({})", r.exchange(), r.preference())).collect::<Vec<_>>()));
    lookup!(" TXT", blue,    resolver.txt_lookup(domain).await.map(|r| r.into_iter().map(|r| r.to_string()).collect::<Vec<_>>()));
    lookup!(" NS ", magenta, resolver.ns_lookup(domain).await.map(|r| r.into_iter().map(|r| r.to_string()).collect::<Vec<_>>()));

    println!();
    Ok(())
}