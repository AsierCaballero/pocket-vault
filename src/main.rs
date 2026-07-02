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
        Command::Init { force } => {
            if !force && store.initialized() {
                println!("Vault already exists. Use --force to reinitialize.");
                return Ok(());
            }
            let (recipient, identity) = Crypto::generate_keypair()?;
            store.store_keypair("default", &identity, &recipient)?;
            println!("Vault initialized at {}", vault_dir.display());
        }
        Command::Set { key, value } => {
            let recipient = store.get_recipient("default")
                .ok_or_else(|| anyhow::anyhow!("run init first"))?;
            let encrypted = Crypto::encrypt(value.as_bytes(), recipient)?;
            store.set(key, encrypted)?;
            println!("Stored");
        }
        Command::Get { key } => {
            let identity = store.get_identity("default")
                .ok_or_else(|| anyhow::anyhow!("run init first"))?;
            let encrypted = store.get(&key)
                .ok_or_else(|| anyhow::anyhow!("not found: {}", key))?;
            let decrypted = Crypto::decrypt(encrypted, identity)?;
            println!("{}", String::from_utf8(decrypted)?);
        }
        Command::List => {
            for s in store.list() {
                println!("  {}", s);
            }
        }
        Command::Delete { key } => {
            store.delete(&key)?;
            println!("Deleted: {}", key);
        }
        Command::Rotate => {
            let identity = store.get_identity("default")
                .ok_or_else(|| anyhow::anyhow!("run init first"))?;
            let (new_rec, new_id) = Crypto::generate_keypair()?;
            let secrets = store.list();
            for key in &secrets {
                let encrypted = store.get(key).unwrap();
                let decrypted = Crypto::decrypt(encrypted, identity)?;
                let re_encrypted = Crypto::encrypt(&decrypted, &new_rec)?;
                store.set(key.clone(), re_encrypted)?;
            }
            store.store_keypair("default", &new_id, &new_rec)?;
            println!("Rotated keys for {} secrets", secrets.len());
        }
        Command::Backup { path } => {
            store.export_to(&path)?;
            println!("Backup saved to {}", path.display());
        }
        Command::Restore { path } => {
            store.import_from(&path)?;
            println!("Restored from {}", path.display());
        }
    }
    Ok(())
}
