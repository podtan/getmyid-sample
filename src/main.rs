//! Sample application demonstrating getmyid library usage.
//!
//! This application connects to the whoami daemon and retrieves the
//! identity of the current process.

use clap::Parser;
use getmyid::{AsyncClient, Client, GetMyIdError};
use std::path::PathBuf;
use std::time::Duration;

/// Sample application for getmyid library
#[derive(Parser, Debug)]
#[command(name = "getmyid-sample")]
#[command(about = "Retrieve process identity from whoami daemon")]
struct Args {
    /// Path to the whoami Unix socket
    #[arg(short, long, default_value = "/var/run/whoami.sock")]
    socket: PathBuf,

    /// Connection timeout in seconds
    #[arg(short, long, default_value = "5")]
    timeout: u64,

    /// Use async client instead of sync
    #[arg(long)]
    r#async: bool,

    /// Output format (text or json)
    #[arg(short, long, default_value = "text")]
    format: OutputFormat,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

fn main() {
    let args = Args::parse();

    let result = if args.r#async {
        run_async(&args)
    } else {
        run_sync(&args)
    };

    match result {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

fn run_sync(args: &Args) -> Result<(), GetMyIdError> {
    let client = Client::builder()
        .socket_path(&args.socket)
        .timeout(Duration::from_secs(args.timeout))
        .build();

    let identity = client.get_identity()?;

    match args.format {
        OutputFormat::Text => {
            println!("Identity retrieved successfully!");
            println!();
            println!("  Identity:   {}", identity.identity);
            println!("  IDM URL:    {}", identity.idm_url);
            println!("  Config URL: {}", identity.config_url);
            println!("  Token:      {}", identity.token);
            println!("  Process:    {}", identity.process);
            println!("  PID:        {}", identity.pid);
            println!("  UID:        {}", identity.uid);
            println!("  GID:        {}", identity.gid);
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&identity).expect("serialization failed")
            );
        }
    }

    Ok(())
}

#[tokio::main]
async fn run_async(args: &Args) -> Result<(), GetMyIdError> {
    let client = AsyncClient::builder()
        .socket_path(&args.socket)
        .timeout(Duration::from_secs(args.timeout))
        .build();

    let identity = client.get_identity().await?;

    match args.format {
        OutputFormat::Text => {
            println!("Identity retrieved successfully (async)!");
            println!();
            println!("  Identity:   {}", identity.identity);
            println!("  IDM URL:    {}", identity.idm_url);
            println!("  Config URL: {}", identity.config_url);
            println!("  Token:      {}", identity.token);
            println!("  Process:    {}", identity.process);
            println!("  PID:        {}", identity.pid);
            println!("  UID:        {}", identity.uid);
            println!("  GID:        {}", identity.gid);
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&identity).expect("serialization failed")
            );
        }
    }

    Ok(())
}
