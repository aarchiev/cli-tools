use clap::Parser;
use std::net::TcpStream;
use std::time::Duration;
use tokio::task;

#[derive(Parser)]
#[command(name = "nmap-rust")]
#[command(about = "A simple port scanner in Rust")]
struct Args {
    #[arg(short, long)]
    target: String,

    #[arg(short, long, default_value = "1")]
    start_port: u16,

    #[arg(short, long, default_value = "1024")]
    end_port: u16,

    #[arg(short, long, default_value = "1")]
    timeout: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("Scanning {} from port {} to {}...", args.target, args.start_port, args.end_port);

    let mut handles = vec![];

    for port in args.start_port..=args.end_port {
        let target = args.target.clone();
        let timeout = Duration::from_secs(args.timeout);
        let handle = task::spawn(async move {
            if is_port_open(&target, port, timeout).await {
                println!("Port {} is open", port);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Scan complete.");
}

async fn is_port_open(target: &str, port: u16, timeout: Duration) -> bool {
    let addr = format!("{}:{}", target, port);
    match tokio::time::timeout(timeout, tokio::net::TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}
