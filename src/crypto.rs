use age::secrecy::ExposeSecret;
use age::x25519::{Identity, Recipient};
use age::{Decryptor, Encryptor};
use anyhow::Result;
use std::io::{Read, Write};

pub struct Crypto;

impl Crypto {
    pub fn generate_keypair() -> Result<(String, String)> {
        let key = Identity::generate();
        let recipient: Recipient = key.to_public();
        let identity = key.to_string().expose_secret().to_string();
        Ok((recipient.to_string(), identity))
    }

    pub fn encrypt(plaintext: &[u8], recipient_str: &str) -> Result<Vec<u8>> {
        let recipient: Recipient = recipient_str
            .parse()
            .map_err(|e: &'static str| anyhow::anyhow!(e))?;
        let encryptor = Encryptor::with_recipients(vec![Box::new(recipient)])
            .ok_or_else(|| anyhow::anyhow!("no valid recipients"))?;
        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(plaintext)?;
        writer.finish()?;
        Ok(encrypted)
    }

    pub fn decrypt(ciphertext: &[u8], identity_str: &str) -> Result<Vec<u8>> {
        let identity: Identity = identity_str
            .parse()
            .map_err(|e: &'static str| anyhow::anyhow!(e))?;
        let decryptor = Decryptor::new(ciphertext)?;
        let mut decrypted = vec![];
        match decryptor {
            Decryptor::Recipients(d) => {
                let mut reader = d.decrypt(std::iter::once(&identity as &dyn age::Identity))?;
                reader.read_to_end(&mut decrypted)?;
            }
            Decryptor::Passphrase(_) => {
                return Err(anyhow::anyhow!(
                    "file was encrypted with a passphrase, not a recipient"
                ));
            }
        }
        Ok(decrypted)
    }
}
