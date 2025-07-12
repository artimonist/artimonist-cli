use artimonist::bitcoin;

#[derive(clap::Parser)]
pub struct EncryptCommand<const ENCRYPT: bool> {
    /// encrypt/decrypt source
    #[clap(flatten)]
    pub source: EncryptSource,

    /// Password
    #[clap(hide = true, long)]
    pub password: Option<String>,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct EncryptSource {
    /// Private key (Wif) or encrypted key
    #[clap(value_parser = parse_key)]
    pub key: Option<String>,

    /// Text filename containing private keys or encrypted keys
    #[clap(short, long, value_parser = parse_file)]
    pub file: Option<String>,
}

fn parse_key(s: &str) -> Result<String, String> {
    if is_private_key(s) || is_encrypted_key(s) {
        Ok(s.to_string())
    } else {
        Err(format!("Invalid key format: {s}"))
    }
}

fn parse_file(s: &str) -> Result<String, String> {
    if std::path::Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err(format!("File does not exist: {s}"))
    }
}

#[inline(always)]
fn is_private_key(s: &str) -> bool {
    s.starts_with(['K', 'L', '5']) && s.len() == 52 && bitcoin::base58::decode(s).is_ok()
}

#[inline(always)]
fn is_encrypted_key(s: &str) -> bool {
    s.starts_with("6P") && s.len() == 58 && bitcoin::base58::decode(s).is_ok()
}
