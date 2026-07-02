use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct VaultFile {
    keys: HashMap<String, String>,
    encrypted: HashMap<String, Vec<u8>>,
}

pub struct Store {
    path: PathBuf,
    vault: VaultFile,
}

impl Store {
    pub fn new(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path)?;
        let vault_path = path.join("vault.json");
        let vault = if vault_path.exists() {
            serde_json::from_slice(&std::fs::read(&vault_path)?)?
        } else { VaultFile { keys: HashMap::new(), encrypted: HashMap::new() } };
        Ok(Self { path: path.to_path_buf(), vault })
    }

    pub fn initialized(&self) -> bool { self.vault.keys.contains_key("default_identity") }
    pub fn get_identity(&self, name: &str) -> Option<&str> { self.vault.keys.get(&format!("{}_identity", name)).map(|s| s.as_str()) }
    pub fn get_recipient(&self, name: &str) -> Option<&str> { self.vault.keys.get(&format!("{}_recipient", name)).map(|s| s.as_str()) }

    pub fn store_keypair(&mut self, name: &str, identity: &str, recipient: &str) -> Result<()> {
        self.vault.keys.insert(format!("{}_identity", name), identity.into());
        self.vault.keys.insert(format!("{}_recipient", name), recipient.into());
        self.save()
    }

    pub fn set(&mut self, key: String, encrypted: Vec<u8>) -> Result<()> { self.vault.encrypted.insert(key, encrypted); self.save() }
    pub fn get(&self, key: &str) -> Option<&[u8]> { self.vault.encrypted.get(key).map(|v| v.as_slice()) }
    pub fn list(&self) -> Vec<String> { self.vault.encrypted.keys().cloned().collect() }
    pub fn delete(&mut self, key: &str) -> Result<()> { self.vault.encrypted.remove(key); self.save() }

    pub fn export_to(&self, path: &Path) -> Result<()> {
        let data = serde_json::to_vec_pretty(&self.vault)?;
        Ok(std::fs::write(path, data)?)
    }

    pub fn import_from(&mut self, path: &Path) -> Result<()> {
        let data = std::fs::read(path)?;
        self.vault = serde_json::from_slice(&data)?;
        self.save()
    }

    fn save(&self) -> Result<()> {
        Ok(std::fs::write(self.path.join("vault.json"), serde_json::to_vec_pretty(&self.vault)?)?)
    }
}
