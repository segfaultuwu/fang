static PORTS_QUICK: &[u16] = &[21, 22, 23, 25, 53, 80, 443, 3306, 3389, 5900, 8080];
static PORTS_WEB: &[u16] = &[80, 443, 8080, 8443, 8888, 8008, 8800];
static PORTS_DB: &[u16] = &[3306, 5432, 1433, 1521, 27017, 6379, 5984, 9200];

fn ports_full() -> Vec<u16> {
    PORTS_DB.iter()
        .chain(PORTS_WEB.iter())
        .chain(PORTS_QUICK.iter())
        .copied()
        .collect()
}

pub async fn scan(host: &str, scan_type: &str) -> anyhow::Result<()> {
    let ports: Vec<u16> = match scan_type {
        "quick" => PORTS_QUICK.to_vec(),
        "web" => PORTS_WEB.to_vec(),
        "db" => PORTS_DB.to_vec(),
        "full" => ports_full(),
        _ => return Err(anyhow::anyhow!("Invalid scan type: {scan_type}")),
    };

    println!("Scanning {} with profile '{}'", host, scan_type);

    for port in ports {
        let addr = format!("{}:{}", host, port);
        match tokio::time::timeout(
            tokio::time::Duration::from_millis(700),
            tokio::net::TcpStream::connect(&addr),
        )
        .await
        {
            Ok(Ok(_)) => println!("  {:>5} open", port),
            _ => println!("  {:>5} closed", port),
        }
    }
    Ok(())
}