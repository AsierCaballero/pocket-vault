mod cli;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        cli::Command::Init => println!("initializing vault"),
        cli::Command::Set { key, value } => println!("setting {}={}", key, value),
        cli::Command::Get { key } => println!("getting {}", key),
        cli::Command::List => println!("listing secrets"),
        cli::Command::Delete { key } => println!("deleting {}", key),
    }
    Ok(())
}
