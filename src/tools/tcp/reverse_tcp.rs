use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn reverse_tcp(lport: u16) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", lport)).await?;
    let (mut socket, addr) = listener.accept().await?;
    println!("Accepted connection from {}", addr);

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        socket.write_all(input.as_bytes()).await.unwrap();

        let mut buf = vec![0u8; 65536];
        let n = socket.read(&mut buf).await.unwrap();
        print!("{}", String::from_utf8_lossy(&buf[..n]));
    }
}