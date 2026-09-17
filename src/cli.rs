use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Default PID file location — `/tmp/anthropic-proxy.pid` on Unix,
/// the OS temp directory on Windows. Resolved at runtime because clap's
/// `default_value_t` requires `Display`, which `PathBuf` does not implement.
#[cfg(unix)]
pub fn default_pid_file() -> PathBuf {
    PathBuf::from("/tmp/anthropic-proxy.pid")
}

#[cfg(windows)]
pub fn default_pid_file() -> PathBuf {
    std::env::temp_dir().join("anthropic-proxy.pid")
}

#[derive(Parser, Debug)]
#[command(
    name = "anthropic-proxy",
    version,
    about = "Proxy Anthropic API requests to OpenAI-compatible endpoints",
    long_about = "A high-performance proxy that translates Anthropic Claude API requests \
                  to OpenAI-compatible endpoints like OpenRouter, allowing you to use \
                  Claude-compatible clients with any OpenAI-compatible API."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to custom .env configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Enable debug logging (same as DEBUG=true)
    #[arg(short, long)]
    pub debug: bool,

    /// Enable verbose logging (logs full request/response bodies)
    #[arg(short, long)]
    pub verbose: bool,

    /// Port to listen on (overrides PORT env var)
    #[arg(short, long, value_name = "PORT")]
    pub port: Option<u16>,

    /// Address to bind the listener to (overrides ANTHROPIC_PROXY_BIND env var, default 0.0.0.0)
    #[arg(long, value_name = "ADDR")]
    pub bind: Option<String>,

    /// System prompt terms to remove before forwarding upstream (semicolon-separated or repeated)
    #[arg(long, value_name = "TEXT", value_delimiter = ';')]
    pub system_prompt_ignore: Vec<String>,

    /// Run as background daemon (Unix only; on Windows this flag exits with an error)
    #[arg(long)]
    pub daemon: bool,

    /// PID file path (used with daemon commands). Defaults to the OS temp directory if omitted.
    #[arg(long, value_name = "FILE")]
    pub pid_file: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Stop running daemon
    Stop {
        /// PID file path. Defaults to the OS temp directory if omitted.
        #[arg(long, value_name = "FILE")]
        pid_file: Option<PathBuf>,
    },
    /// Check daemon status
    Status {
        /// PID file path. Defaults to the OS temp directory if omitted.
        #[arg(long, value_name = "FILE")]
        pid_file: Option<PathBuf>,
    },
}
