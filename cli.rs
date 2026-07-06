use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "threshold-lab")]
#[command(about = "Toy Shamir Secret Sharing CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Split a secret into shares
    Split {
        #[arg(long)]
        secret: i64,
        #[arg(long, default_value_t = 3)]
        threshold: usize,
        #[arg(long, default_value_t = 5)]
        shares: usize,
    },
    /// Reconstruct a secret from shares
    Reconstruct {
        #[arg(long)]
        threshold: usize,
    },
}