use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pocket-vault", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Set { key: String, value: String },
    Get { key: String },
    List,
    Delete { key: String },
}
