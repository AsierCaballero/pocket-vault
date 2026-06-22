mod cli;
mod crypto;
mod store;

use clap::Parser;
use cli::{Cli, Command};
use crypto::Crypto;
use store::Store;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let vault_dir = dirs::data_dir()
        .map(|d| d.join("pocket-vault"))
        .unwrap_or_else(|| std::path::PathBuf::from("./vault"));

    let mut store = Store::new(&vault_dir)?;

    match cli.command {
        Command::Init => {
            let (recipient, identity) = Crypto::generate_keypair()?;
            store.store_keypair("default", &identity, &recipient)?;
            println!("🔐 Vault initialized at {}", vault_dir.display());
            println!("   Recipient: {}", &recipient[..20]);
        }
        Command::Set { key, value } => {
            let identity = store.get_identity("default")
                .ok_or_else(|| anyhow::anyhow!("vault not initialized. Run init first"))?;
            let encrypted = Crypto::encrypt(value.as_bytes(), identity)?;
            store.set(key, encrypted)?;
            println!("✓ Secret stored");
        }
        Command::Get { key } => {
            let identity = store.get_identity("default")
                .ok_or_else(|| anyhow::anyhow!("vault not initialized"))?;
            let encrypted = store.get(&key)
                .ok_or_else(|| anyhow::anyhow!("secret not found: {}", key))?;
            let decrypted = Crypto::decrypt(encrypted, identity)?;
            println!("{}", String::from_utf8(decrypted)?);
        }
        Command::List => {
            let secrets = store.list();
            if secrets.is_empty() {
                println!("No secrets stored");
            } else {
                for s in &secrets {
                    println!("  {}", s);
                }
            }
        }
        Command::Delete { key } => {
            store.delete(&key)?;
            println!("✓ Secret deleted: {}", key);
        }
    }
    Ok(())
}
