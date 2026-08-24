//! Provider API keys in the OS credential store (PRD §22) — Keychain on macOS,
//! Credential Manager on Windows, Secret Service on Linux. A key never sits in
//! application state, a config file, or a log; it's read from here only at the moment
//! a request is signed, by the adapter that needs it — never handed to the renderer.

use keyring::Entry;

const SERVICE: &str = "dev.anycode.desktop";

#[derive(Debug, thiserror::Error)]
pub enum SecretError {
    #[error(transparent)]
    Keyring(#[from] keyring::Error),
}

fn entry(provider_id: &str) -> Result<Entry, SecretError> {
    Entry::new(SERVICE, provider_id).map_err(Into::into)
}

pub fn set_api_key(provider_id: &str, key: &str) -> Result<(), SecretError> {
    entry(provider_id)?.set_password(key)?;
    Ok(())
}

pub fn get_api_key(provider_id: &str) -> Result<Option<String>, SecretError> {
    match entry(provider_id)?.get_password() {
        Ok(key) => Ok(Some(key)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn delete_api_key(provider_id: &str) -> Result<(), SecretError> {
    match entry(provider_id)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
