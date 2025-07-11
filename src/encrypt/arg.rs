#[derive(clap::Parser)]
pub struct EncryptCommand<const ENCRYPT: bool> {
    /// encrypt/decrypt source
    #[clap(flatten)]
    pub source: EncryptSource,

    /// Password
    #[arg(hide = true)]
    pub password: String,
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
