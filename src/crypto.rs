use anyhow::Result;
use age::{Decryptor, Encryptor};
use age::x25519::{Identity, Recipient};
use rand::rngs::OsRng;
use zeroize::Zeroize;

pub struct Crypto;

impl Crypto {
    pub fn generate_keypair() -> Result<(String, String)> {
        let key = Identity::generate(OsRng);
        let recipient: Recipient = key.to_public().into();
        Ok((
            recipient.to_string(),
            key.to_string(),
        ))
    }

    pub fn encrypt(plaintext: &[u8], recipient_str: &str) -> Result<Vec<u8>> {
        let recipient: Recipient = recipient_str.parse()?;
        let encryptor = Encryptor::with_recipients(vec![Box::new(recipient)]);
        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted, &mut OsRng)?;
        writer.write_all(plaintext)?;
        writer.finish()?;
        Ok(encrypted)
    }

    pub fn decrypt(ciphertext: &[u8], identity_str: &str) -> Result<Vec<u8>> {
        let identity: Identity = identity_str.parse()?;
        let decryptor = Decryptor::new(ciphertext)?;
        let mut decrypted = vec![];
        let mut reader = decryptor.decrypt(&std::iter::once(Box::new(identity) as Box<dyn age::Identity>))?;
        reader.read_to_end(&mut decrypted)?;
        Ok(decrypted)
    }
}
