use artimonist::bitcoin;

#[derive(clap::Parser)]
pub struct EncryptCommand<const ENCRYPT: bool> {
    /// Mnemonic or private key
    #[clap(name = "MNEMONIC|PRIVATE KEY|FILE NAME")]
    pub source: EncryptSource,

    /// Password
    #[clap(hide = true, long)]
    pub password: Option<String>,
}

/// Source of encryption/decryption
#[derive(Clone, Debug)]
pub enum EncryptSource {
    /// Mnemonic or encrypted mnemonic string.
    Mnemonic(String),
    /// Private key in WIF format or encrypted key.
    Key(String),
    /// Text file containing private keys or encrypted keys.
    File(String),
}

impl std::str::FromStr for EncryptSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_private_key(s) || is_encrypted_key(s) {
            Ok(EncryptSource::Key(s.to_string()))
        } else if is_mnemonic(s) {
            Ok(EncryptSource::Mnemonic(s.to_string()))
        } else if std::path::Path::new(s).exists() {
            Ok(EncryptSource::File(s.to_string()))
        } else {
            Err(format!("Invalid input: {s}"))
        }
    }
}

/// "menmonic", "mnemonic; verify" or "mnemonic; count"
#[inline(always)]
fn is_mnemonic(s: &str) -> bool {
    let count = s.split_whitespace().count();
    matches!(count, 12 | 15 | 18 | 21 | 24 | 13 | 16 | 19 | 22 | 25)
}

#[inline(always)]
fn is_private_key(s: &str) -> bool {
    s.starts_with(['K', 'L', '5']) && s.len() == 52 && bitcoin::base58::decode(s).is_ok()
}

/// # Reference:
///   <https://github.com/bitcoin/bips/blob/master/bip-0038.mediawiki>
///   > non-EC-multiplied keys without compression (prefix 6PR)
///   > non-EC-multiplied keys with compression (prefix 6PY)
///   > EC-multiplied keys without compression (prefix 6Pf)
///   > EC-multiplied keys with compression (prefix 6Pn)
#[inline(always)]
fn is_encrypted_key(s: &str) -> bool {
    s.starts_with("6P")
        && matches!(s.as_bytes()[2], b'R' | b'Y' | b'f' | b'n')
        && s.len() == 58
        && bitcoin::base58::decode(s).is_ok()
}
