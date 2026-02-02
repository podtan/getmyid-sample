//! Sample application demonstrating getmyid library usage.
//!
//! This application connects to the whoami daemon and retrieves the
//! identity of the current process.

use clap::Parser;
use getmyid::{AsyncClient, Client, GetMyIdError, RunnerRequest};
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

    /// Instance ID to send in runner context (for dynamic config routing)
    #[arg(short, long)]
    instance_id: Option<u64>,

    /// Include current timestamp in runner context
    #[arg(long)]
    with_timestamp: bool,
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

fn build_runner_request(args: &Args) -> Option<RunnerRequest> {
    if args.instance_id.is_none() && !args.with_timestamp {
        return None;
    }

    let mut req = RunnerRequest::new();

    if let Some(id) = args.instance_id {
        req = req.with_instance_id(id);
    }

    if args.with_timestamp {
        req = req.with_current_timestamp();
    }

    Some(req)
}

fn run_sync(args: &Args) -> Result<(), GetMyIdError> {
    let client = Client::builder()
        .socket_path(&args.socket)
        .timeout(Duration::from_secs(args.timeout))
        .build();

    let runner_req = build_runner_request(args);
    let identity = client.get_identity_with_runner(runner_req)?;

    match args.format {
        OutputFormat::Text => {
            println!("Identity retrieved successfully!");
            println!();
            println!("  Identity:   {}", identity.identity);
            println!("  IDM URL:    {}", identity.idm_url);
            println!("  Config URL: {}", identity.config_url);
            println!("  Token:      {}", identity.token);
            println!();
            println!("  Runner:");
            println!("    Hostname:    {}", identity.runner.hostname);
            println!("    Process:     {}", identity.runner.process);
            println!("    PID:         {}", identity.runner.pid);
            println!("    UID:         {}", identity.runner.uid);
            println!("    GID:         {}", identity.runner.gid);
            if let Some(instance_id) = identity.runner.instance_id {
                println!("    Instance ID: {}", instance_id);
            }
            if let Some(timestamp) = identity.runner.timestamp {
                println!("    Timestamp:   {}", timestamp);
            }
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

    let runner_req = build_runner_request(args);
    let identity = client.get_identity_with_runner(runner_req).await?;

    match args.format {
        OutputFormat::Text => {
            println!("Identity retrieved successfully (async)!");
            println!();
            println!("  Identity:   {}", identity.identity);
            println!("  IDM URL:    {}", identity.idm_url);
            println!("  Config URL: {}", identity.config_url);
            println!("  Token:      {}", identity.token);
            println!();
            println!("  Runner:");
            println!("    Hostname:    {}", identity.runner.hostname);
            println!("    Process:     {}", identity.runner.process);
            println!("    PID:         {}", identity.runner.pid);
            println!("    UID:         {}", identity.runner.uid);
            println!("    GID:         {}", identity.runner.gid);
            if let Some(instance_id) = identity.runner.instance_id {
                println!("    Instance ID: {}", instance_id);
            }
            if let Some(timestamp) = identity.runner.timestamp {
                println!("    Timestamp:   {}", timestamp);
            }
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
