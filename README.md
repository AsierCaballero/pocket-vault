# pocket-vault

Offline secret manager with age encryption. X25519 keypairs, local JSON storage.

```
pocket-vault init
pocket-vault set db_password "s3cret!"
pocket-vault get db_password
pocket-vault rotate               # Re-encrypt all secrets
pocket-vault backup ~/vault.bak   # Export vault
```

## Commands
- `init` — Generate X25519 keypair, create vault
- `set <key> <value>` — Encrypt and store
- `get <key>` — Decrypt and print
- `list` — List stored keys
- `delete <key>` — Remove a secret
- `rotate` — Re-encrypt all secrets with new keys
- `backup <path>` — Export vault to file
- `restore <path>` — Import vault from file
