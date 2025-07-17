use anyhow::anyhow;
use bip38::{Decrypt, EncryptWif};

#[inline(always)]
pub fn bip38_encrypt(wif: &str, password: &str) -> anyhow::Result<String> {
    match artimonist::NETWORK.is_mainnet() {
        true => wif.encrypt_wif(password).map_err(|e| anyhow!(e)),
        false => Ok(wif.to_string()), // No encryption on testnet
    }
}

#[inline(always)]
pub fn bip38_decrypt(wif: &str, password: &str) -> anyhow::Result<String> {
    match artimonist::NETWORK.is_mainnet() {
        true => wif.decrypt_to_wif(password).map_err(|e| anyhow!(e)),
        false => Ok(wif.to_string()), // No decryption on testnet
    }
}
