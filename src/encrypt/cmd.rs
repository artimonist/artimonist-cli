#[derive(clap::Parser)]
pub struct EncryptCommand {
    /// encrypt/decrypt source
    #[clap(flatten)]
    pub source: EncryptSource,

    /// Password
    #[cfg(not(feature = "automatic"))]
    #[arg(skip)]
    pub password: String,
    #[cfg(feature = "automatic")]
    #[arg(short, long, default_value = "123456")]
    pub password: String,

    // encrypt or decrypt
    #[arg(skip)]
    pub is_encrypt: bool,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct EncryptSource {
    /// Private key (Wif)
    pub key: Option<String>,

    /// Encrypt/Decrypt file
    #[arg(short, long)]
    pub file: Option<String>,
}
