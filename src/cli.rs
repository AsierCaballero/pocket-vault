use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pocket-vault", version, about = "Offline secret manager with age encryption")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init { #[arg(long)] force: bool },
    Set { key: String, value: String },
    Get { key: String },
    List,
    Delete { key: String },
    Rotate,
    Backup { path: PathBuf },
    Restore { path: PathBuf },
}
