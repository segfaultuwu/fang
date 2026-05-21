use indicatif::ProgressBar;

pub async fn ping(host: &str, port: u16, retries: u32) -> anyhow::Result<()> {
    let addr = format!("{}:{}", host, port);

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    for attempt in 1..=retries {
        spinner.set_message(format!("Attempt {}/{} → {}", attempt, retries, addr));

        match tokio::net::TcpStream::connect(&addr).await {
            Ok(stream) => {
                spinner.finish_and_clear();
                println!("Connected to {}", stream.peer_addr()?);
                return Ok(());
            }
            Err(e) => {
                if attempt == retries {
                    spinner.finish_and_clear();
                    anyhow::bail!("Failed after {} attempts: {}", retries, e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }

    unreachable!()
}